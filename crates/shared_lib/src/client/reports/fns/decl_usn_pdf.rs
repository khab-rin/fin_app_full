use quick_xml::de;

use crate::service::reports::fns_xsd_shemas::common::FnsKbk;
use crate::{Status, ProcessError};
use crate::service::auth_service::general::ActiveSession;
use crate::service::reports::fns_xsd_shemas::usn_1152017_decl::*;
use crate::primitives::frozen::text::{MidName};
use crate::service::reports::fns_xsd_shemas::common::*;

pub fn make_decl_usn_pdf(
	session: &ActiveSession,
	decl: &UsnDeclUsnFile
) -> Result<Vec<u8>, Status> {

	let inn = session.session_user.company.comp_inn.to_string();
    let kpp = session.session_user.company.kpp.to_string();
    let fio = session.session_user.person.metadata.fio.clone();
    let sur_name = fio.sur_name.to_string();
    let first_name = fio.first_name.to_string();
    let mid_name = fio.mid_name.unwrap_or(MidName::unchecked("")).to_string();

    let fns_branch = decl.document.branch_code.clone();

    let (oktmo, kbk) = match &decl.document.usn_report.report_type {
		UsnDeclTaxReportType::SixPercent(report) =>(report.oktmo_qu_one.clone(), FnsKbk::UsnSix),
		UsnDeclTaxReportType::FifteenPercent(report) => (report.oktmo_qu_one.clone(), FnsKbk::UsnFifteen)
	};

	let year = decl.document.report_year.clone();
	let period = FnsPeriod::Year;

	let version = decl.document.report_version;

	let submission_code = &decl.document.submission_place.clone();

	let tel_num = &decl.document.tax_payer.tel.clone();

	let usn_object = &decl.document.usn_report.usn_object.clone();








	Err(Status::Unknown)
}