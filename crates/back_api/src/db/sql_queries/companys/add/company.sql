INSERT INTO companys(
    comp_inn,
    kpp,
    comp_type,
    comp_status,
    metadata
) 
SELECT $1, $2, $3, $4, $5
ON CONFLICT (comp_inn, kpp)
    DO UPDATE SET comp_inn = companys.comp_inn
RETURNING
    comp_id AS "comp_id: BoxUuid",
    comp_inn AS "comp_inn: CompInn",
    kpp AS "kpp: Kpp",
    comp_type AS "comp_type: CompType",
    comp_status AS "comp_status: CompStatus",
    metadata,
    last_update AS "last_update: DateTime"