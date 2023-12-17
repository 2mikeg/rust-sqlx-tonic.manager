-- Add up migration script here
-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE settlements (
        id uuid default uuid_generate_v1() PRIMARY KEY NOT NULL,
        service_id uuid NOT NULL,
        quantity float8 NOT NULL,
        price float8 NOT NULL,
        amount float8 NOT NULL,
        created_at TIMESTAMPTZ NOT NULL default CURRENT_TIMESTAMP
    )