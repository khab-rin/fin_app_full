use std::sync::Arc;

use shared_lib::Status;
use shared_lib::service::auth_service::implements::WarnEmailData;

use crate::config::BackApiState;

pub(crate) async fn send_warn_mail(
    state_clone: Arc<BackApiState>,
    warn_data: WarnEmailData
) -> Result<Status, Status> {

    let WarnEmailData { 
        email, 
        pers_inn, 
        comp_inn, 
        kpp } = warn_data;

    let html_body = format!(
        "<h3>Служба безопасности: уведомление о входе</h3>\
         <p>Для вашего аккаунта зафиксирована попытка входа с подозрительного устройства.</p>\
         <p><b>Данные пользователя:</b></p>\
         <ul>\
            <li>ИНН Физлица: {pers_inn:?}</li>\
            <li>ИНН Организации: {comp_inn:?}</li>\
            <li>КПП: {kpp:?}</li>\
         </ul>\
         <p>В целях безопасности все активные сессии данного аккаунта были аннулированы. \
         Если это были не вы — немедленно смените пароль.</p>"
    );

    let client = state_clone.config.get_std_client();

    let payload = serde_json::json!({
        "from": state_clone.config.email_sender.from,
        "to": [email.to_string()],
        "subject": "Срочное уведомление безопасности",
        "html": html_body
    });

    let response = client
        .post(&state_clone.config.email_sender.base_url)
        .bearer_auth(&state_clone.config.email_sender.api)
        .json(&payload)
        .send()
        .await
        .inspect_err(|err| {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::QueryPostRequestErr
            )
        }).map_err(|_| Status::QueryPostRequestErr)?;

    if !response.status().is_success() {
        let err = response.text().await.unwrap_or_default();
        tracing::warn!("Resend API вернул ошибку: {}", err);
    }

    Ok(Status::Success)
}