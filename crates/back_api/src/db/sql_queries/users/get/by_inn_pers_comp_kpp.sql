SELECT 
    u.user_id AS "user_id: BoxUuid",
    u.guids AS "guids: Vec<BoxUuid>",
    u.last_update AS "last_update: DateTime"
FROM users AS u
    JOIN persons AS p ON u.pers_id = p.pers_id
    JOIN companys AS c ON u.comp_id = c.comp_id
WHERE
    p.pers_inn = $1 AND c.comp_inn = $2 AND c.kpp = $3