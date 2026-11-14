INSERT INTO persons
(pers_id, pers_inn, metadata, last_update, is_synced)
VALUES($1, $2, $3, $4, $5)
ON CONFLICT(pers_inn)
DO UPDATE SET
	metadata = EXCLUDED.metadata,
	last_update = EXCLUDED.last_update,
	is_synced = EXCLUDED.is_synced
RETURNING
	pers_id as "pers_id: BoxUuid",
	pers_inn as "pers_inn: PersInn",
	metadata as "metadata: serde_json::Value",
	last_update as "last_update: DateTime"