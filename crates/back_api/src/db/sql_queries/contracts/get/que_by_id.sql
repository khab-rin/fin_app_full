SELECT
    contract_id AS "contract_id: BoxUuid",
    
    user_id AS "user_id: BoxUuid",
    comp_id AS "comp_id: BoxUuid",
    ctrpty_id AS "ctrpty_id: BoxUuid",

    contract_num AS "contract_num: DocNum",

    contract_date AS "contract_date: Date",
    title AS "title: String",

    st_date AS "st_date: Date",
    end_date AS "end_date: Date",

    currency AS "currency: Currency",

    total_amount AS "total_amount: RubF",

    payment_deferral_days AS "payment_deferral_days: Integ",

    is_active AS "is_active: bool",
    descrip AS "descrip: String",
    
    entr_date AS "entr_date: Date",

    is_del AS "is_del: bool"
FROM contracts
WHERE contract_id = $1