CREATE TABLE crates (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  url VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  version VARCHAR NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f'
)