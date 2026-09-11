use crate::primitives::frozen::text::{Date, Kpp};
use crate::{ClientState, ProcessError, Status};
use crate::service::reports::service::ReportStep;
use crate::service::reports::fns_xsd_shemas::usn_1110355_notif::*;
use crate::primitives::frozen::text_base::{Digits4_4, String1_40};
use crate::primitives::tax_frozen::implements::Usn6;

use crate::client::reports::fns::helper::make_quaters;
use crate::client::sql_queries::operations::get::reports::fns::usn_notif_6::get_quater_cummul_incomes_usn_notif_6;
use crate::client::reports::fns::helper::make_file_id;

pub async fn make_notif_usn_6_files(
	state: &ClientState,
	year: i32,
	qu: i32,
	fns_branch: Digits4_4
) -> Result<ReportStep, Status> {

	let session = state.get_session().await
		.map_err(|err| err.process_err(err, ""))?;

	let quart_dates = make_quaters(year)
		.map_err(|err| err.process_err(err, ""))?;
	

	let quat_amonts = get_quater_cummul_incomes_usn_notif_6(
		state,
		&quart_dates
	).await.map_err(|err| err.process_err(err, ""))?;


	let kpp: Option<Kpp> = match session.session_user.company.comp_inn.len() {
		12 => None,
		_ => Some(session.session_user.company.kpp.clone())
	};

	let oktmo = session
		.session_user
		.company
		.metadata
		.oktmo_company
		.clone()
		.ok_or(Status::Tech.process_err(Status::SystemLogicErr, ""))?;

	let kbk = UsnNotifKbk::UsnNotifSix;

	let not_year = Digits4_4::new(year.to_string().as_str())
		.map_err(|err| err.process_err(Status::SystemLogicErr, ""))?;

	let notification = match qu {
		1 => UsnNotifNotification { 
			kpp, 
			oktmo, 
			kbk, 
			avans_amnt: quat_amonts.q1 * Usn6::default(), 
			period: UsnNotifPeriod::FirstQuarter, 
			qu_month_num: UsnNotifPeriodNum::QuOne, 
			year: not_year 
		},
		 
		2 => UsnNotifNotification { 
			kpp, 
			oktmo, 
			kbk, 
			avans_amnt: quat_amonts.q2 * Usn6::default() - quat_amonts.q1 * Usn6::default(), 
			period: UsnNotifPeriod::HalfYear, 
			qu_month_num: UsnNotifPeriodNum::QuTwo, 
			year: not_year 
		},

		3 => UsnNotifNotification { 
			kpp, 
			oktmo, 
			kbk, 
			avans_amnt: quat_amonts.q3 * Usn6::default() - quat_amonts.q2 * Usn6::default(), 
			period: UsnNotifPeriod::NineMonths, 
			qu_month_num: UsnNotifPeriodNum::QuThree, 
			year: not_year 
		},
		4 => UsnNotifNotification { 
			kpp, 
			oktmo, 
			kbk, 
			avans_amnt: quat_amonts.q4 * Usn6::default() - quat_amonts.q3 * Usn6::default(),
			period: UsnNotifPeriod::Year, 
			qu_month_num: UsnNotifPeriodNum::QuFour, 
			year: not_year 
		},
		_ => {
			return Err(Status::Tech.process_err(Status::SystemLogicErr, ""));
		}

	};

	let notifications: vec1::Vec1<UsnNotifNotification> = vec1::Vec1::new(notification);

	let tax_payer = match session.session_user.company.comp_inn.len() {
		12 => UsnNotifTaxPayerChoice::Person(
			UsnNotifTaxPayerPerson {
				pers_inn: session.session_user.company.comp_inn.clone()
			}
		)
		,
		_ => UsnNotifTaxPayerChoice::Company(
			UsnNotifTaxPayerCompany {
			comp_inn: session.session_user.company.comp_inn.clone(),
			kpp: session.session_user.company.kpp.clone(),
		})
	};

	let signer = UsnNotifSigner {
		signer_type: UsnNotifSignerType::TAXPAYER,
		delegate_info: None,
		fio: session.session_user.person.metadata.fio.clone()
	};

	let document = UsnNotifDocument {
		knd: UsnNotifUsnKnd::Value,
		doc_date: Date::unchecked(chrono::Utc::now().date_naive()),
		branch_code: fns_branch.clone(),
		tax_payer,
		signer,
		notifications
	};

	let file_id = make_file_id(&session, &fns_branch)
		.map_err(|err| err.process_err(err, ""))?;

	let version_str = format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

	let program_version = String1_40::unchecked(version_str);

	let notif_file = UsnNotifFile {
		file_id,
		program_version,
		format_version: UsnNotifUsnFormat::Value,
		document
	};

	let mut xml_string = String::with_capacity(4096);
	
	// 2. Сериализуем структуру с корневым тегом ФНС в строку
	quick_xml::se::to_writer_with_root(&mut xml_string, "Файл", &notif_file)
		.map_err(|err| err.process_err(Status::FileWriteError, ""))?;

	// 3. Собираем финальный байтовый вектор (добавляя xml-заголовок)
	let mut xml_file: Vec<u8> = Vec::with_capacity(xml_string.len() + 50);
	xml_file.extend_from_slice(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
	xml_file.extend_from_slice(xml_string.as_bytes());



	Err(Status::Unknown)
}