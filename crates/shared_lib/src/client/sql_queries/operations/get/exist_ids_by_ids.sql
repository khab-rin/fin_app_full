SELECT
    oper_id AS "oper_id: BoxUuid"
FROM operations
WHERE oper_id IN (
    SELECT value FROM json_each(?1)
)