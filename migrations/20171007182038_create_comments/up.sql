CREATE TABLE comments (
  id SERIAL PRIMARY KEY,
  message VARCHAR NOT NULL,
  link_id INTEGER NOT NULL REFERENCES links (id),
  author_id INTEGER NOT NULL REFERENCES users (id),
  time INT NOT NULL,
);
