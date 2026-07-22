SELECT
    guids AS "guids: Vec<BoxUuid>"
from users
where user_id = $1