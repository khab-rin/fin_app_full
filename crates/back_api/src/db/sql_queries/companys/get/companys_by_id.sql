SELECT
    comp_id AS "comp_id: BoxUuid",
    comp_inn AS "comp_inn: CompInn",
    kpp AS "kpp: Kpp",
    comp_type AS "comp_type: CompType",
    comp_status AS "comp_status: CompStatus",
    metadata,
    last_update AS "last_update: DateTime"
FROM companys
WHERE comp_id IN (
    SELECT unnest($1::UUID[])
)