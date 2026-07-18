SELECT
    c.comp_id AS "comp_id: BoxUuid",
    c.comp_inn AS "comp_inn: CompInn",
    c.kpp AS "kpp: Kpp",
    c.comp_type AS "comp_type: CompType",
    c.comp_status AS "comp_status: CompStatus",
    c.metadata,
    c.last_update AS "last_update: DateTime"
FROM users AS u
JOIN companys AS c ON u.comp_id = c.comp_id
WHERE u.user_id = $1