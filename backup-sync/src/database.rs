use rusqlite::{params, Connection, Result as SqliteResult};

pub fn create_database() -> SqliteResult<()> {
  let database = Connection::open("tracking.db").unwrap();

  database.execute(
    "CREATE TABLE files (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      file_name TEXT,
      size_in_bytes INTEGER,
      date_created TEXT DEFAULT (CURRENT_TIMESTAMP)
    
    )", [],
  )?;

  database.execute(
    "CREATE TABLE hashes (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      file_id INTEGER,
      hash TEXT,
      FOREIGN KEY (file_id) REFERENCES files(id)
    
    )", [],
  )?;

  database.execute(
    "CREATE TABLE backups (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      version INTEGER,
      description TEXT,
      date_created TEXT DEFAULT (CURRENT_TIMESTAMP)
    
    )", [],
  )?;

  database.execute(
    "CREATE TABLE backup_files (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      backup_id INTEGER,
      file_id INTEGER,
      FOREIGN KEY (backup_id) REFERENCES backups(id),
      FOREIGN KEY (file_id) REFERENCES files(id)
    
    )", [],
  )?;

  database.close().unwrap();
  Ok(())
}

pub fn insert_file(file_name: &str, file_size: u64) -> SqliteResult<i32> {
  let database = Connection::open("tracking.db").unwrap();

  database.execute(
    "DELETE FROM hashes WHERE file_id IN (SELECT id FROM files WHERE file_name = ?)",
    params![file_name],
  )?;

  database.execute(
    "DELETE FROM files WHERE file_name = ?",
    params![file_name],
  )?;

  database.execute(
    "INSERT INTO files (file_name, size_in_bytes) VALUES (?, ?)",
    params![file_name, file_size as i64],
  )?;

  let file_id = database.last_insert_rowid() as i32;
  database.close().unwrap();
  Ok(file_id)
}

pub fn insert_hash(file_id: i32, hash: &str) -> SqliteResult<()> {
  let database = Connection::open("tracking.db").unwrap();

  database.execute(
    "INSERT INTO hashes (file_id, hash) VALUES (?1, ?2)",
    &[&file_id as &dyn rusqlite::types::ToSql, &hash],
  )?;

  database.close().unwrap();
  Ok(())
}

pub fn update_hashes_and_delete_file(file_name: &str, file_size: u64, hashes: Vec<String>) -> SqliteResult<()> {
  let mut database = Connection::open("tracking.db").unwrap();
  let transaction = database.transaction()?;

  transaction.execute(
      "DELETE FROM hashes WHERE file_id IN (SELECT id FROM files WHERE file_name = ?)",
      params![file_name],
  )?;

  for hash in &hashes {
      transaction.execute(
          "INSERT INTO hashes (file_id, hash) VALUES ((SELECT id FROM files WHERE file_name = ?), ?)",
          params![file_name, hash],
      )?;
  }

  transaction.execute(
      "UPDATE files SET size_in_bytes = ? WHERE file_name = ?",
      params![file_size as i64, file_name],
  )?;

  transaction.commit()?;
  Ok(())
}

pub fn get_file_hashes(file_name: &str) -> SqliteResult<Vec<String>> {
  let database = Connection::open("tracking.db")?;

  let mut statement = database.prepare(
      "SELECT
          hash
       FROM
          hashes
       LEFT JOIN
          files ON files.id = hashes.file_id
       WHERE
          file_name = ?",
  )?;

  let mut rows = statement.query(params![file_name])?;
  let mut hashes = Vec::new();

  while let Some(row) = rows.next()? {
      hashes.push(row.get(0)?);
  }

  Ok(hashes)
}

pub fn delete_file(file_name: &str) -> SqliteResult<()> {
  let database = Connection::open("tracking.db").unwrap();

  database.execute(
    "DELETE FROM hashes WHERE file_id IN (SELECT id FROM files WHERE file_name = ?)",
    params![file_name],
  )?;

  database.execute(
    "DELETE FROM files WHERE file_name = ?",
    params![file_name],
  )?;

  database.close().unwrap();
  Ok(())
}