INSERT INTO companys(
    inn, 
    kpp, 
    comp_type, 
    comp_status, 
    metadata) 
SELECT * FROM UNNEST(
    $1::text[], 
    $2::text[], 
    $3::text[],
    $4::text[],
    $5::jsonb[])
ON CONFLICT (inn, kpp) DO UPDATE SET
    comp_status = EXCLUDED.comp_status,
    metadata = EXCLUDED.metadata,
    last_update = CURRENT_TIMESTAMP
RETURNING
    comp_id AS "comp_id: BoxUuid",
    inn  AS "comp_inn:CompInn",
    kpp AS "kpp:Kpp",
    comp_type as "comp_type:CompType", 
    comp_status as "comp_status:CompStatus", 
    metadata,
    last_update as "last_update: DateTime";
