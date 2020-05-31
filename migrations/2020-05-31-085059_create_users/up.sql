-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

create table if not exists users (
    id UUID primary key default uuid_generate_v4(),
    username varchar unique not null,
    email varchar unique not null,
    password varchar not null
);