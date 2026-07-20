CREATE TABLE IF NOT EXISTS companys (
    comp_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    comp_inn VARCHAR(12) NOT NULL,
    kpp VARCHAR(9) NOT NULL DEFAULT '',
    comp_type VARCHAR(10) NOT NULL,
    comp_status VARCHAR(12) NOT NULL DEFAULT 'ACTIVE',
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    last_update TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT is_inn
        CHECK (comp_inn ~ '^([0-9]{10}|[0-9]{12})$'),

    CONSTRAINT is_kpp
        CHECK (
            (LENGTH(comp_inn) = 12 AND kpp = '') OR
            (LENGTH(comp_inn) = 10 AND kpp ~ '^[0-9]{9}$')
        ),

    CONSTRAINT valid_type
        CHECK (comp_type in ('BANK', 'GOV', 'IP', 'COM_ENT')),
    
    CONSTRAINT valid_status
        CHECK (comp_status in ('ACTIVE', 'LIQUIDATING', 'LIQUIDATED', 'BANKRUPT', 'REORGANIZING'))
);

CREATE UNIQUE INDEX IF NOT EXISTS inn_kpp_ind ON companys(comp_inn, kpp);
CREATE INDEX IF NOT EXISTS comp_type_ind ON companys(comp_type);


CREATE OR REPLACE FUNCTION process_input()
RETURNS TRIGGER AS $$
BEGIN
    NEW.comp_type := UPPER(NEW.comp_type);
    NEW.comp_status := UPPER(NEW.comp_status);

    IF (TG_OP = 'UPDATE') THEN
        IF (NEW.comp_inn <> OLD.comp_inn) OR (NEW.comp_type <> OLD.comp_type) THEN
            RAISE EXCEPTION 'Запрещено менять ИНН или Тип организации';
        END IF;
    END IF;

    IF LENGTH(NEW.comp_inn) = 12 THEN
        IF NEW.comp_type != 'IP' AND NEW.comp_type IS NOT NULL THEN
            RAISE EXCEPTION 'Пара ИНН - ТИП ошибочна: для ИНН из 12 цифр тип должен быть IP';
        END IF;
        
        NEW.kpp := '';
        NEW.comp_type := 'IP';
    ELSIF LENGTH(NEW.comp_inn) = 10 THEN
        IF NEW.comp_type = 'IP' THEN
            RAISE EXCEPTION 'Пара ИНН - ТИП ошибочна: организация с ИНН из 10 цифр не может быть IP';
        END IF;
    END IF;

    NEW.last_update := CURRENT_TIMESTAMP;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;