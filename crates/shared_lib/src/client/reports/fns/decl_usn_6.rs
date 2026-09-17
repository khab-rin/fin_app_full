use crate::primitives::frozen::text::{CompStatus, Date, Kpp, Phone};
use crate::service::reports::fns_xsd_shemas::usn_1152017_decl::UsnDeclTaxPayerCompany;
use crate::{ClientState, ProcessError, Status};
use crate::service::reports::service::{ReportStep, ReportInfo};
use crate::service::reports::fns_xsd_shemas::usn_1152017_decl::*;
use crate::primitives::frozen::text_base::{Digits4_4, String1_40};
use crate::primitives::tax_frozen::implements::Usn15;
use crate::service::reports::fns_xsd_shemas::common::*;

use crate::client::reports::fns::helper::make_quaters;
use crate::client::sql_queries::operations::get::reports::fns::usn_incomes::get_quater_cummul_incomes_usn;
use crate::client::sql_queries::operations::get::reports::fns::usn_costs::get_quater_cummul_costs_usn;
use crate::client::reports::fns::notif_pdf::make_notif_pdf;
use crate::client::reports::fns::helper::make_file_id;
use crate::client::back_api::post_query::post_query_back_api;

use crate::sql_models::company::implements::Company;


pub async fn make_decl_6_files(
	state: &ClientState,
	year: i32,
	qu: i32,
	fns_branch: Digits4_4,

) -> Result<ReportStep, Status> {

	let failed_result = ReportStep::TryLater { text: ReportInfo::ClientServiceError };

	let session = state.get_session().await
		.map_err(|err| err.process_err(err, ""))?;

	let user_id = session.session_user.user.user_id.clone();

	let power_verify_response = post_query_back_api(
		&state, 
		state.config.get_std_client(), 
		ApiRoutes::MchdVerivyPower, 
		&(user_id, &HomeMchdPower::FNS02))
		.await
		.map_err(|err| err.process_err(err, ""))?;

	let power_verify = 


	let comp_name = session.session_user.company.metadata.comp_name
		.as_ref()
		.ok_or_else(|| Status::Tech.process_err(Status::DataCorruptionErr, ""))?
		.short_egrul_name
		.as_ref()
		.ok_or_else(|| Status::Tech.process_err(Status::DataCorruptionErr, ""))?
		.clone();

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


	Err(Status::Unknown)
}