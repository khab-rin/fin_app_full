INSERT INTO persons(pers_id, pers_inn, metadata)
SELECT $1, $2, $3
ON CONFLICT (pers_inn)
    DO UPDATE SET
        metadata = EXCLUDED.metadata
    
RETURNING
    pers_id as "pers_id: BoxUuid",
    pers_inn as "pers_inn: PersInn",
    metadata,
    last_update as "last_update: DateTime"