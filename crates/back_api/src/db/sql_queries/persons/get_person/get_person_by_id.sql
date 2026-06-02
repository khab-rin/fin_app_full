SELECT
    pers_id AS "pers_id: BoxUuid",
    inn AS "pers_inn: PersInn",
    metadata,
    last_update AS "last_update: DateTime"
FROM persons
WHERE pers_id IN (
    SELECT unnest($1::UUID[])
)