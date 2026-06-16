SELECT
    jsonb_build_object(
        'user_id', u.user_id,
        'mchd_tax_guid', u.mchd_tax_guid,
        'tax_powers', u.tax_powers,
        'mchd_home_guid', u.mchd_home_guid,
        'home_powers', u.home_powers,
        'last_update', u.last_update
    ) AS "user!",
    to_jsonb(p) AS "person!",
    to_jsonb(c) AS "company!"
FROM users u
    JOIN persons p ON u.pers_id = p.pers_id
    JOIN companys c ON u.comp_id = c.comp_id
WHERE u.user_id = $1;