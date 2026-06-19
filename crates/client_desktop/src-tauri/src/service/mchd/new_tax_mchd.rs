use shared_lib::Status;
use shared_lib::service::mchd::service::{MchdStep, NewMchdData, MchdInfo};
use shared_lib::parsers::mchd::implements::PersonDocums;

use crate::state::ClientState;
use crate::service::mchd::helper::check_update_user;

pub(crate) async  fn make_new_tax_mchd(
    state: &ClientState,
    data: &NewMchdData
) -> Result<MchdStep, Status> {

    let failed_result = MchdStep::TryLater { text:MchdInfo::ClientServiceError };

    let session = match state.get_session().await {
        Ok(s) => s,
        Err(err) => {
            log::error!(
                "FUN make_new_tax_mchd FAILED BY MISS Session, err = {}", err
            );
            return Ok(failed_result);
        }
    };

    let mut person = session.session_user.person.clone();

    if !check_update_user(&mut person, data) {
        return Ok(MchdStep::WrongData { text: MchdInfo::WrongPerson })
    }

    let mut session_lock = state.session.lock().await;
    
    // Пересобираем сессию: клонируем старую структуру ActiveSession, 
    // но заменяем в ней person на наш обновленный
    if let Some(current_session) = session_lock.as_ref() {
        let mut updated_session = (**current_session).clone();
        updated_session.session_user.person = person;          // обновляем человека
        
        // Кладим обратно в Mutex, упаковав в новый Arc
        *session_lock = Some(std::sync::Arc::new(updated_session));
    } else {
        // На случай, если пока мы проверяли пользователя, сессия внезапно исчезла (разлогинился)
        log::error!("Session was dropped during check_update_user");
        return Ok(failed_result);
    }
    
    // Не забываем явно или неявно дропнуть лок перед асинхронными операциями дальше
    drop(session_lock);



    let NewMchdData { 
        poa_number, 
        poa_end_date, 
        manager_tittle, 
        manager_sur_name, 
        manager_first_name, 
        manager_mid_name, 
        manager_birth_day, 
        manager_snils, 
        manager_inn, 
        manager_is_citizen, 
        user_sur_name, 
        user_first_name, 
        user_mid_name, 
        user_birth_day, 
        user_gender, 
        user_snils, 
        user_inn, 
        user_passport_number, 
        user_passport_issueer, 
        user_passport_ussuer_code, 
        user_is_citizen,
        powers } = data;
    
    

    Err(Status::Unknown)
}