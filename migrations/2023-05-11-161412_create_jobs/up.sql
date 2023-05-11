CREATE TABLE job (
  id VARCHAR NOT NULL PRIMARY KEY,
  name VARCHAR NOT NULL,
  src VARCHAR NOT NULL,
  dst VARCHAR NOT NULL,
  hourly BOOLEAN NOT NULL DEFAULT 'f',
  daily BOOLEAN NOT NULL DEFAULT 'f',
  weekly BOOLEAN NOT NULL DEFAULT 'f',
  monthly BOOLEAN NOT NULL DEFAULT 'f'
)