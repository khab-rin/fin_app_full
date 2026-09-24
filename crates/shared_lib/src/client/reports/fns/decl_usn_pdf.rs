
use serde::{Serialize, Deserialize};
use crate::primitives::tax_frozen::implements::Tax;

use crate::Status;
use crate::primitives::tax_frozen::implements::Usn15;
use crate::service::auth_service::general::ActiveSession;
use crate::service::reports::fns_xsd_shemas::usn_1152017_decl::*;
use crate::service::reports::fns_xsd_shemas::common::*;

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

	let mut pages_count = 1;

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
			fill_fifteen_usn_pdf(report, &mut elems);
		},
		UsnDeclTaxReportType::SixPercent(report) => {
			fill_six_usn_pdf(report, &mut elems)
		}
	}


	Err(Status::Unknown)
}

pub fn fill_fifteen_usn_pdf(
	report: &UsnDeclTaxFifteen,
	elems: &mut Elems
) {
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

pub fn fill_six_usn_pdf(
	report: &UsnDeclTaxSix,
	elems: &mut Elems
) {

}