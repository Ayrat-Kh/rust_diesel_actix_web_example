CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Your SQL goes here
CREATE TABLE users (
  id uuid DEFAULT uuid_generate_v4(),
  username VARCHAR(100),
  avatar VARCHAR(100),
  PRIMARY KEY(id)
);