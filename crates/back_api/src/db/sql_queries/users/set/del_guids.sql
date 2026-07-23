UPDATE users
SET 
    guids = $2::uuid[],
    last_update = CURRENT_TIMESTAMP
WHERE user_id = $1
RETURNING user_id AS "user_id: BoxUuid";