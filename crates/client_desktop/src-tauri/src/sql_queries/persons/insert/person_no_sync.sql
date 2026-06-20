INSERT INTO persons
(pers_id, pers_inn, metadata, last_update, is_synced) 
VALUES($1, $2, $3, $4, 0)
ON CONFLICT (pers_inn)
DO UPDATE SET
    metadata = (
        SELECT json_group_object(key, value)
        FROM (
            SELECT key, value FROM json_each(persons.metadata)
            UNION ALL
            SELECT key, value FROM json_each(EXCLUDED.metadata) 
            WHERE json_type(EXCLUDED.metadata, '$.' || key) IS NOT 'null'
        )
    ),
    pers_id = EXCLUDED.pers_id,
    last_update = EXCLUDED.last_update,
    is_synced = 0
RETURNING
    pers_id as "pers_id: BoxUuid",
    pers_inn as "pers_inn: PersInn",
    metadata,
    last_update as "last_update: DateTime"
