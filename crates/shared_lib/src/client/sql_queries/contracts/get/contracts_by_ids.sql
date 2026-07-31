SELECT
    contract_id AS "contract_id: BoxUuid",
    
    user_id AS "user_id: BoxUuid",
    comp_id AS "comp_id: BoxUuid",
    ctrpty_id AS"ctrpty_id: BoxUuid",

    contract_num AS"contract_num: String",

    contract_date AS "contract_date: Date",
    title AS "title: String",

    start_date AS "start_date: Date",
    end_date AS "end_date: Date",

    currency AS "currency: String",

    total_amount AS "total_amount: RubF",

    payment_deferral_days AS "payment_deferral_days: i32",

    is_active AS "is_active: i32",
    description AS "description: String",
    
    entr_date AS "entr_date: Date",
    updated_at AS "updated_at: DateTime",
    is_del AS "is_del: i32",
    external_id AS "external_id: String"
FROM contracts
WHERE comp_id = $1 AND ctrpty_id = $2