DELETE FROM sessions s
USING 
    users u JOIN
    persons p ON u.pers_id = p.pers_id JOIN
    companys c ON u.comp_id = c.comp_id
WHERE s.token = $1 AND s.user_id = u.user_id
RETURNING
    u.email AS "email: Email",
    p.pers_inn AS "pers_inn: PersInn",
    c.comp_inn AS "comp_inn: CompInn",
    c.kpp AS "kpp: Kpp"

    