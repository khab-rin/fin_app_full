use crate::primitives::frozen::text::MidName;
use crate::Status;
use crate::primitives::tax_frozen::implements::Tax;

use crate::primitives::tax_frozen::implements::Usn15;
use crate::service::auth_service::general::ActiveSession;
use crate::service::reports::fns_xsd_shemas::usn_1152017_decl::*;
use crate::client::reports::fns::usn_decl_notif_elems::Elems;
use crate::client::reports::fns::pdf_fill::custom_pdf_fill;

pub fn fill_usn_decl_elems_make_pdf(
	session: &ActiveSession,
	decl: &UsnDeclUsnFile,
	pdf_tpl_bytes: &[u8]
) -> Result<Vec<u8>, Status> {

	let mut elems = Elems::default();

	match &decl.document.tax_payer.tax_payer {
		UsnDeclTaxPayerType::Company(company) =>  {
			elems.text7_0 = company.comp_name.to_string()
		},
		UsnDeclTaxPayerType::Person(person) => {
			elems.text7_0 = person.fio.sur_name.to_string();
			elems.text7_1 = person.fio.first_name.to_string();
			if let Some(m) = &person.fio.mid_name {
				elems.text7_2 = m.to_string()
			}

		}
	}

	elems.text1 = session.session_user.company.comp_inn.to_string();
	elems.text2 = session.session_user.company.kpp.to_string();
	if &elems.text2 == "0" || elems.text2.is_empty() {
		elems.text2 = "-".to_string();
	}
	elems.text3 = decl.document.report_version.to_string();
	elems.text4 = decl.document.report_period.to_string();
	elems.text5_0 = decl.document.report_year.to_string();
	elems.text5_1 = decl.document.submission_place.to_string();
	elems.text6 = decl.document.branch_code.to_string();

	match &decl.document.tax_payer.tax_payer {
		UsnDeclTaxPayerType::Company(company) =>  {
			elems.text7_0 = company.comp_name.to_string();
			if let Some(info) = &company.transformation_info {
				elems.text10_0 = info.liq_status.to_string();
				if let Some(inn) = &info.comp_inn {
					elems.text11_0 = inn.to_string();
				}
				if let Some(kpp) = &info.kpp {
					elems.text11_1 = kpp.to_string();
				}
			}
		},
		UsnDeclTaxPayerType::Person(person) => {
			elems.text7_0 = person.fio.sur_name.to_string();
			elems.text7_1 = person.fio.first_name.to_string();
			if let Some(m) = &person.fio.mid_name {
				elems.text7_2 = m.to_string()
			}

		}
	}

	if let Some(tel) = &decl.document.tax_payer.tel {
		elems.text9 = tel.to_string();
	}

	elems.text10_1 = decl.document.usn_report.usn_object.to_string();

	if let Some(delegate) = &decl.document.signer.delegate_info {
		elems.text12_3 = delegate.doc_name.to_string();
	}

	elems.text14 = decl.document.signer.signer_type.to_string();
	elems.text12_0 = session.session_user.person.metadata.fio.sur_name.to_string();
	elems.text12_1 = session.session_user.person.metadata.fio.first_name.to_string();
	elems.text12_2 = session.session_user.person.metadata.fio.mid_name.clone().unwrap_or(MidName::unchecked("")).to_string();

	if let Some(delegate_info) = &decl.document.signer.delegate_info {
		elems.text12_3 = delegate_info.doc_name.to_string();
	}

	let date_str = decl.document.doc_create_date.to_string();
	let y_m_d: Vec<&str> = date_str.split('-').collect();

	elems.text15_0 = y_m_d[2].to_string();
	elems.text15_1 = y_m_d[1].to_string();
	elems.text15_2 = y_m_d[0].to_string();

	match &decl.document.usn_report.report_type {
		UsnDeclTaxReportType::FifteenPercent(report) => {
			fill_fifteen_usn_elems(report, &mut elems);
		},
		UsnDeclTaxReportType::SixPercent(report) => {
			fill_six_usn_elems(report, &mut elems)
		}
	}

	elems.text13_0 = "4".to_string();
	elems.text13_1 = "0".to_string();

	custom_pdf_fill(&elems, pdf_tpl_bytes)
	
}

pub fn fill_fifteen_usn_elems(
	report: &UsnDeclTaxFifteen,
	elems: &mut Elems
) {
	// Технические признаки — оставляем числами через to_string
	let s_t_20 = 2;
	let s_t_42 = 3;
	let s_t_47 = 4;

	elems.text20 = s_t_20.to_string();
	elems.text42 = s_t_42.to_string();
	elems.text47 = s_t_47.to_string();

	// Доходы (Строки 210-213)
	let s210 = report.calculation.income.first_qu.unwrap_or(0);
	elems.text27_0 = zero_to_(s210);

	let s211 = report.calculation.income.second_qu.unwrap_or(0);
	elems.text27_1 = zero_to_(s211);

	let s212 = report.calculation.income.third_qu.unwrap_or(0);
	elems.text27_2 = zero_to_(s212);
	
	let s213 = report.calculation.income.fourth_qu;
	elems.text27_3 = zero_to_(s213);

	// Расходы (Строки 220-223)
	let s220 = report.calculation.expenses.first_qu.unwrap_or(0);
	elems.text27_4 = zero_to_(s220);
	
	let s221 = report.calculation.expenses.second_qu.unwrap_or(0);
	elems.text27_5 = zero_to_(s221);
	
	let s222 = report.calculation.expenses.third_qu.unwrap_or(0);
	elems.text27_6 = zero_to_(s222);
	
	let s223 = report.calculation.expenses.fourth_qu;
	elems.text27_7 = zero_to_(s223);

	// Убыток прошлых лет (Строка 230)
	let s230 = report.calculation.prev_negative_taxable_base.unwrap_or(0);
	elems.text27_8 = zero_to_(s230);

	// Налоговая база для исчисления налога (Строки 240-243)
	let s240 = (s210 - s220).max(0);
	elems.text27_9 = zero_to_(s240);

	let s241 = (s211 - s221).max(0);
	elems.text27_10 = zero_to_(s241);

	let s242 = (s212 - s222).max(0);
	elems.text27_11 = zero_to_(s242);

	let s243 = (s213 - s223).max(0);
	elems.text27_12 = zero_to_(s243);

	let s250 = (s220 - s210).max(0);
	elems.text27_13 = zero_to_(s250);

	let s251 = (s221 - s211).max(0);
	elems.text27_14 = zero_to_(s251);

	let s252 = (s222 - s212).max(0);
	elems.text27_15 = zero_to_(s252);

	let s253 = (s223 - s213).max(0);
	elems.text27_16 = zero_to_(s253);


	let (r1, r2, r3, r4) = (
		report.calculation.rate.qu_one.as_ref().unwrap_or(&Tax::Usn15(Usn15::default())).clone(),
		report.calculation.rate.qu_two.as_ref().unwrap_or(&Tax::Usn15(Usn15::default())).clone(),
		report.calculation.rate.qu_three.as_ref().unwrap_or(&Tax::Usn15(Usn15::default())).clone(),
		&report.calculation.rate.qu_four,
	);

	let (s260, s260_1) = Tax::get_parts(&r1);
	let (s261, s261_1) = Tax::get_parts(&r2);
	let (s262, s262_1) = Tax::get_parts(&r3);
	let (s263, s263_1) = Tax::get_parts(r4);

	elems.text28_0_0 = s260.to_string();
	elems.text28_0_1 = s261.to_string();
	elems.text28_0_2 = s262.to_string();
	elems.text28_0_3 = s263.to_string();
	elems.text28_1_0 = s260_1.to_string();
	elems.text28_1_1 = s261_1.to_string();
	elems.text28_1_2 = s262_1.to_string();
	elems.text28_1_3 = s263_1.to_string();

	let (s270, s271, s272, s273) = if let Some(calc) = &report.calculation.calculated_tax {
		(
			calc.first_qu.unwrap_or(0),
			calc.second_qu.unwrap_or(0),
			calc.third_qu.unwrap_or(0),
			calc.fourth_qu
		)
	} else {
		(0, 0, 0, 0)
	};
	elems.text27_17 = zero_to_(s270);
	elems.text27_18 = zero_to_(s271);
	elems.text27_19 = zero_to_(s272);
	elems.text27_20 = zero_to_(s273);

	let s280 = report.calculation.min_tax;
	elems.text27_21_0 = zero_to_(s280);

	// Страховые взносы (Строки 290-320)
	if let Some(social) = &report.calculation.required_social {
		let s290 = social.fix_amnt;
		elems.text27_21_1 = zero_to_(s290);

		let s300 = social.one_perc;
		elems.text26_2_1_0_0 = zero_to_(s300);

		let s310 = social.one_perc_curr_year;
		elems.text26_2_1_0_1 = zero_to_(s310);

		let s320 = social.one_perc_prev_year;
		elems.text26_2_1_0_2 = zero_to_(s320);
	}

	// ОКТМО — оставляем как есть через to_string
	elems.text21_0 = report.oktmo_qu_one.to_string();

	if let Some(o) = &report.oktmo_qu_two {
		elems.text21_1 = o.to_string();
	}

	if let Some(o) = &report.oktmo_qu_three {
		elems.text21_2 = o.to_string();
	}

	if let Some(o) = &report.oktmo_qu_four {
		elems.text21_3 = o.to_string();
	}

	// Раздел 1.2 — Авансовые платежи к уплате / уменьшению
	let s020 = s270;
	elems.text22_0 = zero_to_(s020);

	let s040 = (s271 - s020).max(0);
	elems.text22_1 = zero_to_(s040);

	let s050 = (s020 - s271).max(0);
	elems.text22_2 = zero_to_(s050);

	let s070 = (s272 - (s020 + s040 - s050)).max(0);
	elems.text22_3 = zero_to_(s070);

	let s080 = ((s020 + s040 - s050) - s272).max(0);
	elems.text22_4 = zero_to_(s080);

	let s100 = (s273 - (s020 + s040 - s050 - s070 - s080)).max(0);
	elems.text22_5 = zero_to_(s100);

	let s101 = report.patent_tax.unwrap_or(0);
	elems.text22_6 = zero_to_(s101);

	// Строки 110 и 120 (Налог к уменьшению / Минимальный налог)
	let s110 = if s273 >= s280 {
		if s273 - s272 < 0 { (s272 - s273).max(0) } else { 0 }
	} else {
		if s272 > s280 { (s272 - s280).max(0) } else { 0 }
	};
	elems.text22_7_0 = zero_to_(s110);

	let s120 = if s280 > s273 && s280 > ((s020 + s040 - s050 + s070 - s080) + s101) {
		(s280 - (s020 + s040 - s050 + s070 - s080) - s101).max(0)
	} else {
		0
	};
	elems.text22_7_1 = zero_to_(s120);
}




pub fn fill_six_usn_elems(
	report: &UsnDeclTaxSix,
	elems: &mut Elems
) {
	// Технические признаки — оставляем числами через to_string
	let s_t_16 = 2;
	let s_t_23 = 3;
	let s_t_41 = 4;

	elems.text16 = s_t_16.to_string();
	elems.text23 = s_t_23.to_string();
	elems.text41 = s_t_41.to_string();

	// Признак налогоплательщика (1 или 2) — оставляем to_string
	let s102 = report.calculation.employers_exist;
	elems.text24_0 = s102.to_string();

	// Доходы (Строки 110-113)
	let s110 = report.calculation.taxable_income.first_qu.unwrap_or(0);
	elems.text25_0 = zero_to_(s110);

	let s111 = report.calculation.taxable_income.second_qu.unwrap_or(0);
	elems.text25_1 = zero_to_(s111);

	let s112 = report.calculation.taxable_income.third_qu.unwrap_or(0);
	elems.text25_2 = zero_to_(s112);

	let s113 = report.calculation.taxable_income.fourth_qu;
	elems.text25_3 = zero_to_(s113);

	// Ставки налога — оставляем через to_string, так как это проценты
	let (r1, r2, r3, r4) = (
		report.calculation.rate.qu_one.as_ref().unwrap_or(&Tax::Usn15(Usn15::default())).clone(),
		report.calculation.rate.qu_two.as_ref().unwrap_or(&Tax::Usn15(Usn15::default())).clone(),
		report.calculation.rate.qu_three.as_ref().unwrap_or(&Tax::Usn15(Usn15::default())).clone(),
		&report.calculation.rate.qu_four,
	);

	let (s120, s120_1) = Tax::get_parts(&r1);
	let (s121, s121_1) = Tax::get_parts(&r2);
	let (s122, s122_1) = Tax::get_parts(&r3);
	let (s123, s123_1) = Tax::get_parts(r4);

	elems.text34_0 = s120.to_string();
	elems.text34_1 = s121.to_string();
	elems.text34_2 = s122.to_string();
	elems.text34_3 = s123.to_string();
	elems.text35_0 = s120_1.to_string();
	elems.text35_1 = s121_1.to_string();
	elems.text35_2 = s122_1.to_string();
	elems.text35_3 = s123_1.to_string();

	// Исчисленный налог (Строки 130-133)
	let s130 = report.calculation.calc_tax.first_qu.unwrap_or(0);
	let s131 = report.calculation.calc_tax.second_qu.unwrap_or(0);
	let s132 = report.calculation.calc_tax.third_qu.unwrap_or(0);
	let s133 = report.calculation.calc_tax.fourth_qu;

	elems.text25_5 = zero_to_(s130);
	elems.text25_6 = zero_to_(s131);
	elems.text25_7 = zero_to_(s132);
	elems.text25_8 = zero_to_(s133);

	// Налоговый вычет / Страховые взносы к уменьшению (Строки 140-143)
	let s140 = report.calculation.tot_social.first_qu.unwrap_or(0).min(s130);
	let s141 = report.calculation.tot_social.second_qu.unwrap_or(0).min(s131);
	let s142 = report.calculation.tot_social.third_qu.unwrap_or(0).min(s132);
	let s143 = report.calculation.tot_social.fourth_qu.min(s133);

	elems.text25_9 = zero_to_(s140);
	elems.text25_10 = zero_to_(s141);
	elems.text25_11_0 = zero_to_(s142);
	elems.text25_11_1_0 = zero_to_(s143);

	// Фиксированные страховые взносы IP
	let (s150, s160, s161, s162) = if let Some(social) = &report.calculation.ip_social {
		(
			social.fix_amnt,
			social.one_perc,
			social.one_perc_curr_year,
			social.one_perc_prev_year
		)
	} else {
		(0, 0, 0, 0)
	};

	elems.text25_11_1_1 = zero_to_(s150);
	elems.text26_0 = zero_to_(s160);
	elems.text26_1 = zero_to_(s161);
	elems.text26_2_0_0 = zero_to_(s162);

	// ОКТМО — оставляем как есть через to_string
	let s010 = &report.oktmo_qu_one;
	elems.text17_0 = s010.to_string();

	if let Some(o) = &report.oktmo_qu_two {
		elems.text17_1 = o.to_string();
	}

	if let Some(o) = &report.oktmo_qu_three {
		elems.text17_2 = o.to_string();
	}

	if let Some(o) = &report.oktmo_qu_four {
		elems.text17_4 = o.to_string();
	}

	// Раздел 1.1 — Авансовые платежи к уплате / уменьшению
	let s020 = if s130 - s140 > 0 { s130 - s140 } else { 0 };

	let s040 = if (s131 - s141) - s020 >= 0 { (s131 - s141) - s020 } else { 0 };

	let s050 = if (s131 - s141) - s020 < 0 { s020 - (s131 - s141) } else { 0 };

	let s070 = if (s132 - s142) - (s020 + s040 - s050) >= 0 { (s132 - s142) - (s020 + s040 - s050) } else { 0 };

	let s080 = if (s132 - s142) - (s020 + s040 - s050) < 0 { (s020 + s040 - s050) - (s132 - s142) } else { 0 };

	let s101 = report.patent_tax.unwrap_or(0);

	let s100 = if (s133 - s143) - (s020 + s040 - s050 + s070 - s080) - s101 >= 0 {
		(s133 - s143) - (s020 + s040 - s050 + s070 - s080) - s101
	} else { 0 };

	let s110 = if (s133 - s143) - (s020 + s040 - s050 + s070 - s080) - s101 < 0 {
		(s020 + s040 - s050 + s070 - s080) + s101 - (s133 - s143)
	} else { 0 };

	// Присваиваем значения результатов Раздела 1.1 с использованием zero_to_
	elems.text18_0 = zero_to_(s020);
	elems.text18_1 = zero_to_(s040);
	elems.text18_2 = zero_to_(s050);
	elems.text18_3 = zero_to_(s070);
	elems.text18_4 = zero_to_(s080);
	elems.text18_5 = zero_to_(s100);
	elems.text18_6_0 = zero_to_(s101);
	elems.text18_6_1 = zero_to_(s110);
}



fn zero_to_(val: i64) -> String {
	if val == 0 {
		"-".to_string()
	} else {
		val.to_string()
	}
}