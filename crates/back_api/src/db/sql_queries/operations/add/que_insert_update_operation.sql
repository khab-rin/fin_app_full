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
) VALUES(
	$1, $2, $3,$4,$5,$6,$7, $8, $9,$10,$11,$12,$13, $14, $15
) ON CONFLICT(oper_id) 
	DO UPDATE SET
		is_del = EXCLUDED.is_del

RETURNING
	oper_id as "oper_id: BoxUuid",

	user_id as "user_id: BoxUuid",
	comp_id as "comp_id: BoxUuid",
	ctrpty_id as "ctrpty_id: BoxUuid",

	contract_id as "contract_id: BoxUuid",

	debet as "debet: Account",
	credit as "credit: Account",
	amount as "amount: RubF",
	oper_date as "oper_date: Date",

	doc_type as "doc_type: DocType",
	doc_num as "doc_num: DocNum",
	doc_date as "doc_date: Date",

	is_storno as "is_storno: bool",
	is_del as "is_del: bool",

	entr_date as "entr_date: Date"