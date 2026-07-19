use shared_lib::Status;
use shared_lib::primitives::frozen::implements::BoxUuid;

pub(crate) fn get_device_id() -> Result<BoxUuid, Status> {

    let id_string = match machine_uid::get() {
        Ok(i) => i,
        Err(err) => {
            log::error!(
                "FUN get_device_id FAILED BY machine_uid::get(), tech_err = {:?}, local_err = {:?}",
                err, Status::SystemErr
            );
            return Err(Status::SystemErr);
        }
    };

    let id_uuid_str = uuid::Uuid::new_v5(
        &uuid::Uuid::NAMESPACE_DNS,
        id_string.as_bytes()
    ).to_string();

    BoxUuid::new(&id_uuid_str)
}

