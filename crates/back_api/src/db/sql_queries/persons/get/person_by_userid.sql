SELECT
    p.pers_id AS "pers_id: BoxUuid",
    p.pers_inn AS "pers_inn: PersInn",
    p.metadata,
    p.last_update AS "last_update: DateTime"
FROM users AS u 
JOIN persons AS p ON u.pers_id = p.pers_id
WHERE u.user_id = $1
