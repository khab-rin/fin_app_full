use rust_decimal::prelude::ToPrimitive;

use crate::primitives::frozen::text::{CompStatus, Phone};
use crate::service::reports::fns_xsd_shemas::common::FnsSignerType;
use crate::service::reports::fns_xsd_shemas::usn_1152017_decl::UsnDeclTaxPayerCompany;
use crate::{ClientState, ProcessError, Status};
use crate::service::reports::service::{ReportStep, ReportInfo, VerifyPowersResult};
use crate::service::reports::fns_xsd_shemas::usn_1152017_decl::*;
use crate::primitives::frozen::text_base::{Digits4_4, String1_120};
use crate::primitives::tax_frozen::implements::{Tax, Usn6};

use crate::service::api_routes::implements::ApiRoutes;
use crate::service::mchd::home_mchd_power::HomeMchdPower;

use crate::client::back_api::post_query::post_query_back_api;
use crate::client::reports::fns::helper::{make_quaters, count_quater_taxes_usn6_decl};
use crate::client::sql_queries::operations::get::reports::fns::usn_incomes::get_quater_cummul_incomes_usn;
use crate::client::sql_queries::operations::get::reports::fns::usn_social_fee::get_quater_cummul_social_usn;




pub async fn make_decl_6_files(
	state: &ClientState,
	year: i32,
	qu: i32,
	fns_branch: Digits4_4,

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
		&state, 
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

	let cumul_incomes_kop = match get_quater_cummul_incomes_usn(state, &dates).await {
		Ok(c) => c,
		Err(err) => {
			err.process_err(err, "");
			return failed_result;
		}
	};

	let taxable_income = UsnDeclQuaterAmnts {
		first_qu: cumul_incomes_kop.q1.as_ref().round().to_u64(),
		second_qu: cumul_incomes_kop.q2.as_ref().round().to_u64(),
		third_qu: cumul_incomes_kop.q3.as_ref().round().to_u64(),
		fourth_qu: cumul_incomes_kop.q4.as_ref().round().to_u64().unwrap_or(0),
	};

	let rate = UsnDeclRate {
		qu_one: Some(Tax::Usn6(Usn6::default())),
		qu_two: Some(Tax::Usn6(Usn6::default())),
		qu_three: Some(Tax::Usn6(Usn6::default())),
		qu_four: Tax::Usn6(Usn6::default()),
		rate_reason: None
	};

	let first_qu = taxable_income.first_qu
		.zip(rate.qu_one.clone())
		.map(|(inc, r)| r.multiply_u64(inc));

	let second_qu = taxable_income.second_qu
		.zip(rate.qu_two.clone())
		.map(|(inc, r)| r.multiply_u64(inc));

	let third_qu = taxable_income.third_qu
		.zip(rate.qu_three.clone())
		.map(|(inc, r)| r.multiply_u64(inc));

	let fourth_qu = rate.qu_four.clone().multiply_u64(taxable_income.fourth_qu); 

	let calc_tax = UsnDeclQuaterAmnts {
		first_qu,
		second_qu,
		third_qu,
		fourth_qu,
	};

	let tax_deduction_dec = match get_quater_cummul_social_usn(state, &dates).await {
		Ok(a) => a,
		Err(err) => {
			err.process_err(err, "");
			return failed_result;
		}
	};

	let tot_social = UsnDeclQuaterAmnts {
		first_qu: tax_deduction_dec.q1.as_ref().round().to_u64()
			.zip(calc_tax.first_qu)
			.map(|(vznos, tax)| vznos.min(tax)),

		second_qu: tax_deduction_dec.q2.as_ref().round().to_u64()
			.zip(calc_tax.second_qu)
			.map(|(vznos, tax)| vznos.min(tax)),

		third_qu: tax_deduction_dec.q3.as_ref().round().to_u64()
			.zip(calc_tax.third_qu)
			.map(|(vznos, tax)| vznos.min(tax)),

		fourth_qu: tax_deduction_dec.q4.as_ref().round().to_u64().unwrap_or(0).min(calc_tax.fourth_qu)
	};

	let ip_social = if session.session_user.company.comp_inn.len() == 12 {
		let total_income_year = taxable_income.fourth_qu; 
		let calculated_1_percent = total_income_year.saturating_sub(300_000) / 100;
		let one_perc_curr = calculated_1_percent.min(319_444);

		Some(UsnDeclQuaterAmntsInfo {
			fix_amnt: 57390,
			one_perc: one_perc_curr,
			one_perc_curr_year: one_perc_curr,
			one_perc_prev_year: 0,
		})
	} else {
		None
	};

	let trade_fee: Option<UsnDeclTradeFee> = None;

	let employers_exist = UsnDeclEmplyersExistType::WithoutEmployees;

	let usn_decl_tax_six_calculation = UsnDeclTaxSixCal {
		employers_exist,
		taxable_income,
		rate,
		calc_tax: calc_tax.clone(),
		tot_social: tot_social.clone(),
		ip_social,
		trade_fee,
	};

	let oktmo = if let Some(o) = session
		.session_user
		.company
		.metadata
		.oktmo_company.as_ref() {
			o.clone()
		} else {
			Status::Tech.process_err(Status::SystemLogicErr, "");
			return failed_result;
		};

	let tax_q1_i64 = (calc_tax.first_qu.unwrap_or(0) as i64) - (tot_social.first_qu.unwrap_or(0) as i64);
	let tax_q2_i64 = (calc_tax.second_qu.unwrap_or(0) as i64) - (tot_social.second_qu.unwrap_or(0) as i64);
	let tax_q3_i64 = (calc_tax.third_qu.unwrap_or(0) as i64) - (tot_social.third_qu.unwrap_or(0) as i64);
	let tax_q4_i64 = (calc_tax.fourth_qu.clone() as i64) - (tot_social.fourth_qu.clone() as i64);

	let avans_qu_one: i64 = tax_q1_i64;
	let avans_qu_two: i64 = tax_q2_i64 - tax_q1_i64;
	let avans_qu_three: i64 = tax_q3_i64 - tax_q2_i64;
	let tax_final_year: i64 = tax_q4_i64 - tax_q3_i64; 
	
	let usn_decl_tax_six = UsnDeclTaxSix {
		calculation: usn_decl_tax_six_calculation,
		oktmo_qu_one: oktmo,
		avans_qu_one: Some(avans_qu_one),
		oktmo_qu_two: None,
		avans_qu_two: Some(avans_qu_two),
		oktmo_qu_three: None,
		avans_qu_three: Some(avans_qu_three),
		oktmo_qu_four: None,
		avans_qu_four: tax_final_year,
		patent_tax: None		
	};

	



	










	Err(Status::Unknown)
}