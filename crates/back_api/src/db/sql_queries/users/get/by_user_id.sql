SELECT
    jsonb_build_object(
        'user_id', u.user_id,
        'guids', u.guids,
        'last_update', u.last_update
    ) AS "user!",
    to_jsonb(p) AS "person!",
    to_jsonb(c) AS "company!"
FROM users u
    JOIN persons p ON u.pers_id = p.pers_id
    JOIN companys c ON u.comp_id = c.comp_id
WHERE u.user_id = $1;