use crate::Status;

use crate::service::auth_service::general::ActiveSession;
use crate::primitives::frozen::text::MidName;
use crate::service::reports::fns_xsd_shemas::usn_1110355_notif::UsnNotifFile;

use crate::client::reports::fns::usn_decl_notif_elems::Elems;
use crate::client::reports::fns::pdf_fill::custom_pdf_fill;

pub fn fill_usn_notif_elems_make_pdf(
	session: &ActiveSession,
	notif: &UsnNotifFile,
	pdf_tpl_bytes: &[u8]
) -> Result<Vec<u8>, Status> {

	let mut elems = Elems::default();

	elems.text1 = session.session_user.company.comp_inn.to_string();
	elems.text2 = session.session_user.company.kpp.to_string();
	if &elems.text2 == "0" || elems.text2.is_empty() {
		elems.text2 = "-".to_string();
	}

	elems.text3 = notif.document.branch_code.to_string();
	elems.text4 = "2".to_string();
	elems.text5 = "0".to_string();

	elems.text6 = notif.document.signer.signer_type.to_string();

	elems.text7 = "2".to_string();

	let fio = session.session_user.person.metadata.fio.clone();
	elems.text8_0 = fio.sur_name.to_string();
	elems.text8_1 = fio.first_name.to_string();
	elems.text8_2 = fio.mid_name.unwrap_or(MidName::unchecked("")).to_string();

	let notif_date = notif.document.doc_date.to_string();

	let y_m_d: Vec<&str> = notif_date.split('-').collect();

	elems.text9_0 = y_m_d[2].to_string();
	elems.text9_1 = y_m_d[1].to_string();
	elems.text10 = y_m_d[0].to_string();

	if let Some(delegate) = &notif.document.signer.delegate_info {
		elems.text11_0 = delegate.delegate_doc_info.to_string();
	}

	elems.text7 = "2".to_string();

	elems.text12_0 = elems.text2.clone();

	let notification = notif.document.notifications.first();

	elems.text13_0_0 = notification.oktmo.to_string();
	elems.text14_0_0 = notification.kbk.to_string();

	let avans_amnt = notification.avans_amnt.to_string();
	let (rub, kop) = match avans_amnt.split_once('.') {
        Some((r, k)) => (r.to_string(), format!("{:0<2}", &k[..std::cmp::min(k.len(), 2)])),
        None => (avans_amnt.clone(), "00".to_string()),
    };

	elems.text15_0_0 = rub.to_string();
	elems.text16_0_0_0 = kop.to_string();

	elems.text16_0_1_0 = notification.period.to_string();
	elems.text16_0_2_0 = notification.qu_month_num.to_string();

	elems.text17_0 = notification.year.to_string();
	

	custom_pdf_fill(&elems, pdf_tpl_bytes)
}

