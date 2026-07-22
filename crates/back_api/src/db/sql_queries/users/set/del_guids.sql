UPDATE users
SET guids = $2
WHERE user_id = $1;