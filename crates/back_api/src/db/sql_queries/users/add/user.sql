INSERT INTO users(
	user_id,
    pers_id,
    comp_id,
    phone,
    password_hash,
    email,
    guids
)
VALUES (
    $1, $2, $3, $4, $5, $6, $7
)
ON CONFLICT (pers_id, comp_id)
	DO UPDATE SET
		email = EXCLUDED.email,
		guids = EXCLUDED.guids
RETURNING
    user_id AS "user_id: BoxUuid",
    guids AS "guids: Vec<BoxUuid>",
    last_update AS "last_update: DateTime";
