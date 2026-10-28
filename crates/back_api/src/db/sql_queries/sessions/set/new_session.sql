INSERT INTO sessions (session_id, user_id, device_id)
VALUES ($1, $2, $3)
ON CONFLICT (user_id, device_id)
DO UPDATE SET 
    last_login = CURRENT_TIMESTAMP
RETURNING token AS "token: BoxUuid";