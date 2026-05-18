WITH garbage_collector AS (
    DELETE FROM call_cf
    WHERE expires_t < CURRENT_TIMESTAMP
),
current_call AS (
    DELETE FROM call_cf
    WHERE device_id = $1 
      AND external_id = $2 
      AND expires_t >= CURRENT_TIMESTAMP
    RETURNING user_id, expires_t
)
SELECT 
    user_id AS "user_id: BoxUuid",
    expires_t AS "expires_t: DateTime"
FROM current_call;