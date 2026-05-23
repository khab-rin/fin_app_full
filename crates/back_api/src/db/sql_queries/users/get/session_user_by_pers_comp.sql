SELECT
    to_jsonb(u) AS "user",
    to_jsonb(p) AS "person",
    to_jsonb(c) AS "company" 
FROM users u
JOIN persons p ON u.pers_id = p.pers_id
JOIN companys c ON u.comp_id = c.comp_id
WHERE p.pers_id = $1 AND c.comp_id = $2