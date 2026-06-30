use shared_lib::Status;
use shared_lib::primitives::frozen::implements_base::String1_255;
use shared_lib::service::auth_service::client_state::ActiveSession;
use shared_lib::service::mchd::service::{MchdInfo, MchdStep, NewMchdData};
use shared_lib::service::mchd::poa::PoaMchd;

pub(crate) fn make_mchd_step_tax_success(
    session: &ActiveSession,
    poa_mchd: &PoaMchd,
    data: &NewMchdData
) -> Result<MchdStep, Status> {
    let failed_res = MchdStep::TryLater { text: MchdInfo::ClientServiceError };

    let doc_name_str = format!("{}.doc", poa_mchd.flie_identificator.as_ref());
    let xml_name_str = format!("{}.xml", poa_mchd.flie_identificator.as_ref());
    let doc_name = String1_255::unchecked(doc_name_str);
    let xml_name = String1_255::unchecked(xml_name_str);

    let xml_string = match quick_xml::se::to_string(&poa_mchd) {
        Ok(xml) => xml,
        Err(err) => {
            log::error!(
                "FUN make_mchd_step_tax_success FAILED BY MchdStep MAPPING, tech_err = {}, local_err = {}",
                err, Status::MappingError
            );
            return Ok(failed_res);
        }
    };

    let xml_file = xml_string.into_bytes();

    let NewMchdData { 
        poa_number, 
        poa_end_date, 
        manager_tittle, 
        manager_sur_name, 
        manager_first_name, 
        manager_mid_name, 
        manager_snils, 
        manager_inn, 
        user_sur_name, 
        user_first_name, 
        user_mid_name, 
        user_snils, 
        user_inn, 
        user_passport_number, 
        powers ,
        ..
    } = data;


    let manager_full_name = format!("{} {} {}", manager_sur_name, manager_first_name, manager_mid_name);

    // Сборка ФИО Представителя (пользователя)
    let user_full_name = format!("{} {} {}", user_sur_name, user_first_name, user_mid_name);

    let comp_name_data = match session.session_user.company.metadata.comp_name.clone() {
        Some(d) => d,
        None => {
            log::error!(
                "FUN make_mchd_step_tax_success FAILED BY MISS session.session_user.company.metadata.comp_name, err = {}",
                Status::DadataMissFields
            );
            return Err(Status::DadataMissFields);
        }
    };

    let comp_name = match comp_name_data.full_egrul_name {
        Some(n) => n,
        None => {
            log::error!(
                "FUN make_mchd_step_tax_success FAILED BY MISS session.session_user.company.metadata.comp_name.full_egrul_name, err = {}",
                Status::DadataMissFields
            );
            return Err(Status::DadataMissFields);
        }
    };

    let comp_inn = session.session_user.company.comp_inn.as_ref();
    let kpp = session.session_user.company.kpp.as_ref();

    // Рендеринг списка полномочий (Powers) в HTML-строку
    let mut powers_html = String::new();

    for power in powers.iter() {
        let info = power.get_power_info();
        powers_html.push_str(&format!(
            "<li><b>{}</b> — {}</li>",
            info.code, info.name
        ));
    }
    if powers_html.is_empty() {
        powers_html = "<li>Полномочия не указаны</li>".to_string();
    }

    // --- 3. Генерация HTML/DOC шаблона ---
    let html_text = format!(
        r#"<html xmlns:o="urn:schemas-microsoft-com:office:office" xmlns:w="urn:schemas-microsoft-com:office:word" xmlns="http://www.w3.org/TR/REC-html40">
        <head>
            <meta charset="utf-8">
            <style>
                body {{ 
                    font-family: 'Times New Roman', Times, serif; 
                    font-size: 12pt; 
                    line-height: 1.5; 
                    color: #000000;
                }}
                .title {{ 
                    text-align: center; 
                    font-weight: bold; 
                    font-size: 14pt; 
                    margin-bottom: 30px; 
                }}
                .table-info {{
                    width: 100%;
                    margin-bottom: 20px;
                    border-collapse: collapse;
                }}
                .table-info td {{
                    padding: 4px;
                    vertical-align: top;
                }}
                .section-title {{
                    font-weight: bold;
                    margin-top: 20px;
                    margin-bottom: 10px;
                    text-decoration: underline;
                }}
                .content {{ 
                    text-align: justify; 
                    text-indent: 1.25cm; 
                    margin-bottom: 15px;
                }}
                .powers-list {{
                    margin-bottom: 20px;
                    padding-left: 20px;
                }}
                .powers-list li {{
                    margin-bottom: 8px;
                    text-align: justify;
                }}
                .footer-table {{
                    margin-top: 50px;
                    width: 100%;
                }}
            </style>
        </head>
        <body>
            <div class="title">СВЕДЕНИЯ О ДОВЕРЕННОСТИ В ЭЛЕКТРОННОЙ ФОРМЕ (МЧД)<br>Внутренний № {poa_number}</div>
            
            <table class="table-info">
                <tr>
                    <td style="width: 25%; font-weight: bold;">Доверитель (Организация):</td>
                    <td>ООО «{comp_name}», ИНН: {comp_inn}, КПП: {kpp}</td>
                </tr>
                <tr>
                    <td style="font-weight: bold;">В лице руководителя:</td>
                    <td>{manager_tittle} — {manager_full_name}, ИНН: {manager_inn}, СНИЛС: {manager_snils}</td>
                </tr>
                <tr>
                    <td style="font-weight: bold;">Представитель (Доверенное лицо):</td>
                    <td>{user_full_name}, ИНН: {user_inn}, СНИЛС: {user_snils}, Паспорт: {user_passport_number}</td>
                </tr>
                <tr>
                    <td style="font-weight: bold;">Срок действия до:</td>
                    <td>{poa_end_date}</td>
                </tr>
            </table>

            <div class="section-title">Перечень переданных полномочий:</div>
            <ul class="powers-list">
                {powers_html}
            </ul>

            <div class="content">
                Настоящая машиночитаемая доверенность (МЧД) сформирована в формате XML, соответствующем требованиям ФНС России, подписана усиленной квалифицированной электронной подписью (УКЭП) генерального директора и подготовлена для отправки в государственные органы.
            </div>
            
            <table class="footer-table">
                <tr>
                    <td style="width: 40%; font-size: 10pt; color: #555555;">Идентификатор файла (GUID):</td>
                    <td style="text-align: right; font-family: monospace; font-size: 10pt; color: #555555;">{guid}</td>
                </tr>
                <tr>
                    <td style="padding-top: 30px; font-style: italic; color: #666666; font-size: 11pt;" colspan="2">
                        Документ сформирован автоматически в системе fin_app на основе XML-структуры МЧД.
                    </td>
                </tr>
            </table>
        </body>
        </html>"#,
        poa_number = poa_number.as_ref(),
        comp_name = comp_name,
        comp_inn = comp_inn,
        kpp = kpp,
        manager_tittle = manager_tittle.as_ref(),
        manager_full_name = manager_full_name,
        manager_inn = manager_inn.as_ref(),
        manager_snils = manager_snils.as_ref(),
        user_full_name = user_full_name,
        user_inn = user_inn.as_ref(),
        user_snils = user_snils.as_ref(),
        user_passport_number = user_passport_number.as_ref(),
        poa_end_date = poa_end_date.as_ref(),
        powers_html = powers_html,
        guid = poa_mchd.flie_identificator.as_ref()
    );

    let doc_file = html_text.into_bytes();


    let success = MchdStep::Success { 
        doc_name, 
        doc_file, 
        xml_name, 
        xml_file, 
        text: MchdInfo::Success
    };

    Ok(success)
}