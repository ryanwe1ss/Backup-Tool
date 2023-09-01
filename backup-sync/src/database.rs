use rusqlite::{Connection, Result as SqliteResult};

pub fn create_database() -> SqliteResult<()> {
  let database = Connection::open("database.db").unwrap();

  database.execute(
    "CREATE TABLE files (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      filename TEXT,
      date_created TEXT DEFAULT (CURRENT_TIMESTAMP)
    
    )", [],
  )?;

  database.execute(
    "CREATE TABLE hashes (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      file_id INTEGER,
      hash TEXT(256),
      date_created TEXT DEFAULT (CURRENT_TIMESTAMP),
      FOREIGN KEY (file_id) REFERENCES files(id)
    
    )", [],
  )?;

  database.close().unwrap();
  Ok(())
}

pub fn insert_file(filename: &str) -> SqliteResult<i32> {
  let database = Connection::open("database.db").unwrap();

  database.execute(
    "INSERT INTO files (filename) VALUES (?1)",
    &[&filename],
  )?;

  let file_id = database.last_insert_rowid() as i32;
  database.close().unwrap();
  Ok(file_id)
}

pub fn insert_hash(file_id: i32, hash: &str) -> SqliteResult<()> {
  let database = Connection::open("database.db").unwrap();

  database.execute(
    "INSERT INTO hashes (file_id, hash) VALUES (?1, ?2)",
    &[&file_id as &dyn rusqlite::types::ToSql, &hash],
  )?;

  database.close().unwrap();
  Ok(())
}