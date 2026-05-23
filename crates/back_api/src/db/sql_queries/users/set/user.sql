INSERT INTO users(
    pers_id,
    comp_id,
    phone,
    password_hash,
    email,
    mchd_tax_guid,
    mchd_tax_file,
    mchd_home_guid,
    mchd_home_file
)
VALUES (
    $1, $2, $3, $4, $5, $6, $7, $8, $9
)
RETURNING
    user_id AS "user_id: BoxUuid",
    mchd_tax_guid AS "mchd_tax_guid: BoxUuid",
    mchd_tax_file,
    mchd_home_guid AS "mchd_home_guid: BoxUuid",
    mchd_home_file,
    last_update AS "last_update: DateTime";
