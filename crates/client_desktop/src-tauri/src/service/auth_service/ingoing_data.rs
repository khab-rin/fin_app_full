use shared_lib::Status;
use shared_lib::service::auth_service::implements::IngoingData;

use crate::state::ClientState;

pub(crate) async  fn make_ingoing_doc(
    state: &ClientState,
    data: &IngoingData
) -> Result<Vec<u8>, Status> {

    let IngoingData { 
        sur_name, 
        first_name, 
        mid_name, 
        pers_inn, 
        snils, 
        comp_inn, 
        kpp, 
        phone, 
        email 
    } = data;

 let full_name = match mid_name {
        Some(mid) if !mid.as_ref().trim().is_empty() => {
            format!("{} {} {}", sur_name.as_ref(), first_name.as_ref(), mid.as_ref())
        }
        _ => format!("{} {}", sur_name.as_ref(), first_name.as_ref()),
    };

    let html_text = format!(
        r#"<html xmlns:o="urn:schemas-microsoft-com:office:office" xmlns:w="urn:schemas-microsoft-com:office:word" xmlns="http://www.w3.org/TR/REC-html40">
        <head>
            <meta charset="utf-8">
            <style>
                body {{ 
                    font-family: 'Times New Roman', Times, serif; 
                    font-size: 14pt; 
                    line-height: 1.5; 
                    color: #000000;
                }}
                .header {{ 
                    text-align: right; 
                    margin-left: 50%; 
                    margin-bottom: 50px; 
                }}
                .title {{ 
                    text-align: center; 
                    font-weight: bold; 
                    font-size: 16pt; 
                    margin-bottom: 40px; 
                    text-transform: uppercase;
                }}
                .content {{ 
                    text-align: justify; 
                    text-indent: 1.25cm; 
                    margin-bottom: 30px;
                }}
                .disclaimer-text {{
                    font-size: 11pt;
                    line-height: 1.4;
                    text-align: justify;
                    color: #333333;
                    margin-bottom: 10px;
                }}
                .footer {{
                    margin-top: 50px;
                    width: 100%;
                }}
            </style>
        </head>
        <body>
            <div class="header">
                В администрацию информационной<br>
                системы «fin_app»
            </div>
            
            <div class="title">ЗАЯВЛЕНИЕ</div>
            
            <div class="content">
                Я, {full_name}, ИНН {pers_inn}, СНИЛС {snils}, 
                прошу зарегистрировать меня в информационной системе «fin_app» как пользователя, 
                осуществляющего деятельность в организации с ИНН {comp_inn} и КПП {kpp}.
            </div>

            <div class="content">
                Подписывая данный документ электронной цифровой подписью (ЭЦП), я подтверждаю факт ознакомления и полного согласия со следующей информацией:
            </div>

            <div class="disclaimer-text">
                1. Данная система носит информационный характер и помогает подготовить документы для юридически значимых действий. За все юридические последствия подписанных документов, сформированных в системе, единоличную ответственность несет пользователь.
            </div>
            <div class="disclaimer-text">
                2. Пользователь подтверждает, что все указанные при регистрации персональные данные и реквизиты организации являются достоверными, актуальными и принадлежат заявителю.
            </div>
            <div class="disclaimer-text">
                3. Для осуществления обратной связи, направления уведомлений и информационных сообщений администрацией системы используются указанные при регистрации номер телефона ({phone}) и (или) адрес электронной почты ({email}).
            </div>
            <div class="disclaimer-text">
                4. Пользователь осознает, что в соответствии с Федеральным законом № 63-ФЗ «Об электронной подписи», подписание настоящего заявления усиленной электронной подписью признается равнозначным документу на бумажном носителе, подписанному собственноручной подписью.
            </div>
            
            <table class="footer">
                <tr>
                    <td style="width: 40%; font-size: 12pt;">Дата формирования:</td>
                    <td style="text-align: right; font-style: italic; font-size: 12pt; color: #555555;">
                        Документ сформирован автоматически и подписан ЭЦП
                    </td>
                </tr>
            </table>
        </body>
        </html>"#,
        full_name = full_name,
        pers_inn = pers_inn.as_ref(),
        snils = snils.as_ref(),
        comp_inn = comp_inn.as_ref(),
        kpp = kpp.as_ref(),
        phone = phone.as_ref(),
        email = email.as_ref()
    );

    let bytes = html_text.into_bytes();

    let hash_res = blake3::hash(&bytes);

    let hash_string = hash_res.to_hex().to_string();

    {
        let mut temp_info_guard = state.temp_info.lock().await;
        temp_info_guard.file_hash = Some(hash_string);
    }

    Ok(bytes)
}