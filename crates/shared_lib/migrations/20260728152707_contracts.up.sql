CREATE TABLE IF NOT EXISTS contracts (
    contract_id TEXT PRIMARY KEY NOT NULL DEFAULT (lower(hex(randomblob(16)))),
    
    user_id TEXT NOT NULL,
    comp_id TEXT NOT NULL REFERENCES companys(comp_id),
    ctrpty_id TEXT NOT NULL REFERENCES companys(comp_id),

    contract_num TEXT NOT NULL DEFAULT 'б/н',
    contract_date TEXT NOT NULL,
    title TEXT NOT NULL DEFAULT '',

    st_date TEXT NOT NULL,
    end_date TEXT NOT NULL,

    currency TEXT NOT NULL DEFAULT 'RUB',

    total_amount TEXT NOT NULL DEFAULT '0',

    payment_deferral_days INTEGER NOT NULL DEFAULT 0,

    is_active INTEGER NOT NULL DEFAULT 1,
    descrip TEXT NOT NULL DEFAULT '',
    
    entr_date TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),

    is_del INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_contracts_comp_id ON contracts(comp_id);
CREATE INDEX IF NOT EXISTS idx_contracts_user_id ON contracts(user_id);
CREATE INDEX IF NOT EXISTS idx_contracts_ctrpty_id ON contracts(ctrpty_id);