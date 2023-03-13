-- Your SQL goes here
CREATE TABLE tweets (
  id uuid DEFAULT uuid_generate_v4(),
  message VARCHAR(300) NOT NULL,
  created_at TIMESTAMP NOT NULL,
  PRIMARY KEY(id)
);