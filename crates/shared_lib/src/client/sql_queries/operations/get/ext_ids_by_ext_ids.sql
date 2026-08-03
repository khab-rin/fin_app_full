SELECT
    external_id AS "external_id!: i64"
FROM operations
WHERE external_id IN (
    SELECT CAST(VALUE AS INTEGER) FROM json_each(?1)
)