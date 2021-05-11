-- Add migration script here
CREATE TABLE email_data_v3 (
    ID            SERIAL  PRIMARY KEY,
    email_address VARCHAR NOT NULL,
    domain        VARCHAR NOT NULL,
    valid_syntax  BOOLEAN NOT NULL,
    disposable    BOOLEAN,
    webmail       BOOLEAN,
    derivable     BOOLEAN,
    catch_all     BOOLEAN,
    gibberish     BOOLEAN,
    spam          BOOLEAN,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);