CREATE TABLE IF NOT EXISTS contracts (
    contract_id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    
    user_id TEXT NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    comp_id TEXT NOT NULL REFERENCES companys(comp_id) ON DELETE CASCADE,
    ctrpty_id TEXT NOT NULL REFERENCES companys(comp_id) ON DELETE CASCADE,

    contract_num TEXT NOT NULL DEFAULT 'б/н',

    contract_date TEXT NOT NULL,
    title TEXT NOT NULL DEFAULT '',

    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,

    currency TEXT NOT NULL DEFAULT 'RUB',

    total_amount REAL NOT NULL DEFAULT 0,

    payment_deferral_days INTEGER DEFAULT 0,

    is_active INTEGER NOT NULL DEFAULT 1,
    description TEXT DEFAULT '',
    
    entr_date TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    is_del INTEGER NOT NULL DEFAULT 0,
    external_id TEXT NOT NULL DEFAULT '' 
);

CREATE INDEX IF NOT EXISTS idx_contracts_comp_id ON contracts(comp_id);
CREATE INDEX IF NOT EXISTS idx_contracts_user_id ON contracts(user_id);
CREATE INDEX IF NOT EXISTS idx_contracts_ctrpty_id ON contracts(ctrpty_id);

CREATE INDEX IF NOT EXISTS idx_contracts_comp_ext_id ON contracts(comp_id, external_id) 
WHERE external_id != '';