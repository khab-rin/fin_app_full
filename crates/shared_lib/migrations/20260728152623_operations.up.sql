CREATE TABLE IF NOT EXISTS operations (
    oper_id TEXT PRIMARY KEY NOT NULL,
    
    user_id TEXT NOT NULL,
    comp_id TEXT NOT NULL REFERENCES companys(comp_id),
    ctrpty_id TEXT NOT NULL REFERENCES companys(comp_id),

    contract_id TEXT NOT NULL REFERENCES contracts(contract_id),

    debet TEXT NOT NULL,
    credit TEXT NOT NULL,

    amount REAL NOT NULL,

    oper_date TEXT NOT NULL,

    doc_type TEXT NOT NULL,
    doc_num TEXT NOT NULL,
    doc_date TEXT NOT NULL,

    is_storno BOOLEAN NOT NULL,
    is_del BOOLEAN NOT NULL,

    entr_date TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),

	is_synced INTEGER NOT NULL

);


CREATE INDEX IF NOT EXISTS idx_operations_comp_date ON operations(comp_id, oper_date);
CREATE INDEX IF NOT EXISTS idx_operations_user_date ON operations(user_id, oper_date);
CREATE INDEX IF NOT EXISTS idx_operations_comp_debet ON operations(comp_id, debet);
CREATE INDEX IF NOT EXISTS idx_operations_comp_credit ON operations(comp_id, credit);
