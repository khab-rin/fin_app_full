CREATE TABLE IF NOT EXISTS contracts (
    contract_id TEXT PRIMARY KEY NOT NULL,
    
    user_id TEXT NOT NULL,
    comp_id TEXT NOT NULL REFERENCES companys(comp_id),
    ctrpty_id TEXT NOT NULL REFERENCES companys(comp_id),

    contract_num TEXT NOT NULL,
    contract_date TEXT NOT NULL,
    title TEXT NOT NULL,

    st_date TEXT NOT NULL,
    end_date TEXT NOT NULL,

    currency TEXT NOT NULL,

    total_amount TEXT NOT NULL,

    payment_deferral_days INTEGER NOT NULL,

    is_active BOOLEAN NOT NULL,
    descrip TEXT NOT NULL DEFAULT '',
    
    entr_date TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),

    is_del BOOLEAN NOT NULL,

	is_synced INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_contracts_comp_id ON contracts(comp_id);
CREATE INDEX IF NOT EXISTS idx_contracts_user_id ON contracts(user_id);
CREATE INDEX IF NOT EXISTS idx_contracts_ctrpty_id ON contracts(ctrpty_id);