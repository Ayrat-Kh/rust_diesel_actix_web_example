-- Your SQL goes here
CREATE TABLE photos (
  id uuid DEFAULT uuid_generate_v4(),
  user_id uuid NOT NULL, 
  url VARCHAR(250),
  PRIMARY KEY (id),
  CONSTRAINT fk_user
    FOREIGN KEY(user_id)
      REFERENCES users(id)
)