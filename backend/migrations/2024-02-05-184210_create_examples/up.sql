-- Your SQL goes here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "citext";

CREATE TABLE IF NOT EXISTS public.examples (
  id CITEXT DEFAULT uuid_generate_v1(),
  name CITEXT NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

  CONSTRAINT examples_pkey 
    PRIMARY KEY(id)
);
