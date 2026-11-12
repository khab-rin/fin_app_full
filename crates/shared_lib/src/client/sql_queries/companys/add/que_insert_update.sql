INSERT INTO companys(
	comp_id,
    comp_inn,
    kpp,
    comp_type,
    comp_status,
    metadata,
	last_update
) 
VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
ON CONFLICT (comp_inn, kpp)
    DO UPDATE SET 
        comp_type = EXCLUDED.comp_type,
        comp_status = EXCLUDED.comp_status,
        metadata = EXCLUDED.metadata,
		last_update = EXCLUDED.last_update