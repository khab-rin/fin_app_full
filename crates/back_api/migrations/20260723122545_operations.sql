CREATE TABLE IF NOT EXISTS operations (
    oper_id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(user_id),

    comp_id UUID NOT NULL REFERENCES companys(comp_id),
    ctrpty_id UUID NOT NULL REFERENCES companys(comp_id),
    contract_id UUID NOT NULL REFERENCES contracts(contract_id),

    debet VARCHAR(10) NOT NULL,
    credit VARCHAR(10) NOT NULL,
    amount NUMERIC(15, 2) NOT NULL,
    oper_date DATE,

    doc_type VARCHAR(50),
    doc_num VARCHAR(100),
    doc_date DATE,

    is_storno BOOLEAN NOT NULL DEFAULT false,
    is_del BOOLEAN NOT NULL DEFAULT false,

    entr_date DATE NOT NULL DEFAULT CURRENT_DATE
);

CREATE INDEX IF NOT EXISTS idx_operations_comp_date ON operations(comp_id, oper_date);
CREATE INDEX IF NOT EXISTS idx_operations_user_date ON operations(user_id, oper_date);
CREATE INDEX IF NOT EXISTS idx_operations_comp_debet ON operations(comp_id, debet);
CREATE INDEX IF NOT EXISTS idx_operations_comp_credit ON operations(comp_id, credit);

    
