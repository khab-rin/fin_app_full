SELECT
    pers_id AS "pers_id: BoxUuid",
    pers_inn AS "pers_inn: PersInn",
    metadata,
    last_update AS "last_update: DateTime"
FROM persons
WHERE pers_inn = $1