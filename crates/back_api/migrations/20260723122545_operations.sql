CREATE TABLE IF NOT EXISTS operations (
    oper_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    comp_id UUID NOT NULL REFERENCES companys(comp_id) ON DELETE CASCADE,
    debet VARCHAR(10) NOT NULL,
    credit VARCHAR(10) NOT NULL,
    amount NUMERIC(15, 2) NOT NULL,
    oper_date DATE,
    doc_type VARCHAR(50),
    doc_num VARCHAR(100),
    doc_date DATE,
    entr_date TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_storno BOOLEAN NOT NULL DEFAULT false,
    is_del BOOLEAN NOT NULL DEFAULT false,
    external_id VARCHAR(64)
);

CREATE INDEX IF NOT EXISTS idx_operations_comp_date ON operations(comp_id, oper_date);
CREATE INDEX IF NOT EXISTS idx_operations_user_date ON operations(user_id, oper_date);
CREATE INDEX IF NOT EXISTS idx_operations_comp_debet ON operations(comp_id, debet);
CREATE INDEX IF NOT EXISTS idx_operations_comp_credit ON operations(comp_id, credit);

CREATE UNIQUE INDEX IF NOT EXISTS idx_operations_comp_ext_id 
ON operations(comp_id, external_id) 
WHERE external_id IS NOT NULL;
    
