
CREATE TABLE IF NOT EXISTS call_cf (
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    device_id UUID NOT NULL,
    external_id VARCHAR(50) NOT NULL,
    expires_t TIMESTAMPTZ NOT NULL DEFAULT (CURRENT_TIMESTAMP + INTERVAL '6 minutes'),
    PRIMARY KEY (user_id, device_id)
);