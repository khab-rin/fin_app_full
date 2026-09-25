use std::collections::HashMap;
use serde_json::Value;
use lopdf::{Document, Object, Dictionary, StringFormat};
use serde::{Serialize, Deserialize};

use crate::{ProcessError, Status};
use crate::primitives::tax_frozen::implements::Tax;

use crate::primitives::tax_frozen::implements::Usn15;
use crate::service::auth_service::general::ActiveSession;
use crate::service::reports::fns_xsd_shemas::usn_1152017_decl::*;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Elems {
    #[serde(rename = "Text1")]
    pub text1: String, // инн
    
    #[serde(rename = "Text2")]
    pub text2: String, // кпп

    #[serde(rename = "Text3")]
    pub text3: String, // кор
    
    #[serde(rename = "Text4")]
    pub text4: String, // нп
    
    #[serde(rename = "Text5.0")]
    pub text5_0: String, // отчг
    
    #[serde(rename = "Text5.1")]
    pub text5_1: String, // код
    
    #[serde(rename = "Text6")]
    pub text6: String, // фнск
    
    #[serde(rename = "Text7.0")]
    pub text7_0: String, // назв1
    
    #[serde(rename = "Text7.1")]
    pub text7_1: String, // назв2
    
    #[serde(rename = "Text7.2")]
    pub text7_2: String, // назв3
    
    #[serde(rename = "Text7.3")]
    pub text7_3: String, // назв4
    
    #[serde(rename = "Text9")]
    pub text9: String, // +7913

    #[serde(rename = "Text10.0")]
    pub text10_0: String, // л
    
    #[serde(rename = "Text10.1")]
    pub text10_1: String, // т
    
    #[serde(rename = "Text11.0")]
    pub text11_0: String, // инн2
    
    #[serde(rename = "Text11.1")]
    pub text11_1: String, // кпп2
    
    #[serde(rename = "Text12.0")]
    pub text12_0: String, // фамил
    
    #[serde(rename = "Text12.1")]
    pub text12_1: String, // имя
    
    #[serde(rename = "Text12.2")]
    pub text12_2: String, // отч
    
    #[serde(rename = "Text12.3")]
    pub text12_3: String, // доверр
    
    #[serde(rename = "Text12.4")]
    pub text12_4: String, // доверр2
    
    #[serde(rename = "Text13.0")]
    pub text13_0: String, // лис
    
    #[serde(rename = "Text13.1")]
    pub text13_1: String, // стр
    
    #[serde(rename = "Text14")]
    pub text14: String, // н
    
    #[serde(rename = "Text15.0")]
    pub text15_0: String, // 22 (день)
    
    #[serde(rename = "Text15.1")]
    pub text15_1: String, // 01 (месяц)
    
    #[serde(rename = "Text15.2")]
    pub text15_2: String, // 2020 (год)

    #[serde(rename = "Text16")]
    pub text16: String, // ст2
    
    #[serde(rename = "Text17.0")]
    pub text17_0: String, // 010стр2
    
    #[serde(rename = "Text17.1")]
    pub text17_1: String, // 030стр2
    
    #[serde(rename = "Text17.2")]
    pub text17_2: String, // 060стр2
    
    #[serde(rename = "Text17.4")]
    pub text17_4: String, // 090стр2
    
    #[serde(rename = "Text18.0")]
    pub text18_0: String, // 020стр2
    
    #[serde(rename = "Text18.1")]
    pub text18_1: String, // 040стр2
    
    #[serde(rename = "Text18.2")]
    pub text18_2: String, // 050стр2
    
    #[serde(rename = "Text18.3")]
    pub text18_3: String, // 070стр2
    
    #[serde(rename = "Text18.4")]
    pub text18_4: String, // 080стр2
    
    #[serde(rename = "Text18.5")]
    pub text18_5: String, // 100стр2
    
    #[serde(rename = "Text18.6.0")]
    pub text18_6_0: String, // 101стр2
    
    #[serde(rename = "Text18.6.1")]
    pub text18_6_1: String, // 110стр2

    #[serde(rename = "Text19")]
    pub text19: String, // пустое поле
    
    #[serde(rename = "Text20")]
    pub text20: String, // ст3
    
    #[serde(rename = "Text21.0")]
    pub text21_0: String, // 010стр3
    
    #[serde(rename = "Text21.1")]
    pub text21_1: String, // 030стр3
    
    #[serde(rename = "Text21.2")]
    pub text21_2: String, // 060стр3
    
    #[serde(rename = "Text21.3")]
    pub text21_3: String, // 090стр3
    
    #[serde(rename = "Text22.0")]
    pub text22_0: String, // 020стр3
    
    #[serde(rename = "Text22.1")]
    pub text22_1: String, // 040стр3
    
    #[serde(rename = "Text22.2")]
    pub text22_2: String, // 050стр3
    
    #[serde(rename = "Text22.3")]
    pub text22_3: String, // 070стр3
    
    #[serde(rename = "Text22.4")]
    pub text22_4: String, // 080стр3
    
    #[serde(rename = "Text22.5")]
    pub text22_5: String, // 100стр3
    
    #[serde(rename = "Text22.6")]
    pub text22_6: String, // 101стр3
    
    #[serde(rename = "Text22.7.0")]
    pub text22_7_0: String, // 110стр3
    
    #[serde(rename = "Text22.7.1")]
    pub text22_7_1: String, // 120стр3

    #[serde(rename = "Text23")]
    pub text23: String, // ст4
    
    #[serde(rename = "Text24.0")]
    pub text24_0: String, // п
    
    #[serde(rename = "Text25.0")]
    pub text25_0: String, // 110стр4
    
    #[serde(rename = "Text25.1")]
    pub text25_1: String, // 11стр4
    
    #[serde(rename = "Text25.2")]
    pub text25_2: String, // 112стр4
    
    #[serde(rename = "Text25.3")]
    pub text25_3: String, // 113стр4
    
    #[serde(rename = "Text25.4")]
    pub text25_4: String, // 124 часть 2
    
    #[serde(rename = "Text25.5")]
    pub text25_5: String, // 130
    
    #[serde(rename = "Text25.6")]
    pub text25_6: String, // 131стр4
    
    #[serde(rename = "Text25.7")]
    pub text25_7: String, // 132стр4
    
    #[serde(rename = "Text25.8")]
    pub text25_8: String, // 133стр4
    
    #[serde(rename = "Text25.9")]
    pub text25_9: String, // 140стр5
    
    #[serde(rename = "Text25.10")]
    pub text25_10: String, // 141стр5
    
    #[serde(rename = "Text25.11.0")]
    pub text25_11_0: String, // 142стр5
    
    #[serde(rename = "Text25.11.1.0")]
    pub text25_11_1_0: String, // 143стр5
    
    #[serde(rename = "Text25.11.1.1")]
    pub text25_11_1_1: String, // 150стр5
    
    #[serde(rename = "Text34.0")]
    pub text34_0: String, // 6
    
    #[serde(rename = "Text34.1")]
    pub text34_1: String, // 6

	 #[serde(rename = "Text34.2")]
    pub text34_2: String, // 6
    
    #[serde(rename = "Text34.3")]
    pub text34_3: String, // 6
    
    #[serde(rename = "Text35.0")]
    pub text35_0: String, // 0
    
    #[serde(rename = "Text35.1")]
    pub text35_1: String, // 0
    
    #[serde(rename = "Text35.2")]
    pub text35_2: String, // 0
    
    #[serde(rename = "Text35.3")]
    pub text35_3: String, // 0
    
    #[serde(rename = "Text40")]
    pub text40: String, // 124
    
    #[serde(rename = "Text41")]
    pub text41: String, // ст5
    
    #[serde(rename = "Text26.0")]
    pub text26_0: String, // 160стр
    
    #[serde(rename = "Text26.1")]
    pub text26_1: String, // 161стр
    
    #[serde(rename = "Text26.2.0.0")]
    pub text26_2_0_0: String, // 162стр
    
    #[serde(rename = "Text26.2.1.0.0")]
    pub text26_2_1_0_0: String, // 300ст9
    
    #[serde(rename = "Text26.2.1.0.1")]
    pub text26_2_1_0_1: String, // 310ст9
    
    #[serde(rename = "Text26.2.1.0.2")]
    pub text26_2_1_0_2: String, // 320ст9

    #[serde(rename = "Text37")]
    pub text37: String, // ст6
    
    #[serde(rename = "Text39.0")]
    pub text39_0: String, // 110стр6
    
    #[serde(rename = "Text39.1")]
    pub text39_1: String, // 111стр6
    
    #[serde(rename = "Text39.2")]
    pub text39_2: String, // 112стр6
    
    #[serde(rename = "Text39.3")]
    pub text39_3: String, // 113стр6
    
    #[serde(rename = "Text39.4.0")]
    pub text39_4_0: String, // 130стр6
    
    #[serde(rename = "Text39.4.1")]
    pub text39_4_1: String, // 14стр6
    
    #[serde(rename = "Text39.5.0")]
    pub text39_5_0: String, // 131стр6
    
    #[serde(rename = "Text39.5.1")]
    pub text39_5_1: String, // 141стр6
    
    #[serde(rename = "Text39.6.0")]
    pub text39_6_0: String, // 132стр6
    
    #[serde(rename = "Text39.6.1")]
    pub text39_6_1: String, // 142стр6
    
    #[serde(rename = "Text39.7.0")]
    pub text39_7_0: String, // 133стр6
    
    #[serde(rename = "Text39.7.1")]
    pub text39_7_1: String, // 143стр6

    #[serde(rename = "Text36")]
    pub text36: String, // ст7
    
    #[serde(rename = "Text38.0")]
    pub text38_0: String, // 150стр7
    
    #[serde(rename = "Text38.1")]
    pub text38_1: String, // 151стр7
    
    #[serde(rename = "Text38.2")]
    pub text38_2: String, // 152стр7
    
    #[serde(rename = "Text38.3")]
    pub text38_3: String, // 153стр7
    
    #[serde(rename = "Text38.4")]
    pub text38_4: String, // 160стр7
    
    #[serde(rename = "Text38.5")]
    pub text38_5: String, // 161стр7
    
    #[serde(rename = "Text38.6")]
    pub text38_6: String, // 162стр7
    
    #[serde(rename = "Text38.7")]
    pub text38_7: String, // 163стр7

    #[serde(rename = "Text42")]
    pub text42: String, // ст8
    
    #[serde(rename = "Text27.0")]
    pub text27_0: String, // 210стр8
    
    #[serde(rename = "Text27.1")]
    pub text27_1: String, // 211стр8

	 #[serde(rename = "Text27.2")]
    pub text27_2: String, // 212стр8
    
    #[serde(rename = "Text27.3")]
    pub text27_3: String, // 213стр8
    
    #[serde(rename = "Text27.4")]
    pub text27_4: String, // 220стр8
    
    #[serde(rename = "Text27.5")]
    pub text27_5: String, // 221стр8
    
    #[serde(rename = "Text27.6")]
    pub text27_6: String, // 222стр8
    
    #[serde(rename = "Text27.7")]
    pub text27_7: String, // 223стр8
    
    #[serde(rename = "Text27.8")]
    pub text27_8: String, // 230стр8
    
    #[serde(rename = "Text27.9")]
    pub text27_9: String, // 240стр8
    
    #[serde(rename = "Text27.10")]
    pub text27_10: String, // 241стр8
    
    #[serde(rename = "Text27.11")]
    pub text27_11: String, // 242стр8
    
    #[serde(rename = "Text27.12")]
    pub text27_12: String, // 243стр8
    
    #[serde(rename = "Text27.13")]
    pub text27_13: String, // 250стр8
    
    #[serde(rename = "Text27.14")]
    pub text27_14: String, // 251стр8
    
    #[serde(rename = "Text27.15")]
    pub text27_15: String, // 252стр8
    
    #[serde(rename = "Text27.16")]
    pub text27_16: String, // 253стр8

    #[serde(rename = "Text27.17")]
    pub text27_17: String, // 270ст9
    
    #[serde(rename = "Text27.18")]
    pub text27_18: String, // 271ст9
    
    #[serde(rename = "Text27.19")]
    pub text27_19: String, // 272ст9
    
    #[serde(rename = "Text27.20")]
    pub text27_20: String, // 273ст9
    
    #[serde(rename = "Text27.21.0")]
    pub text27_21_0: String, // 280ст9
    
    #[serde(rename = "Text27.21.1")]
    pub text27_21_1: String, // 290ст9
    
    #[serde(rename = "Text28.0.0")]
    pub text28_0_0: String, // 15
    
    #[serde(rename = "Text28.0.1")]
    pub text28_0_1: String, // 15
    
    #[serde(rename = "Text28.0.2")]
    pub text28_0_2: String, // 15
    
    #[serde(rename = "Text28.0.3")]
    pub text28_0_3: String, // 15
    
    #[serde(rename = "Text28.1.0")]
    pub text28_1_0: String, // 0
    
    #[serde(rename = "Text28.1.1")]
    pub text28_1_1: String, // 0
    
    #[serde(rename = "Text28.1.2")]
    pub text28_1_2: String, // 0
    
    #[serde(rename = "Text28.1.3")]
    pub text28_1_3: String, // 0
    
    #[serde(rename = "Text45")]
    pub text45: String, // 264
    
    #[serde(rename = "Text46")]
    pub text46: String, // 264 посл.точ
    
    #[serde(rename = "Text47")]
    pub text47: String, // ст9
}


pub fn make_decl_usn_pdf(
	session: &ActiveSession,
	decl: &UsnDeclUsnFile
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

	fill_usn_decl_pdf(&elems)
}

pub fn fill_fifteen_usn_elems(
	report: &UsnDeclTaxFifteen,
	elems: &mut Elems
) {
	let s_t_20 = 2;
	let s_t_42 = 3;
	let s_t_47 = 4;

	elems.text20 = s_t_20.to_string();
	elems.text42 = s_t_42.to_string();
	elems.text47 = s_t_47.to_string();



	let s210 = report.calculation.income.first_qu.unwrap_or(0);
	elems.text27_0 = s210.to_string();
	let s211 = report.calculation.income.second_qu.unwrap_or(0);
	elems.text27_1 = s211.to_string();
	let s212 = report.calculation.income.third_qu.unwrap_or(0);
	elems.text27_2 = s212.to_string();
	let s213 = report.calculation.income.fourth_qu;
	elems.text27_3 = s213.to_string();

	let s220 = report.calculation.expenses.first_qu.unwrap_or(0);
	elems.text27_4 = s220.to_string();
	let s221 = report.calculation.expenses.second_qu.unwrap_or(0);
	elems.text27_5 = s221.to_string();
	let s222 = report.calculation.expenses.third_qu.unwrap_or(0);
	elems.text27_6 = s222.to_string();
	let s223 = report.calculation.expenses.fourth_qu;
	elems.text27_7 = s223.to_string();

	let s230 = report.calculation.prev_negative_taxable_base.unwrap_or(0);
	elems.text27_8 = s230.to_string();

	let s240 = (s210 - s220).max(0);
	elems.text27_9 = s240.to_string();

	let s241 = (s211 - s221).max(0);
	elems.text27_10 = s241.to_string();

	let s242 = (s212 - s222).max(0);
	elems.text27_11 = s242.to_string();

	let s243 = (s213 - s223).max(0);
	elems.text27_12 = s243.to_string();

	let s250 = (s220 - s210).max(0);
	elems.text27_13 = s250.to_string();

	let s251 = (s221 - s211).max(0);
	elems.text27_14 = s251.to_string();

	let s252 = (s222 - s212).max(0);
	elems.text27_15 = s252.to_string();

	let s253 = (s223 - s213).max(0);
	elems.text27_16 = s253.to_string();

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
	elems.text27_17 = s270.to_string();
	elems.text27_18 = s271.to_string();
	elems.text27_19 = s272.to_string();
	elems.text27_20 = s273.to_string();


	let s280 = report.calculation.min_tax;
	elems.text27_21_0 = s280.to_string();

	if let Some(social) = &report.calculation.required_social {
		let s290 = social.fix_amnt;
		elems.text27_21_1 = s290.to_string();

		let s300 = social.one_perc;
		elems.text26_2_1_0_0 = s300.to_string();

		let s310 = social.one_perc_curr_year;
		elems.text26_2_1_0_1 = s310.to_string();

		let s320 = social.one_perc_prev_year;
		elems.text26_2_1_0_2 = s320.to_string();
	}

	elems.text17_0 = report.oktmo_qu_one.to_string();

	if let Some(o) = &report.oktmo_qu_two {
		elems.text17_1 = o.to_string();
	}

	if let Some(o) = &report.oktmo_qu_three {
		elems.text17_2 = o.to_string();
	}

	if let Some(o) = &report.oktmo_qu_four {
		elems.text17_4 = o.to_string();
	}

	let s020 = s270;
	elems.text22_0 = s020.to_string();

	let s040 = (s271 - s020).max(0);
	elems.text22_1 = s040.to_string();

	let s050 = (s020 - s271).max(0);
	elems.text22_2 = s050.to_string();

	let s070 = (s272 - (s020 + s040 - s050)).max(0);
	elems.text22_3 = s070.to_string();

	let s080 = ((s020 + s040 - s050) - s272).max(0);
	elems.text22_4 = s080.to_string();

	let s100 = (s273 - (s020 + s040 - s050 - s070 - s080)).max(0);
	elems.text22_5 = s100.to_string();

	let s101 = report.patent_tax.unwrap_or(0);
	elems.text22_6 = s101.to_string();

	let s110 = if s273 >= s280 {
		if s273 - s272 < 0 { (s272 - s273).max(0) } else { 0 }
	} else {
		if s272 > s280 { (s272 - s280).max(0) } else { 0 }
	};

	elems.text22_7_0 = s110.to_string();

	let s120 = if s280 > s273 && s280 > ((s020 + s040 - s050 + s070 - s080) + s101) {
		(s280 - (s020 + s040 - s050 + s070 - s080) - s101).max(0)
	} else {
		0
	};

	elems.text22_7_1 = s120.to_string();

}




pub fn fill_six_usn_elems(
	report: &UsnDeclTaxSix,
	elems: &mut Elems
) {

	let s_t_16 = 2;
	let s_t_23 = 3;
	let s_t_41 = 4;

	elems.text16 = s_t_16.to_string();
	elems.text23 = s_t_23.to_string();
	elems.text41 = s_t_41.to_string();


	let s102 = report.calculation.employers_exist;
	elems.text24_0 = s102.to_string();

	let s110 = report.calculation.taxable_income.first_qu.unwrap_or(0);
	elems.text25_0 = s110.to_string();

	let s111 = report.calculation.taxable_income.second_qu.unwrap_or(0);
	elems.text25_1 = s111.to_string();

	let s112 = report.calculation.taxable_income.third_qu.unwrap_or(0);
	elems.text25_2 = s112.to_string();

	let s113 = report.calculation.taxable_income.fourth_qu;
	elems.text25_3 = s113.to_string();

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

	let s130 = report.calculation.calc_tax.first_qu.unwrap_or(0);
	let s131 = report.calculation.calc_tax.second_qu.unwrap_or(0);
	let s132 = report.calculation.calc_tax.third_qu.unwrap_or(0);
	let s133 = report.calculation.calc_tax.fourth_qu;

	elems.text25_5 = s130.to_string();
	elems.text25_6 = s131.to_string();
	elems.text25_7 = s132.to_string();
	elems.text25_8 = s133.to_string();

	let s140 = report.calculation.tot_social.first_qu.unwrap_or(0).min(s130);
	let s141 = report.calculation.tot_social.second_qu.unwrap_or(0).min(s131);
	let s142 = report.calculation.tot_social.third_qu.unwrap_or(0).min(s132);
	let s143 = report.calculation.tot_social.fourth_qu.min(s133);

	elems.text25_9 = s140.to_string();
	elems.text25_10 = s140.to_string();
	elems.text25_11_0 = s140.to_string();
	elems.text25_11_1_0 = s140.to_string();

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

	elems.text25_11_1_1 = s150.to_string();
	elems.text26_0 = s160.to_string();
	elems.text26_1 = s161.to_string();
	elems.text26_2_0_0 = s162.to_string();

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

	let s020 = if s130 - s140 > 0 {
		s130 - s140
	} else { 0 };

	let s040 = if (s131 - s141) - s020 >= 0 {
		(s131 - s141) - s020
	} else { 0 };

	let s050 = if (s131 - s141) - s020 < 0 {
		s020 - (s131 - s141)
	} else { 0 };

	let s070 = if (s132 - s142) - (s020 + s040 - s050) >= 0 {
		(s132 - s142) - (s020 + s040 - s050)
	} else { 0 };

	let s080 = if (s132 - s142) - (s020 + s040 - s050) < 0 {
		(s020 + s040 - s050) - (s132 - s142)
	} else { 0 };


	let s101 = report.patent_tax.unwrap_or(0);

	let s100 = if (s133 - s143) - (s020 + s040 - s050 + s070 - s080) - s101 >= 0 {
		(s133 - s143) - (s020 + s040 - s050 + s070 - s080) - s101
	} else { 0 };

	let s110 = if (s133 - s143) - (s020 + s040 - s050 + s070 - s080) - s101 < 0 {
		(s020 + s040 - s050 + s070 - s080) + s101 - (s133 - s143)
	} else { 0 };

	elems.text18_0 = s020.to_string();
	elems.text18_1 = s040.to_string();
	elems.text18_2 = s050.to_string();
	elems.text18_3 = s070.to_string();
	elems.text18_4 = s080.to_string();
	elems.text18_5 = s100.to_string();
	elems.text18_6_0 = s101.to_string();
	elems.text18_6_1 = s110.to_string();

}



pub fn fill_usn_decl_pdf(
	elems: &Elems
) -> Result<Vec<u8>, Status> {
	// 1. Загрузка шаблона PDF из ресурсов
	let pdf_tpl_bytes = include_bytes!("../../../../../../resourses/decl_usn_regul.PDF");
	let mut doc = lopdf::Document::load_mem(pdf_tpl_bytes)
		.map_err(|err| err.process_err(Status::FileReadError, "Ошибка загрузки шаблона PDF"))?;

	// 2. Конвертируем структуру Elems в JSON-значение
	let json_value = serde_json::to_value(elems)
		.map_err(|_| Status::Tech.process_err(Status::MappingError, "Ошибка сериализации Elems в JSON"))?;

	// 3. Превращаем JSON-объект в HashMap<String, String>, как требует движок
	let mut data_map = HashMap::new();
	if let Value::Object(map) = json_value {
		for (key, val) in map {
			if let Value::String(s) = val {
				data_map.insert(key, s);
			} else if !val.is_null() {
				// Обработка чисел или булевых значений, если они появятся в структуре
				data_map.insert(key, val.to_string());
			}
		}
	} else {
		return Err(Status::Tech.process_err(Status::MappingError, "Структура Elems не является JSON-объектом"));
	}

	// 4. Запускаем универсальный процесс заполнения через дерево AcroForm
	fill_pdf_form(&mut doc, &data_map)
		.map_err(|_| Status::Tech.process_err(Status::FileWriteError, "Ошибка модификации полей PDF"))?;

	// 5. Сохраняем обновленный документ в байтовый вектор
	let mut output_bytes = Vec::new();
	doc.save_to(&mut output_bytes)
		.map_err(|_| Status::Tech.process_err(Status::FileWriteError, "Ошибка сохранения итогового PDF"))?;

	Ok(output_bytes)
}


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

fn encode_pdf_string(val: &str) -> Vec<u8> {
    if val.is_ascii() {
        return val.as_bytes().to_vec();
    }
    
    // Для кириллицы (ФИО) кодируем в UTF-16BE с маркером порядка байт BOM (0xFE 0xFF)
    let mut encoded = vec![0xFE, 0xFF];
    for ch in val.encode_utf16() {
        encoded.push((ch >> 8) as u8);
        encoded.push((ch & 0xFF) as u8);
    }
    encoded
}

/// Рекурсивная функция обхода и модификации дерева интерактивных полей
fn traverse_and_fill_field(
    doc: &mut Document,
    field_ref: lopdf::ObjectId,
    parent_name: String,
    data: &HashMap<String, String>,
) -> Result<(), lopdf::Error> {
    // Получаем актуальный словарь объекта напрямую из документа
    let mut dict = doc.get_object(field_ref)?.as_dict()?.clone();

    // Шаг 1. Формируем имя текущего узла и полный составной путь
    let mut node_name = String::new();
    let mut current_name = parent_name.clone();
    
    if let Ok(t_obj) = dict.get(b"T") {
        if let Ok(name_bytes) = t_obj.as_str() {
            if let Ok(name_str) = String::from_utf8(name_bytes.to_vec()) {
                node_name = name_str.clone();
                if !current_name.is_empty() {
                    current_name.push('.');
                }
                current_name.push_str(&name_str);
            }
        }
    }

    // Шаг 2. Ищем значение (сначала по полному пути, затем по короткому имени узла)
    let mut value_to_set = data.get(&current_name).cloned();
    if value_to_set.is_none() && !node_name.is_empty() {
        value_to_set = data.get(&node_name).cloned();
    }

    // Если значение найдено, записываем его в текущий узел
    if let Some(ref val) = value_to_set {
        let pdf_string = Object::String(encode_pdf_string(val), StringFormat::Literal);
        dict.set(b"V", pdf_string);
        dict.remove(b"AP"); 
        doc.set_object(field_ref, Object::Dictionary(dict.clone()));
    }

    // Шаг 3. Идем вглубь дерева /Kids
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
            // КРИТИЧЕСКИЙ МОМЕНТ: Если у родителя (например, Text2) было найдено значение,
            // но у дочернего элемента нет своего имени /T, мы принудительно передаем 
            // значение родителя вниз, чтобы заполнились виджеты на всех страницах.
            let kid_dict = doc.get_object(kid_ref)?.as_dict()?;
            
            if kid_dict.get(b"T").is_err() && value_to_set.is_some() {
                // У дочернего виджета нет своего имени, значит это отображение родительского поля
                let mut updated_kid_dict = kid_dict.clone();
                let val = value_to_set.as_ref().unwrap();
                let pdf_string = Object::String(encode_pdf_string(val), StringFormat::Literal);
                
                updated_kid_dict.set(b"V", pdf_string);
                updated_kid_dict.remove(b"AP");
                doc.set_object(kid_ref, Object::Dictionary(updated_kid_dict));
            }

            // В любом случае продолжаем стандартную рекурсию, чтобы не пропустить уникальные поля вроде Text12.0
            traverse_and_fill_field(doc, kid_ref, current_name.clone(), data)?;
        }
    }

    Ok(())
}