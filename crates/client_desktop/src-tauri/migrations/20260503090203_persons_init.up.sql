CREATE TABLE IF NOT EXISTS persons (
    pers_id TEXT PRIMARY KEY NOT NULL,
    pers_inn TEXT NOT NULL UNIQUE,
    metadata TEXT NOT NULL DEFAULT '{}',
    last_update DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_synced INTEGER NOT NULL DEFAULT 0,

    CONSTRAINT check_inn_person CHECK (length(pers_inn) = 12 AND pers_inn GLOB '[0-9]*'),
    CONSTRAINT check_synced_bool CHECK (is_synced IN (0, 1))

);

CREATE INDEX IF NOT EXISTS idx_persons_inn ON persons(pers_inn);

DROP TRIGGER IF EXISTS trg_persons_update_timestamp;
CREATE TRIGGER trg_persons_update_timestamp
AFTER UPDATE ON persons
FOR EACH ROW
BEGIN
    UPDATE persons 
    SET last_update = CURRENT_TIMESTAMP 
    WHERE pers_id = OLD.pers_id;
END;


DROP TRIGGER IF EXISTS trg_persons_clean_insert;
CREATE TRIGGER trg_persons_clean_insert
BEFORE INSERT ON persons
FOR EACH ROW
BEGIN
    SELECT CASE
        WHEN NEW.pers_inn NOT GLOB '[0-9]*'
        THEN RAISE(ABORT, 'ИНН должны содержать только цифры')
    END;
END;