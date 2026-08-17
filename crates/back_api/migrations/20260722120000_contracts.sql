CREATE TABLE IF NOT EXISTS contracts (
    contract_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    user_id UUID NOT NULL REFERENCES users(user_id),
    comp_id UUID NOT NULL REFERENCES companys(comp_id),
    ctrpty_id UUID NOT NULL REFERENCES companys(comp_id),


    contract_num VARCHAR(100) NOT NULL DEFAULT 'б/н',
    contract_date DATE NOT NULL,
    title VARCHAR(255) NOT NULL DEFAULT '',

    start_date DATE NOT NULL,
    end_date DATE NOT NULL,

    currency VARCHAR(3) NOT NULL DEFAULT 'RUB',
    total_amount NUMERIC(15, 2) NOT NULL DEFAULT 0,

    payment_deferral_days INT NOT NULL DEFAULT 0,

    is_active BOOLEAN NOT NULL DEFAULT true,
    description TEXT DEFAULT '',
    
    entr_date TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    is_del BOOLEAN NOT NULL DEFAULT false

);
