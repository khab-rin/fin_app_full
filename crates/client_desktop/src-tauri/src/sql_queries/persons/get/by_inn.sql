SELECT
    pers_id as "pers_id: BoxUuid",
    pers_inn as "pers_inn: PersInn",
    metadata,
    last_update as "last_update: DateTime"
FROM persons
WHERE pers_inn = $1