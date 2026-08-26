INSERT INTO contracts(
    contract_id,
    user_id,
    comp_id,
    ctrpty_id,
    contract_num,
    contract_date,
    title,
    st_date,
    end_date,
    currency,
    total_amount,
    payment_deferral_days,
    is_active,
    descrip,
    entr_date,
    is_del
) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
ON CONFLICT (contract_id)
    DO UPDATE SET
        descrip = EXCLUDED.descrip,
        is_del = EXCLUDED.is_del
RETURNING
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