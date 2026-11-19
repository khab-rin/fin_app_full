select
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
from operations
where oper_id = $1