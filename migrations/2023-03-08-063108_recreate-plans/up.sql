-- Your SQL goes here
CREATE TABLE plans (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  title VARCHAR NOT NULL,
  from_hr INTEGER,
  from_min INTEGER,
  to_hr INTEGER,
  to_min INTEGER,
  started BOOLEAN
);
