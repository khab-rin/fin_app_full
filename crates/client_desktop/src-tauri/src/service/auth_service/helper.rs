use shared_lib::Status;
use shared_lib::primitives::frozen::implements::BoxUuid;

pub(crate) fn get_device_id() -> Result<BoxUuid, Status> {
    let id_string = machine_uid::
        get().inspect_err(|err| {
            log::error!(
                "tech_err = {}, stat_err = {}",
                err, Status::SystemErr
            )
        }).map_err(|_| Status::SystemErr)?;

    let id_uuid_str = uuid::Uuid::new_v5(
        &uuid::Uuid::NAMESPACE_DNS,
        id_string.as_bytes()
    ).to_string();

    BoxUuid::new(&id_uuid_str)
}