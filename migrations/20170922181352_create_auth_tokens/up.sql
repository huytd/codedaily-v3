CREATE TABLE auth_tokens (
  token VARCHAR NOT NULL PRIMARY KEY,
  user_id SERIAL NOT NULL,
  expired_at BIGINT NOT NULL
);
