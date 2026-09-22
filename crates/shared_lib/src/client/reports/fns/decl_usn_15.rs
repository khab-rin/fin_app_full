use crate::primitives::frozen::text_base::Digits4_4;
use crate::{ClientState, ProcessError, Status};
use crate::service::reports::service::{VerifyPowersResult, ReportStep, ReportInfo, QuartCummulAmnt};
use crate::service::api_routes::implements::ApiRoutes;
use crate::service::mchd::home_mchd_power::HomeMchdPower;
use crate::primitives::frozen::text::{CompStatus, Phone, RubF};
use crate::primitives::frozen::text_base::{String1_120};
use crate::service::reports::fns_xsd_shemas::usn_1152017_decl::*;
use crate::service::reports::fns_xsd_shemas::common::*;
use crate::primitives::tax_frozen::implements::{Tax, Usn15};
use crate::primitives::calculated::implements::RubC;

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

	let signer = match power_verify.is_manager {
		true => FnsSignerType::TAXPAYER,
		false => FnsSignerType::DELEGATE
	};

	let decl_signer = UsnDeclSigner {fio, delegate_info, signer };

	let dates = match make_quaters(year) {
		Ok(d) => d,
		Err(err) => {
			err.process_err(err, "");
			return failed_result;
		}
	};

	let kumul_incomes_kop = match get_quater_cummul_incomes_usn(state, &dates).await {
		Ok(i) => i,
		Err(err) => {
			err.process_err(err, "");
			return failed_result;
		}
	};

	let kumul_costs_kop = match get_quater_cummul_costs_usn(state, &dates).await {
		Ok(c) => c,
		Err(err) => {
			err.process_err(err, "");
			return failed_result;
		}
	};

	let a = RubF::try_from((kumul_incomes_kop.q1 - kumul_costs_kop.q1).max(RubC::new()));

	let taxable_base_kop = QuartCummulAmnt {
		q1: kumul_incomes_kop.q1 - kumul_costs_kop.q1,
		q2: kumul_incomes_kop.q2 - kumul_costs_kop.q2,
		q3: kumul_incomes_kop.q3 - kumul_costs_kop.q3,
		q4: kumul_incomes_kop.q4 - kumul_costs_kop.q4,
	};

	let income = kumul_incomes_kop.into_fns_quater_amnts();

	let rate = UsnDeclRate {
		qu_one: Some(Tax::Usn15(Usn15::default())),
		qu_two: Some(Tax::Usn15(Usn15::default())),
		qu_three: Some(Tax::Usn15(Usn15::default())),
		qu_four: Tax::Usn15(Usn15::default()),
		rate_reason: None
	};


	Err(Status::Unknown)
}