use tauri_plugin_fs::FsExt;
use tokio::io::AsyncReadExt;

use std::str::FromStr;
use encoding_rs::WINDOWS_1251;
use futures::stream::{self, StreamExt};

use crate::static_data::null_elems::NULLS;
use crate::primitives::frozen::text::{BoxUuid, Kpp};
use crate::{ClientState, ProcessError, Status};
use crate::sql_models::operation::parser::{
    BlockFields,
    ParsedBlock, 
    StatementHead
};
use crate::sql_models::operation::service::{
    OperationInfo,
    OperationStep
};
use crate::sql_models::operation::implements::OperationRaw;
use crate::primitives::composite::implements::RasBicAcc;
use crate::sql_models::company::implements::{CompCrateData, CtrPtyTypes};
use crate::primitives::frozen::text::CompType;

use crate::client::operation::helper::make_statement_block_map;
use crate::client::operation::statement_parser::comment::parse_comment;
use crate::client::sql_queries::companys::get::fun_get_or_parse_insert_many::get_or_parse_insert_companys_function;
use crate::client::operation::statement_parser::make_operations::make_statement_operation_raw;
use crate::client::sql_queries::operations::get::fun_get_exists_by_ids::get_exists_operations_by_ids_function;
use crate::client::sql_queries::companys::get::que::sql_get_company_ids_by_type_function;

pub async fn parse_statement(
    state: &ClientState,
    ras_bic_acc: &RasBicAcc,
    path: &str
) -> Result<OperationStep, Status> {

    let failed_result = OperationStep::TryLater {text: OperationInfo::ClientApiSystemError,};

	let session = match state.get_session().await {
		Ok(s) => s,
		Err(err) => {
			err.process_err(err, None);
			return Ok(failed_result);
		}
	};

    let mut new_company_pairs: std::collections::HashSet<CompCrateData> = std::collections::HashSet::new();

    let mut parsed_blocks: Vec<ParsedBlock> = vec![];

	let tauri_file_path = tauri_plugin_fs::FilePath::from_str(path)
		.map_err(|_| Status::FilePathError)?;

	let mut tauri_open_options = tauri_plugin_fs::OpenOptions::new();
	tauri_open_options.read(true);

	let open_file_obj = state.app_handle.fs().open(tauri_file_path.clone(), tauri_open_options.clone())
		.map_err(|err| {
			let ext_info = format!("wrong tauri_file_path = {:?}, wrong tauri_open_options = {:?}", tauri_file_path, tauri_open_options);
			err.process_err(Status::FileReadError, Some(&ext_info))
		})?;
	
	let mut file = tokio::fs::File::from_std(open_file_obj);
	let mut bytes: Vec<u8> = vec![];

	if let Err(err) = file.read_to_end(&mut bytes).await {
		let ext_info = format!("wrong file path = {:?}, wrong tokio file = {:?}", path, file);
		err.process_err(Status::FileReadError, Some(&ext_info));
		return Ok(failed_result);
	}

    let buffer = match String::from_utf8(bytes) {
        Ok(good_utf8) => good_utf8,
        Err(err) => {
            let win1251_bytes = err.into_bytes();
            let (cow, _, _) = WINDOWS_1251.decode(&win1251_bytes);
            cow.into_owned()
        }
    };

    let mut data_iter = buffer.split("СекцияД");

    let full_head = match data_iter.next() {
        Some(a) => a.to_string(),
        None =>  {
            Status::Tech.process_err(Status::DataCorruptionErr, None);
            return Ok(OperationStep::Loading { text: OperationInfo::WrongFile });
        }
    };

    let mut full_head_iter = full_head.split("СекцияРасчСчет");

    match full_head_iter.next() {
        Some(_) => {},
        None =>  {
            Status::Tech.process_err(Status::DataCorruptionErr, None);
            return Ok(OperationStep::Loading { text: OperationInfo::WrongFile });
        }
    };

    let head_str  = match full_head_iter.next() {
        Some(a) => a.trim(),
        None =>  {
            Status::Tech.process_err(Status::DataCorruptionErr, None);
            return Ok(OperationStep::Loading { text: OperationInfo::WrongFile });
        }
    };

    let head_map = make_statement_block_map(head_str);

    let head = match StatementHead::from_map(&head_map) {
        Ok(h) => h,
        Err(err) => {
            err.process_err(err, None);
            return Ok(failed_result);
        }
    };

    if head.head_acc != ras_bic_acc.ras_acc {
        return Ok(OperationStep::Loading {
            text: OperationInfo::WrongBankAcc,
        });
    }

    for block_str in data_iter {
        let block_map: std::collections::HashMap<&str, &str> = make_statement_block_map(block_str);

		let mut block_fields = BlockFields::try_from(&block_map)
    		.map_err(|err| err.process_err(err, None))?;

		if block_fields.pay_inn.len() == 12 {
			block_fields.pay_kpp = Kpp::unchecked("0".to_string());
		}

		if block_fields.rec_inn.len() == 12 {
			block_fields.rec_kpp = Kpp::unchecked("0".to_string());
		}

        let comment_data = parse_comment(&block_fields.doc_comment);

		let pay_key: CompCrateData = CompCrateData {comp_inn: block_fields.pay_inn.clone(), kpp: block_fields.pay_kpp.clone()};
		new_company_pairs.insert(pay_key);

		let rec_key: CompCrateData = CompCrateData {comp_inn: block_fields.rec_inn.clone(), kpp: block_fields.rec_kpp.clone()};
		new_company_pairs.insert(rec_key);

        parsed_blocks.push(ParsedBlock { block_fields, comment_data});

    }

	let new_company_pairs: Vec<CompCrateData> = new_company_pairs
		.into_iter()
		.filter(|x| (x.comp_inn.len() == 10 && x.kpp.len() == 9) || (x.comp_inn.len() == 12 && x.kpp == NULLS.p.ip_kpp))
		.collect();

	if let Err(err) = get_or_parse_insert_companys_function(state, new_company_pairs).await {
		err.process_err(err, None);
		return Ok(OperationStep::from_err(&err));
	}

    let mut operations: Vec<OperationRaw> = vec![];   

	let gov: std::collections::HashSet<BoxUuid> = match sql_get_company_ids_by_type_function(&session.local_db, &CompType::Gov).await {
		Ok(v) => v.into_iter().collect(),
		Err(err) => {
			err.process_err(err, None);
			return Ok(OperationStep::from_err(&err));
		}
	};

	let com_ent: std::collections::HashSet<BoxUuid> = match sql_get_company_ids_by_type_function(&session.local_db, &CompType::ComEnt).await {
		Ok(v) => v.into_iter().collect(),
		Err(err) => {
			err.process_err(err, None);
			return Ok(OperationStep::from_err(&err));
		}
	};

	let ip: std::collections::HashSet<BoxUuid> = match sql_get_company_ids_by_type_function(&session.local_db, &CompType::Ip).await {
		Ok(v) => v.into_iter().collect(),
		Err(err) => {
			err.process_err(err, None);
			return Ok(OperationStep::from_err(&err));
		}
	};

	let banks: std::collections::HashSet<BoxUuid> = match sql_get_company_ids_by_type_function(&session.local_db, &CompType::Bank).await {
		Ok(v) => v.into_iter().collect(),
		Err(err) => {
			err.process_err(err, None);
			return Ok(OperationStep::from_err(&err));
		}
	};

	let ctrpty_types = CtrPtyTypes {gov, com_ent, ip, banks};

	let mut tasks_vec = vec![];

    for block in &parsed_blocks {
        tasks_vec.push(make_statement_operation_raw(state, block, &ctrpty_types));
    }

    let mut task_stream = stream::iter(tasks_vec).buffer_unordered(2);

    while let Some(res) = task_stream.next().await {
        match res {
            Ok(o) => operations.push(o),
            Err(err) => {
                err.process_err(err, None);
                return Ok(OperationStep::from_err(&err));
            } 
        }
    }

	let check_ids: Vec<BoxUuid> = operations.iter().map(|x| x.oper_id.clone()).collect();

	let exist_ids: std::collections::HashSet<BoxUuid> = get_exists_operations_by_ids_function(
		state, &check_ids
	).await
	.map_err(|err| err.process_err(err, None))?
	.into_iter()
	.collect();

    for operation in operations.iter_mut() {
        if exist_ids.contains(&operation.oper_id) {
            operation.is_duplicate  = true;
        }
    }
    
    let success_result = OperationStep::StatementSuccess {
        text: OperationInfo::SuccessRaw,
        operations
    };

    Ok(success_result)
    
}