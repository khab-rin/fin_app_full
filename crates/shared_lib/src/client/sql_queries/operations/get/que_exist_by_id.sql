SELECT EXISTS (
    SELECT 1 FROM operations WHERE oper_id = $1
) AS "operation_exist: bool";