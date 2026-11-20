INSERT INTO persons(
	pers_id,
	pers_inn,
	metadata
)
VALUES ($1, $2, $3)
ON CONFLICT (pers_inn)
	DO UPDATE SET
		metadata = EXCLUDED.metadata,
		last_update = now()

RETURNING
    pers_id as "pers_id: BoxUuid",
    pers_inn as "pers_inn: PersInn",
    metadata,
    last_update as "last_update: DateTime"
