-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

create table users (
  id uuid default uuid_generate_v4() PRIMARY KEY,
  username varchar not null UNIQUE,
  email varchar not null,
  salt varchar not null,
  password_hash varchar not null,
  full_name varchar NULL,
  bio varchar NULL,
  image varchar NULL,
  -- email_verified
  -- active
  created_at timestamp not NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at timestamp not NULL DEFAULT CURRENT_TIMESTAMP
);