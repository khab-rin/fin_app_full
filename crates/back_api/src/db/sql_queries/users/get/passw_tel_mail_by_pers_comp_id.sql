SELECT
    u.phone AS "phone: Phone",
    u.email AS "email: Email",
    u.password_hash
FROM users AS u
JOIN companys AS c ON u.comp_id = c.comp_id
JOIN persons AS p ON u.pers_id = p.pers_id
WHERE c.comp_id = $1 AND p.pers_id = $2