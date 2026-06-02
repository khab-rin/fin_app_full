SELECT
    comp_id AS "comp_id: BoxUuid",
    inn AS "comp_inn: CompInn",
    kpp AS "kpp: Kpp",
    comp_type AS "comp_type: CompType",
    comp_status AS "comp_status: CompStatus",
    metadata,
    last_update AS "last_update: DateTime"
FROM companys
WHERE inn = $1 AND kpp = $2