CREATE TABLE IF NOT EXISTS operations (
    oper_id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    
    user_id TEXT NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    comp_id TEXT NOT NULL REFERENCES companys(comp_id) ON DELETE CASCADE,
    ctrpty_id TEXT NOT NULL REFERENCES companys(comp_id) ON DELETE CASCADE,
    contract_id TEXT REFERENCES contracts(contract_id) ON DELETE CASCADE,

    debet TEXT NOT NULL,
    credit TEXT NOT NULL,

    amount REAL NOT NULL,

    oper_date TEXT,

    doc_type TEXT,
    doc_num TEXT,
    doc_date TEXT,

    is_storno INTEGER NOT NULL DEFAULT 0,
    is_del INTEGER NOT NULL DEFAULT 0,

    entr_date TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    
    external_id TEXT
);


CREATE INDEX IF NOT EXISTS idx_operations_comp_date ON operations(comp_id, oper_date);
CREATE INDEX IF NOT EXISTS idx_operations_user_date ON operations(user_id, oper_date);
CREATE INDEX IF NOT EXISTS idx_operations_comp_debet ON operations(comp_id, debet);
CREATE INDEX IF NOT EXISTS idx_operations_comp_credit ON operations(comp_id, credit);

CREATE UNIQUE INDEX IF NOT EXISTS idx_operations_comp_ext_id 
ON operations(comp_id, external_id) 
WHERE external_id IS NOT NULL;