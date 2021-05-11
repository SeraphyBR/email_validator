-- Add migration script here
CREATE TABLE email_data_v3 (
    ID            SERIAL      PRIMARY KEY,
    email_address VARCHAR     NOT NULL,
    domain        VARCHAR     NOT NULL,
    valid_syntax  BOOLEAN     NOT NULL,
    disposable    BOOLEAN     NOT NULL,
    webmail       BOOLEAN     NOT NULL,
    deliverable   BOOLEAN     NOT NULL,
    catch_all     BOOLEAN     NOT NULL,
    gibberish     BOOLEAN     NOT NULL,
    spam          BOOLEAN     NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);