SELECT 
    u.user_id AS "user_id: BoxUuid",
    u.phone AS "phone: Phone", 
    u.password_hash, 
    s.token AS "token: BoxUuid"
FROM users AS u
JOIN persons AS p ON u.pers_id = p.pers_id
JOIN companys AS c ON u.comp_id = c.comp_id
LEFT JOIN sessions AS s ON u.user_id = s.user_id AND s.device_id = $4
WHERE p.pers_inn = $1 AND 
    c.comp_inn = $2 AND 
    c.kpp = $3
    