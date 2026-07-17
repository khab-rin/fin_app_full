SELECT 
    phone AS "phone: Phone",
    email AS "email: Email"
FROM users
WHERE user_id = $1