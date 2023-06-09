CREATE TABLE backup_jobs (
  id VARCHAR NOT NULL PRIMARY KEY,
  name VARCHAR NOT NULL UNIQUE,
  src VARCHAR NOT NULL,
  dst VARCHAR NOT NULL UNIQUE,
  is_ready BOOLEAN NOT NULL DEFAULT 'f',
  hourly BOOLEAN NOT NULL DEFAULT 'f',
  daily BOOLEAN NOT NULL DEFAULT 'f',
  weekly BOOLEAN NOT NULL DEFAULT 'f',
  monthly BOOLEAN NOT NULL DEFAULT 'f'
);

CREATE TABLE logs(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  job_id VARCHAR,
  level INTEGER NOT NULL,
  message VARCHAR NOT NULL,
  created_at TIMESTAMP DEFAULT (strftime('%Y-%m-%d %H:%M:%S', 'now')),
  FOREIGN KEY (job_id) REFERENCES backup_jobs(id)
);