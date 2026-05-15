INSERT INTO call_cf(user_id, device_id, external_id)
SELECT $1, $2, $3
ON CONFLICT (user_id, device_id)
DO UPDATE SET
    external_id = EXCLUDED.external_id,
    expires_t = CURRENT_TIMESTAMP + INTERVAL '5 minutes'
RETURNING true as "success!";