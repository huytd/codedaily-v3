CREATE TABLE votes (
  id SERIAL PRIMARY KEY,
  link_id INT NOT NULL,
  user_id INT NOT NULL,
  dir SMALLINT NOT NULL,
  UNIQUE (link_id, user_id)
);
