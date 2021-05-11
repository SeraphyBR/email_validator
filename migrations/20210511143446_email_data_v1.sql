-- Add migration script here
CREATE TABLE email_data_v1 (
    ID            SERIAL      PRIMARY KEY,
    email_address VARCHAR     NOT NULL,
    domain        VARCHAR     NOT NULL,
    valid_syntax  BOOLEAN     NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
