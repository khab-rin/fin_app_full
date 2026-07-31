INSERT INTO persons
(pers_id, pers_inn, metadata, last_update, is_synced) 
VALUES($1, $2, json($3), $4, 1)
ON CONFLICT (pers_inn)
DO UPDATE SET
    metadata = json($3), 
    pers_id = EXCLUDED.pers_id,
    last_update = EXCLUDED.last_update,
    is_synced = 0
RETURNING
    pers_id as "pers_id: BoxUuid",
    pers_inn as "pers_inn: PersInn",
    metadata as "metadata: serde_json::Value",
    last_update as "last_update: DateTime"