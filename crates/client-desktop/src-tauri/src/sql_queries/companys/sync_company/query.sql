INSERT INTO companys(
    comp_id,
    inn,
    kpp,
    comp_type,
    comp_status,
    metadata,
    last_update,
    is_synced
)
VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, 1)
ON CONFLICT(inn, kpp) DO UPDATE SET
    comp_id = excluded.comp_id,
    comp_type = excluded.comp_type,
    comp_status = excluded.comp_status,
    metadata = excluded.metadata,
    last_update = excluded.last_update,
    is_synced = 1
RETURNING
    comp_id AS "comp_id: Uuid",
    inn AS "inn:Inn",
    kpp AS "kpp:Kpp"
