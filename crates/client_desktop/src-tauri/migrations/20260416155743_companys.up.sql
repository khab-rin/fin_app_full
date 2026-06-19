CREATE TABLE IF NOT EXISTS companys (
    comp_id TEXT PRIMARY KEY NOT NULL,
    comp_inn TEXT NOT NULL,
    kpp TEXT NOT NULL DEFAULT '0',
    comp_type TEXT NOT NULL,
    comp_status TEXT NOT NULL DEFAULT 'ACTIVE',
    metadata TEXT NOT NULL DEFAULT '{}',
    last_update DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_synced INTEGER NOT NULL DEFAULT 0,
     
    UNIQUE (comp_inn, kpp),

    CONSTRAINT check_inn_kpp_logic CHECK (
    CASE 
        WHEN length(comp_inn) = 12 THEN kpp = '0'
        WHEN length(comp_inn) = 10 THEN length(kpp) = 9
        ELSE 0
    END),

    CONSTRAINT check_inn_length CHECK (length(comp_inn) IN (10, 12)),

    CONSTRAINT check_kpp_length CHECK (length(kpp) IN (1, 9)),

    CONSTRAINT check_inn_numeric CHECK (comp_inn GLOB '[0-9]*'),
    
    CONSTRAINT check_kpp_numeric CHECK (kpp GLOB '[0-9]*'),

    CONSTRAINT check_synced_bool CHECK (is_synced IN (0, 1))
);

CREATE INDEX IF NOT EXISTS inn_kpp_index ON companys(comp_inn, kpp);

