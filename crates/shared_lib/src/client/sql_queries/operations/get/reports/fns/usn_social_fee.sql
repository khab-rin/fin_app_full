SELECT 
	COALESCE(SUM(CASE WHEN oper_date BETWEEN $1 AND $2 THEN amount ELSE 0 END), 0) AS 'q1: RubF',
	COALESCE(SUM(CASE WHEN oper_date BETWEEN $1 AND $3 THEN amount ELSE 0 END), 0) AS 'q2: RubF',
	COALESCE(SUM(CASE WHEN oper_date BETWEEN $1 AND $4 THEN amount ELSE 0 END), 0) AS 'q3: RubF',
	COALESCE(SUM(CASE WHEN oper_date BETWEEN $1 AND $5 THEN amount ELSE 0 END), 0) AS 'q4: RubF'
FROM operations
WHERE is_del = 0 AND credit = '69'
