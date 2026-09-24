use crate::primitives::frozen::text_base::Digits4_4;
use crate::{ClientState, ProcessError, Status};
use crate::service::reports::service::{VerifyPowersResult, ReportStep, ReportInfo, QuartCummulAmnt};
use crate::service::api_routes::implements::ApiRoutes;
use crate::service::mchd::home_mchd_power::HomeMchdPower;
use crate::primitives::frozen::text::{CompStatus, Phone, RubF, Date};
use crate::primitives::frozen::text_base::{String1_120, String1_40};
use crate::service::reports::fns_xsd_shemas::usn_1152017_decl::*;
use crate::service::reports::fns_xsd_shemas::common::*;
use crate::primitives::tax_frozen::implements::{Tax, Usn15};


use crate::client::back_api::post_query::post_query_back_api;
use crate::client::reports::fns::helper::{make_file_id, make_quaters};
use crate::client::sql_queries::operations::get::reports::fns::usn_incomes::get_quater_cummul_incomes_usn;
use crate::client::sql_queries::operations::get::reports::fns::usn_costs::get_quater_cummul_costs_usn;


pub async fn make_decl_15_files(
	state: &ClientState,
	year: i32,
	qu: i32,
	fns_branch: Digits4_4
) -> Result<ReportStep, Status> {

	let failed_result = Ok(ReportStep::TryLater { text: ReportInfo::ClientServiceError });

	let session = match state.get_session().await {
		Ok(s) => s,
		Err(err) => {
			err.process_err(err, "");
			return failed_result;
		}
	};

	let user_id = session.session_user.user.user_id.clone();

	let power_verify_response = match post_query_back_api(
		state, 
		state.config.get_std_client(), 
		ApiRoutes::MchdVerivyPower, 
		&(user_id, &HomeMchdPower::FNS02))
		.await
		{
			Ok(r) => r,
			Err(err) => {
				err.process_err(err, "");
				return failed_result;
			}
		};
	
	let power_verify: VerifyPowersResult = match power_verify_response.json().await {
		Ok(p) => p,
		Err(err) => {
			err.process_err(Status::MappingError, "");
			return failed_result;
		}
	};

	if !power_verify.is_manager && power_verify.mchd_uuid.is_none() {
		return Ok(ReportStep::TryLater { text: ReportInfo::MissPower });
	}

	let datata_comp_name = match session.session_user.company.metadata.comp_name.as_ref() {
		Some(d) => d,
		None => {
			Status::Tech.process_err(Status::DataCorruptionErr, "");
			return failed_result;
		}
	};

	let comp_name = match datata_comp_name.short_egrul_name.as_ref() {
		Some(n) => n.clone(), 
		None => {
			Status::Tech.process_err(Status::DataCorruptionErr, "");
			return failed_result;
		}
	};

	let comp_inn = session.session_user.company.comp_inn.clone();

	let kpp = session.session_user.company.kpp.clone();

	let fio = session.session_user.person.metadata.fio.clone();

	let tel: Option<Phone> = session.session_user.company.metadata.phone.clone();

	let liq_status = match session.session_user.company.comp_status {
		CompStatus::Active => None,
		CompStatus::Liquidated | CompStatus::Liquidating | CompStatus::Bankrupt => Some(UsnDeclLiquidStatus::LIQUIDATION),
		CompStatus::Reorganizing => Some(UsnDeclLiquidStatus:: ACQUISITION)
	};

	let transformation_info = liq_status.map(|st| UsnDeclTrnasformationInfo {
		liq_status:st,
		comp_inn: None,
		kpp: None
	});

	let tax_payer_choice = match session.session_user.company.comp_inn.len() {
		12 => {
			UsnDeclTaxPayerType::Person( UsnDeclTaxPayerPerson {
				fio,
				pers_inn: comp_inn
			})
		},
		_ => {
			UsnDeclTaxPayerType::Company({
				UsnDeclTaxPayerCompany {
					transformation_info,
					comp_name,
					comp_inn,
					kpp
				}
			})
		}
	};

	let tax_payer = UsnDeclTaxPayer {
		tax_payer: tax_payer_choice,
		tel
	};

	let fio = match power_verify.is_manager {
		true => None,
		false => Some(session.session_user.person.metadata.fio.clone())
	};

	let delegate_info = match power_verify.is_manager {
		true => None,
		false => {
			if let Some(g) = power_verify.mchd_uuid {
				Some(UsnDeclDelegateInfo{doc_name:String1_120::unchecked(g.to_string())})
			} else {
				Status::Tech.process_err(Status::SystemLogicErr, "");
				return failed_result;
			}
		}
	};

	let signer_type = match power_verify.is_manager {
		true => FnsSignerType::TAXPAYER,
		false => FnsSignerType::DELEGATE
	};

	let decl_signer = UsnDeclSigner {fio, delegate_info, signer_type };

	let dates = match make_quaters(year) {
		Ok(d) => d,
		Err(err) => {
			err.process_err(err, "");
			return failed_result;
		}
	};

	let kumul_incomes_decimal = match get_quater_cummul_incomes_usn(state, &dates).await {
		Ok(i) => i,
		Err(err) => {
			err.process_err(err, "");
			return failed_result;
		}
	};

	let kumul_costs_decimal = match get_quater_cummul_costs_usn(state, &dates).await {
		Ok(c) => c,
		Err(err) => {
			err.process_err(err, "");
			return failed_result;
		}
	};


	let (q1, q2, q3, q4) = match (|| -> Result<_, Status> {
		Ok((
			RubF::try_from(&kumul_incomes_decimal.q1 - &kumul_costs_decimal.q1)?,
			RubF::try_from(&kumul_incomes_decimal.q2 - &kumul_costs_decimal.q2)?,
			RubF::try_from(&kumul_incomes_decimal.q3 - &kumul_costs_decimal.q3)?,
			RubF::try_from(&kumul_incomes_decimal.q4 - &kumul_costs_decimal.q4)?,
		))
	})() {
		Ok(quarters) => quarters,
		Err(err) => {
			err.process_err(err, ""); // Ошибка перехвачена и обработана
			return failed_result;
		}
	};


	let taxable_base_decimal = QuartCummulAmnt {q1, q2, q3, q4};

	let (q1, q2, q3, q4) = match (|| -> Result<_, Status> {
		Ok((
			RubF::try_from(&taxable_base_decimal.q1 * Usn15::default())?,
			RubF::try_from(&taxable_base_decimal.q2 * Usn15::default())?,
			RubF::try_from(&taxable_base_decimal.q3 * Usn15::default())?,
			RubF::try_from(&taxable_base_decimal.q4 * Usn15::default())?,
		))
	})() {
		Ok(quarters) => quarters,
		Err(err) => {
			err.process_err(err, ""); // Ошибка перехвачена и обработана
			return failed_result;
		}
	};

	let calculated_tax_decimal = QuartCummulAmnt {q1, q2, q3, q4};

	

	let income = QuartCummulAmnt::into_fns_quater_amnts(&kumul_incomes_decimal);
	let expenses = QuartCummulAmnt::into_fns_quater_amnts(&kumul_costs_decimal);
	let taxable_base = QuartCummulAmnt::into_fns_quater_amnts(&taxable_base_decimal);
	let calculated_tax = QuartCummulAmnt::into_fns_quater_amnts(&calculated_tax_decimal);
	let min_tax = (income.fourth_qu as f64 * 0.01).round() as i64;
	let tax_9months = calculated_tax.third_qu.unwrap_or(0);

	let due_choice = if min_tax > calculated_tax.fourth_qu {
		let min_tax_diff = min_tax - tax_9months;
		if min_tax_diff >= 0 {
			UsnDeclFifteenDueChoise::MinTaxAmnt(min_tax_diff)
		} else {
			UsnDeclFifteenDueChoise::Due(min_tax_diff) 
		}
	} else {
		let tax_diff = calculated_tax.fourth_qu - tax_9months;
		UsnDeclFifteenDueChoise::Due(tax_diff)
	};


	let (q1, q2, q3) = (
		calculated_tax.first_qu.unwrap_or(0),
		(calculated_tax.second_qu.unwrap_or(0) - calculated_tax.first_qu.unwrap_or(0)).max(0),
		(calculated_tax.third_qu.unwrap_or(0) - calculated_tax.second_qu.unwrap_or(0)).max(0),
	);


	let rate = UsnDeclRate {
		qu_one: Some(Tax::Usn15(Usn15::default())),
		qu_two: Some(Tax::Usn15(Usn15::default())),
		qu_three: Some(Tax::Usn15(Usn15::default())),
		qu_four: Tax::Usn15(Usn15::default()),
		rate_reason: None
	};

	let required_social = if session.session_user.company.comp_inn.len() == 12 {
		let total_base = taxable_base.fourth_qu; 
		let calculated_1_percent = (total_base - 300000).max(0) / 100;
		let one_perc_curr = calculated_1_percent.min(321818);

		Some(IpSocialAmnts {
			fix_amnt: 57390,
			one_perc: one_perc_curr,
			one_perc_curr_year: one_perc_curr,
			one_perc_prev_year: 0,
		})
	} else {
		None
	};

	let prev_negative_taxable_base: Option<i64> = None;
	

	let calculation = UsnDeclTaxFifteenCal {
		prev_negative_taxable_base,
		min_tax,
		income,
		expenses,
		taxable_base,
		rate,
		calculated_tax: Some(calculated_tax),
		required_social
	};

	let oktmo = match session.session_user.company.metadata.oktmo_company.as_ref() {
		Some(o) => o.clone(),
		None => {
			Status::Tech.process_err(Status::SystemLogicErr, "");
			return failed_result;
		}
	};

	

	let declaration = UsnDeclTaxFifteen {
		oktmo_qu_one: oktmo,
		avans_qu_one: Some(q1),
		oktmo_qu_two: None,
		avans_qu_two: Some(q2),
		oktmo_qu_three: None,
		avans_qu_three: Some(q3),
		oktmo_qu_four: None,
		patent_tax: None,
		due_choice,
		calculation
	};

	let usn_report = UsnDeclUsnReport {
		usn_object: UsnDeclUsnObject::IncomeMinusExpenses,
		report_type: UsnDeclTaxReportType::FifteenPercent(Box::new(declaration)),
		charity_report: None,
		kkt_expense: None
	};

	let submission_place = match session.session_user.company.comp_inn.len() {
		12 => UsnDeclSubmissionPlace::IndividualBusinessAddress,
		_ => UsnDeclSubmissionPlace::CompanyAddress
	};


	let document = UsnDeclUsnDocument {
		report_code: FnsKnd::UsnDeclatation,
		doc_create_date: Date::unchecked(chrono::Utc::now().date_naive()),
		report_period: UsnDeclReportPeriod::Year,
		report_year: Digits4_4::unchecked(year.to_string()),
		branch_code: fns_branch.clone(),
		report_version: 0,
		submission_place,
		tax_payer,
		signer: decl_signer,
		usn_report
	};

	let file_id = match make_file_id(&session, &fns_branch, FnsKnd::UsnDeclatation){
		Ok(f) => f,
		Err(err) => {
			err.process_err(err, "");
			return failed_result;
		}
	};

	let version_str = format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

	let program_version = String1_40::unchecked(version_str);

	let format_version = FnsDocFormVersion::UsnDeclatation;

	let decl_file = UsnDeclUsnFile {
		document,
		file_id: file_id.clone(),
		program_version,
		format_version
	};

	let mut xml_string = String::with_capacity(4096);

	 if let Err(err) = quick_xml::se::to_writer_with_root(&mut xml_string, "Файл", &decl_file) {
		err.process_err(Status::FileWriteError, "");
		return failed_result;
	}

	let mut xml_file: Vec<u8> = Vec::with_capacity(xml_string.len() + 50);
	xml_file.extend_from_slice(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
	xml_file.extend_from_slice(xml_string.as_bytes());

	let xml_name = format!("{}.xml", file_id);
	let pdf_name = format!("{}.pdf", file_id);



	Err(Status::Unknown)
}