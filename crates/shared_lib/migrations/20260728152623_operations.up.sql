CREATE TABLE IF NOT EXISTS operations (
    oper_id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    
    user_id TEXT NOT NULL,
    comp_id TEXT NOT NULL REFERENCES companys(comp_id),
    ctrpty_id TEXT NOT NULL REFERENCES companys(comp_id),

    contract_id TEXT REFERENCES contracts(contract_id),

    debet TEXT NOT NULL,
    credit TEXT NOT NULL,

    amount REAL NOT NULL,

    oper_date TEXT,

    doc_type TEXT,
    doc_num TEXT,
    doc_date TEXT,

    is_storno BOOLEAN NOT NULL DEFAULT false,
    is_del BOOLEAN NOT NULL DEFAULT false,

    entr_date TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))

);


CREATE INDEX IF NOT EXISTS idx_operations_comp_date ON operations(comp_id, oper_date);
CREATE INDEX IF NOT EXISTS idx_operations_user_date ON operations(user_id, oper_date);
CREATE INDEX IF NOT EXISTS idx_operations_comp_debet ON operations(comp_id, debet);
CREATE INDEX IF NOT EXISTS idx_operations_comp_credit ON operations(comp_id, credit);
