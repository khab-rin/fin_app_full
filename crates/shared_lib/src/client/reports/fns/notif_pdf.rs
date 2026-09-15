use crate::ProcessError;
use crate::Status;
use crate::service::auth_service::general::ActiveSession;
use crate::primitives::frozen::text::MidName;
use crate::service::reports::fns_xsd_shemas::usn_1110355_notif::UsnNotifFile;

use lopdf::{Document, Object, Dictionary, StringFormat};
use std::collections::HashMap;

/// Основная функция для генерации PDF-уведомления
pub fn make_notif_pdf(
    session: &ActiveSession,
    notif: &UsnNotifFile
) -> Result<Vec<u8>, Status> {

    let inn = session.session_user.company.comp_inn.to_string();
    let kpp = session.session_user.company.kpp.to_string();
    let fio = session.session_user.person.metadata.fio.clone();
    let sur_name = fio.sur_name.to_string();
    let first_name = fio.first_name.to_string();
    let mid_name = fio.mid_name.unwrap_or(MidName::unchecked("")).to_string();

    let fns_branch = notif.document.branch_code.to_string();
    let oktmo = notif.document.notifications[0].oktmo.to_string();
    let kbk = notif.document.notifications[0].kbk.to_string();
    let avans_amnt = notif.document.notifications[0].avans_amnt.to_string();
    let period_code = notif.document.notifications[0].period.to_string();
    let year = notif.document.notifications[0].year.to_string();

    // Загрузка шаблона PDF из ресурсов
    let pdf_tpl_bytes = include_bytes!("../../../../../../resourses/1110355.pdf"); 
    let mut doc = lopdf::Document::load_mem(pdf_tpl_bytes)
        .map_err(|err| err.process_err(Status::Tech, "Не удалось загрузить PDF шаблон"))?;

    // Сборка HashMap со всеми заполняемыми полями
    let mut fields_to_fill: HashMap<String, String> = HashMap::new();

    // 1. ИНН и КПП организации (сквозные поля на 1 и 2 страницах)
    fields_to_fill.insert("Text1".to_string(), inn);
    fields_to_fill.insert("Text2".to_string(), kpp.clone());

    // 2. Метаданные титульного листа
    fields_to_fill.insert("Text3".to_string(), fns_branch);
    fields_to_fill.insert("Text4".to_string(), "002".to_string()); // Количество страниц документа
    fields_to_fill.insert("Text5".to_string(), "000".to_string()); // Количество страниц копий приложений
    fields_to_fill.insert("Text6".to_string(), "1".to_string());    // Признак налогоплательщика (1 - сам, 2 - представитель)

    // 3. ФИО налогоплательщика (строго с большой буквы T, как определил парсер)
    fields_to_fill.insert("Text8.0".to_string(), sur_name);
    fields_to_fill.insert("Text8.1".to_string(), first_name);
    fields_to_fill.insert("Text8.2".to_string(), mid_name);

    // Идентификатор второй страницы
    fields_to_fill.insert("Text7".to_string(), "2".to_string());

    // Дата подписания (15 сентября 2026 года в качестве примера)
    fields_to_fill.insert("Text9.0".to_string(), "15".to_string());
    fields_to_fill.insert("Text9.1".to_string(), "09".to_string());
    fields_to_fill.insert("Text10".to_string(), "2026".to_string());

    // 4. Разделение суммы авансового платежа на рубли и копейки
    let (rub, kop) = match avans_amnt.split_once('.') {
        Some((r, k)) => (r.to_string(), format!("{:0<2}", &k[..std::cmp::min(k.len(), 2)])),
        None => (avans_amnt.clone(), "00".to_string()),
    };

    // 5. Блок данных на второй странице (Индексы .0.0 соответствуют первому блоку на листе)
    fields_to_fill.insert("Text12.0".to_string(), kpp);
    fields_to_fill.insert("Text13.0.0".to_string(), oktmo);
    fields_to_fill.insert("Text14.0.0".to_string(), kbk);
    fields_to_fill.insert("Text15.0.0".to_string(), rub);
    fields_to_fill.insert("Text16.0.0.0".to_string(), kop);
    
    // Периодичность платежа
    fields_to_fill.insert("Text16.0.1.0".to_string(), period_code); // Код периода (2 знака)
    fields_to_fill.insert("Text16.0.2.0".to_string(), "01".to_string());    // Номер периода (2 знака)
    fields_to_fill.insert("Text17.0".to_string(), year);           // Год отчетного периода (4 знака)

    // Заполнение формы значениями
    fill_pdf_form(&mut doc, &fields_to_fill)
        .map_err(|err| err.process_err(Status::Tech, "Ошибка при заполнении PDF полей"))?;

    // Сохранение обновленного документа в байты
    let mut output_bytes = Vec::new();
    doc.save_to(&mut output_bytes)
        .map_err(|err| err.process_err(Status::FileReadError, "Не удалось сохранить PDF в байты"))?;

    Ok(output_bytes)
}

/// Функция инициализации заполнения интерактивной формы
pub fn fill_pdf_form(doc: &mut Document, data: &HashMap<String, String>) -> Result<(), lopdf::Error> {
    // 1. Получаем ссылку на ID каталога из трейлера документа
    let catalog_ref = doc.trailer.get(b"Root")
        .and_then(|obj| obj.as_reference())?;

    // 2. Получаем изменяемую копию словаря Catalog
    let mut catalog = doc.get_object(catalog_ref)?.as_dict()?.clone();

    // 3. Ищем или создаем AcroForm с добавлением флага автоматического рендеринга полей
    if let Ok(acro_form_obj) = catalog.get(b"AcroForm") {
        if let Ok(acro_form_ref) = acro_form_obj.as_reference() {
            if let Ok(mut acro_form_dict) = doc.get_object(acro_form_ref).and_then(|obj| obj.as_dict()).map(|d| d.clone()) {
                
                // Принудительно заставляем ридер (Acrobat, Chrome) отображать записанный текст
                acro_form_dict.set(b"NeedAppearances", Object::Boolean(true));
                doc.set_object(acro_form_ref, Object::Dictionary(acro_form_dict.clone()));
                
                if let Ok(fields_obj) = acro_form_dict.get(b"Fields") {
                    let mut field_refs = Vec::new();
                    
                    // Безопасно собираем ссылки во временный вектор для обхода ограничений заимствования
                    if let Ok(fields_array) = doc.dereference(fields_obj).and_then(|(_, obj)| obj.as_array()) {
                        for field_item in fields_array {
                            if let Ok(reference) = field_item.as_reference() {
                                field_refs.push(reference);
                            }
                        }
                    }

                    // Модифицируем документ
                    for reference in field_refs {
                        traverse_and_fill_field(doc, reference, String::new(), data)?;
                    }
                }
            }
        }
    }
    
    // Сохраняем обновленный каталог обратно в документ
    doc.set_object(catalog_ref, Object::Dictionary(catalog));
    Ok(())
}

/// Рекурсивная функция обхода и модификации дерева интерактивных полей
fn traverse_and_fill_field(
    doc: &mut Document,
    field_ref: lopdf::ObjectId,
    parent_name: String,
    data: &HashMap<String, String>,
) -> Result<(), lopdf::Error> {
    let mut dict = doc.get_object(field_ref)?.as_dict()?.clone();

    // Шаг 1. Формируем полное имя текущего поля с учетом родителей
    let mut current_name = parent_name.clone();
    if let Ok(t_obj) = dict.get(b"T") {
        if let Ok(name_bytes) = t_obj.as_str() {
            if let Ok(name_str) = String::from_utf8(name_bytes.to_vec()) {
                if !current_name.is_empty() {
                    current_name.push('.');
                }
                current_name.push_str(&name_str);
            }
        }
    }

    // Шаг 2. Проверяем точное совпадение имени (для Text1, Text3, Text8.0 и др.) до спуска в /Kids
    let value_to_set = data.get(&current_name).cloned();

    if let Some(ref val) = value_to_set {
        let pdf_string = Object::String(val.as_bytes().to_vec(), StringFormat::Literal);
        dict.set(b"V", pdf_string);
        dict.remove(b"AP"); 
        doc.set_object(field_ref, Object::Dictionary(dict.clone()));
        
        // Если это групповое поле с виджетами на разных страницах (/Kids),
        // изолируем дочерние ссылки во временный вектор и проталкиваем значения вниз
        if let Ok(kids_obj) = dict.get(b"Kids") {
            let mut kid_refs = Vec::new();
            if let Ok(kids_array) = doc.dereference(kids_obj).and_then(|(_, obj)| obj.as_array()) {
                for kid in kids_array {
                    if let Ok(kid_ref) = kid.as_reference() {
                        kid_refs.push(kid_ref);
                    }
                }
            }
            for kid_ref in kid_refs {
                force_set_value_downstream(doc, kid_ref, val)?;
            }
        }
    }

    // Шаг 3. Если совпадения по имени на текущем уровне нет, собираем ссылки на /Kids и идем вглубь
    if let Ok(kids_obj) = dict.get(b"Kids") {
        let mut kid_refs = Vec::new();
        if let Ok(kids_array) = doc.dereference(kids_obj).and_then(|(_, obj)| obj.as_array()) {
            for kid in kids_array {
                if let Ok(kid_ref) = kid.as_reference() {
                    kid_refs.push(kid_ref);
                }
            }
        }
        for kid_ref in kid_refs {
            traverse_and_fill_field(doc, kid_ref, current_name.clone(), data)?;
        }
    }

    Ok(())
}

/// Вспомогательная функция для сквозного проталкивания значения дочерним виджетам
fn force_set_value_downstream(doc: &mut Document, field_ref: lopdf::ObjectId, value: &str) -> Result<(), lopdf::Error> {
    let mut dict = doc.get_object(field_ref)?.as_dict()?.clone();
    let pdf_string = Object::String(value.as_bytes().to_vec(), StringFormat::Literal);
    
    dict.set(b"V", pdf_string);
    dict.remove(b"AP");
    doc.set_object(field_ref, Object::Dictionary(dict.clone()));

    if let Ok(kids_obj) = dict.get(b"Kids") {
        let mut kid_refs = Vec::new();
        if let Ok(kids_array) = doc.dereference(kids_obj).and_then(|(_, obj)| obj.as_array()) {
            for kid in kids_array {
                if let Ok(kid_ref) = kid.as_reference() {
                    kid_refs.push(kid_ref);
                }
            }
        }
        for kid_ref in kid_refs {
            force_set_value_downstream(doc, kid_ref, value)?;
        }
	}
	Ok(())
}