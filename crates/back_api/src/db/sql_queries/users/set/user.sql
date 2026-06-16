INSERT INTO users(
    pers_id,
    comp_id,
    phone,
    password_hash,
    email,
    mchd_tax_guid,
    tax_powers,
    mchd_home_guid,
    home_powers
)
VALUES (
    $1, $2, $3, $4, $5, $6, $7, $8, $9
)
RETURNING
    user_id AS "user_id: BoxUuid",
    mchd_tax_guid AS "mchd_tax_guid: BoxUuid",
    tax_powers,
    mchd_home_guid AS "mchd_home_guid: BoxUuid",
    home_powers,
    last_update AS "last_update: DateTime";
