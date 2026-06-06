INSERT INTO companys(
    inn,
    kpp,
    comp_type,
    comp_status,
    metadata
) 
SELECT $1, $2, $3, $4, $5
ON CONFLICT (inn, kpp)
    DO UPDATE SET inn = companys.inn
RETURNING
    comp_id AS "comp_id: BoxUuid",
    inn AS "comp_inn: CompInn",
    kpp AS "kpp: Kpp",
    comp_type AS "comp_type: CompType",
    comp_status AS "comp_status: CompStatus",
    metadata,
    last_update AS "last_update: DateTime"