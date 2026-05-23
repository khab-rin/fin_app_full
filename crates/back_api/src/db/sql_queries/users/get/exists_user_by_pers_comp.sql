SELECT 1 AS exists_flag
FROM users 
WHERE pers_id = $1 AND comp_id = $2
LIMIT 1;