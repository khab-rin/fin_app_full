INSERT INTO persons(pers_id, inn, metadata)
SELECT $1, $2, $3
ON CONFLICT (inn)
DO UPDATE SET
    metadata = persons.metadata || EXCLUDED.metadata
RETURNING
    pers_id as "pers_id: BoxUuid",
    inn as "inn: Inn",
    metadata,
    last_update as "last_update: DateTime"