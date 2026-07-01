UPDATE users
SET guids = guids || ARRAY[$2]::uuid[]
WHERE user_id = $1