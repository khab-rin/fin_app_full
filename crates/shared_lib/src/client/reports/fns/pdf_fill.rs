use std::collections::HashMap;
use serde_json::Value;
use lopdf::{Document, Object, Dictionary, ObjectId};

use crate::{ProcessError, Status};
use crate::client::reports::fns::usn_decl_notif_elems::Elems;


pub fn custom_pdf_fill(
	elems: &Elems,
	pdf_tpl_bytes: &[u8]
) -> Result<Vec<u8>, Status> {

	let mut doc = lopdf::Document::load_mem(pdf_tpl_bytes)
		.map_err(|err| err.process_err(Status::PdfFileLogicError, ""))?;

	let mut pdf_key_values_map: HashMap<String, Vec<ObjectId>> = HashMap::new();
	
	let mut acro_form_id: Option<ObjectId> = None;

	{
		let catalog = doc.catalog()
			.map_err(|err| err.process_err(Status::PdfFileLogicError, ""))?;

		if let Ok(Object::Reference(id)) = catalog.get(b"AcroForm") {
			acro_form_id = Some(*id);
		}

		let acro_form_obj = catalog.get_deref(b"AcroForm", &doc)
			.map_err(|err| err.process_err(Status::PdfFileLogicError, "Ключ AcroForm не найден или поврежден"))?;

		let acro_form: &Dictionary = acro_form_obj.as_dict()
			.map_err(|err| err.process_err(Status::PdfFileLogicError, "AcroForm не является словарем"))?;

		collect_pdf_pattern_keys_ids(acro_form, &doc, &mut pdf_key_values_map)
			.map_err(|err| err.process_err(Status::PdfFileLogicError, ""))?;
	}

	let json_value = serde_json::to_value(elems)
		.map_err(|err| err.process_err(Status::MappingError, ""))?;

	let mut date_map: HashMap<String, String> = HashMap::new();


	if let Value::Object(map) = json_value {
		for (key, val) in map {
			if let Value::String(v) = val {
				let processed_value = if v.is_empty() { "-".to_string() } else { v };
				date_map.insert(key, processed_value);
			}
		}
	}

	
	for (field_name, object_ids) in pdf_key_values_map {
		if let Some(val) = date_map.get(&field_name) {
			for &(k1, k2) in object_ids.iter() {
				let mut field_obj = doc.get_object((k1, k2))
					.map_err(|err| err.process_err(Status::PdfFileLogicError, "Ошибка получения объекта"))?
					.clone();

				let field_dict = field_obj.as_dict_mut()
					.map_err(|err| err.process_err(Status::PdfFileLogicError, "Объект поля не является словарем"))?;

				let encoded_bytes = to_pdf_utf16_bom(val);
				let pdf_value_obj = lopdf::Object::String(encoded_bytes, lopdf::StringFormat::Hexadecimal);

				field_dict.set("V", pdf_value_obj);

				doc.set_object((k1, k2), field_obj);
			}
		}
	}

	if let Some(id) = acro_form_id {

		let acro_obj_ref = doc.get_object(id)
			.map_err(|err| err.process_err(Status::PdfFileLogicError, ""))?;

		let mut acro_obj = acro_obj_ref.clone();

		let acro_dict = acro_obj.as_dict_mut()
			.map_err(|err| err.process_err(Status::PdfFileLogicError, ""))?;

		acro_dict.set("NeedAppearances", Object::Boolean(true));
		doc.set_object(id, acro_obj);
	}

	let mut output_bytes = Vec::new();
	doc.save_to(&mut output_bytes)
		.map_err(|err| err.process_err(Status::PdfFileLogicError, "Не удалось сохранить PDF-документ"))?;

	Ok(output_bytes)
}


fn collect_pdf_pattern_keys_ids(
    acro_form: &Dictionary,
    doc: &Document,
    result: &mut HashMap<String, Vec<ObjectId>>
) -> Result<(), Status> {

    let fields_obj = acro_form.get_deref(b"Fields", doc)
        .map_err(|err| err.process_err(Status::PdfFileLogicError, "Ключ Fields не найден в форме"))?;

    let fields_arr = fields_obj.as_array()
        .map_err(|err| err.process_err(Status::PdfFileLogicError, "Fields не является массивом"))?;

    for obj in fields_arr {
        if let Object::Reference(field_id) = obj {
            recursive_collect_fields(*field_id, None, doc, result)?;
        }
    }

    Ok(())
}

fn recursive_collect_fields(
    field_id: ObjectId,
    parent_name: Option<String>,
    doc: &Document,
    result: &mut HashMap<String, Vec<ObjectId>>
) -> Result<(), Status> {
	
    if let Ok(field_obj_ref) = doc.get_object(field_id) {
        if let Ok(field_dict) = field_obj_ref.as_dict() {

            let mut current_full_name = parent_name.clone();
            if let Ok(t_obj) = field_dict.get_deref(b"T", doc) {
                if let Ok(pdf_string) = t_obj.as_str() {
                    let local_name = String::from_utf8_lossy(pdf_string).into_owned();
                    current_full_name = match current_full_name {
                        Some(p_name) => Some(format!("{}.{}", p_name, local_name)),
                        None => Some(local_name),
                    };
                }
            }

            if let Ok(kids_obj) = field_dict.get_deref(b"Kids", doc) {
                if let Ok(kids_array) = kids_obj.as_array() {

                    let mut has_named_children = false;
                    for kid_ref in kids_array {
                        if let Object::Reference(kid_id) = kid_ref {
                            if let Ok(k_obj) = doc.get_object(*kid_id) {
                                if let Ok(k_dict) = k_obj.as_dict() {
                                    if k_dict.get(b"T").is_ok() {
                                        has_named_children = true;
                                        break;
                                    }
                                }
                            }
                        }
                    }

                    if has_named_children {

                        for kid_ref in kids_array {
                            if let Object::Reference(kid_id) = kid_ref {
                                recursive_collect_fields(*kid_id, current_full_name.clone(), doc, result)?;
                            }
                        }
                    } else {
                        if let Some(final_name) = current_full_name {
                            let mut ids = vec![field_id];
                            for kid_ref in kids_array {
                                if let Object::Reference(kid_id) = kid_ref {
                                    ids.push(*kid_id);
                                }
                            }
                            result.entry(final_name).or_default().extend(ids);
                        }
                    }
                    return Ok(());
                }
            }

            // 3. Если дочерних элементов нет (конечный плоский узел дерева)
            if let Some(final_name) = current_full_name {
                result.entry(final_name).or_default().push(field_id);
            }
        }
    }
    Ok(())
}



fn to_pdf_utf16_bom(
	text: &str
) -> Vec<u8> {
	let mut bytes = vec!(0xFE, 0xFF);

	for mut ch in text.encode_utf16() {
		bytes.push((ch >> 8) as u8);
		bytes.push((ch & 0xFF) as u8);
	}
	bytes
}