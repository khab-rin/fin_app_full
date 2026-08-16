SELECT
    oper_id AS "external_id!: BoxUuid"
FROM operations
WHERE oper_id IN (
    SELECT CAST(VALUE AS INTEGER) FROM json_each(?1)
)