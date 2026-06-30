WITH update_session AS (
    UPDATE sessions
    SET last_login = NOW()
    WHERE device_id = $1 AND token = $2
    RETURNING user_id
),
tab1 AS (
    SELECT 
        t2.user_id, 
        t2.pers_id, 
        t2.comp_id,
        t2.guids,
        t2.last_update
    FROM update_session AS t1
    JOIN users AS t2 ON t1.user_id = t2.user_id
)

SELECT 
    jsonb_build_object(
        'user_id', t1.user_id,
        'guids', t1.guids,
        'last_update', t1.last_update
    ) AS "user!", 
    to_jsonb(t2) AS "person!", 
    to_jsonb(t3) AS "company!"
FROM tab1 AS t1
JOIN persons AS t2 ON t1.pers_id = t2.pers_id
JOIN companys AS t3 ON t1.comp_id = t3.comp_id

