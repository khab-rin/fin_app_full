CREATE TABLE IF NOT EXISTS contracts (
    contract_id UUID PRIMARY KEY,
    
    user_id UUID NOT NULL REFERENCES users(user_id),
    comp_id UUID NOT NULL REFERENCES companys(comp_id),
    ctrpty_id UUID NOT NULL REFERENCES companys(comp_id),


    contract_num VARCHAR(100) NOT NULL,
    contract_date DATE NOT NULL,
    title VARCHAR(255) NOT NULL,

    st_date DATE NOT NULL,
    end_date DATE NOT NULL,

    currency VARCHAR(3) NOT NULL,
    total_amount NUMERIC(15, 2) NOT NULL,

    payment_deferral_days INT NOT NULL,

    is_active BOOLEAN NOT NULL,
    descrip TEXT NOT NULL DEFAULT '',
    
    entr_date DATE NOT NULL DEFAULT CURRENT_DATE,

    is_del BOOLEAN NOT NULL

);
