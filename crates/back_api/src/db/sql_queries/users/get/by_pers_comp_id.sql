SELECT
    user_id AS "user_id: BoxUuid",
    guids AS "guids: Vec<BoxUuid>",
    last_update AS "last_update: DateTime"
FROM users
WHERE pers_id = $1 AND comp_id = $2