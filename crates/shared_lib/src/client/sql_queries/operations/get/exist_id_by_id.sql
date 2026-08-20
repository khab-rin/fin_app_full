SELECT
    oper_id AS "oper_id!: BoxUuid" 
FROM operations
WHERE oper_id = $1    
