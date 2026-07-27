INSERT INTO companys(
    comp_id,
    comp_inn, 
    kpp, 
    comp_type, 
    comp_status, 
    metadata) 
SELECT * FROM UNNEST(
    $1::uuid[], 
    $2::text[], 
    $3::text[],
    $4::text[],
    $5::text[],
    $6::jsonb[])
ON CONFLICT (comp_inn, kpp) DO UPDATE SET
    comp_status = EXCLUDED.comp_status,
    metadata = EXCLUDED.metadata,
    last_update = CURRENT_TIMESTAMP
RETURNING
    comp_id AS "comp_id: BoxUuid",
    comp_inn  AS "comp_inn:CompInn",
    kpp AS "kpp:Kpp",
    comp_type as "comp_type:CompType", 
    comp_status as "comp_status:CompStatus", 
    metadata,
    last_update as "last_update: DateTime";
