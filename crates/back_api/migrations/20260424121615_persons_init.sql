CREATE TABLE IF NOT EXISTS persons (
    pers_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pers_inn VARCHAR(12) NOT NULL UNIQUE,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    last_update TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT is_inn_ind
        CHECK (pers_inn ~ '^[0-9]{12}$')

);

CREATE INDEX IF NOT EXISTS idx_persons_inn ON persons(pers_inn);

CREATE OR REPLACE FUNCTION update_persons_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.last_update := CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;


DROP TRIGGER IF EXISTS update_persons_timestamp_trigger ON persons;
CREATE TRIGGER update_persons_timestamp_trigger
BEFORE UPDATE ON persons
FOR EACH ROW 
EXECUTE FUNCTION update_persons_timestamp();


CREATE OR REPLACE FUNCTION clean_persons_input()
RETURNS TRIGGER AS $$
BEGIN
    NEW.pers_inn := REGEXP_REPLACE(NEW.pers_inn, '[^0-9]', '', 'g');
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER clean_persons_input_trigger
BEFORE INSERT OR UPDATE ON persons
FOR EACH ROW EXECUTE FUNCTION clean_persons_input();