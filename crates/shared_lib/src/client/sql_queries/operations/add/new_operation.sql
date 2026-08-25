INSERT INTO operations(
	oper_id,  
    user_id,
    comp_id,
    ctrpty_id,
    contract_id,
    debet,
    credit,
	amount,
    oper_date,
    doc_type,
    doc_num,
    doc_date,
    is_storno,
    is_del,
    entr_date
)
VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
ON CONFLICT (oper_id) DO NOTHING;
    