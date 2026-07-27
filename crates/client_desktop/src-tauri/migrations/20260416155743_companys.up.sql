CREATE TABLE IF NOT EXISTS companys (
    comp_id TEXT PRIMARY KEY NOT NULL,
    comp_inn TEXT NOT NULL,
    kpp TEXT NOT NULL DEFAULT '',
    comp_type TEXT NOT NULL,
    comp_status TEXT NOT NULL DEFAULT 'ACTIVE',
    metadata TEXT NOT NULL DEFAULT '{}',
    last_update DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
      
    UNIQUE (comp_inn, kpp),

    CONSTRAINT check_inn_kpp_logic CHECK (
        CASE 
            WHEN length(comp_inn) = 12 THEN kpp = ''
            WHEN length(comp_inn) = 10 THEN length(kpp) = 9 AND kpp GLOB '[0-9][0-9][0-9][0-9][0-9][0-9][0-9][0-9][0-9]'
            ELSE 0
        END
    ),

    CONSTRAINT check_inn_length CHECK (length(comp_inn) IN (10, 12)),
    
    CONSTRAINT check_inn_numeric CHECK (
        (length(comp_inn) = 10 AND comp_inn GLOB '[0-9][0-9][0-9][0-9][0-9][0-9][0-9][0-9][0-9][0-9]') OR
        (length(comp_inn) = 12 AND comp_inn GLOB '[0-9][0-9][0-9][0-9][0-9][0-9][0-9][0-9][0-9][0-9][0-9][0-9]')
    ),

    CONSTRAINT check_comp_type CHECK (
        comp_type IN ('BANK', 'GOV', 'IP', 'COM_ENT') AND
        NOT (length(comp_inn) = 12 AND comp_type != 'IP') AND
        NOT (length(comp_inn) = 10 AND comp_type = 'IP')
    ),

    CONSTRAINT check_comp_status CHECK (
        comp_status IN ('ACTIVE', 'LIQUIDATING', 'LIQUIDATED', 'BANKRUPT', 'REORGANIZING')
    ) -- Запятая здесь убрана
);

CREATE INDEX IF NOT EXISTS comp_type_ind ON companys(comp_type);