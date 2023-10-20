use std::fs;
use std::io::{Read, Write, Result};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

use crate::database::{self, get_file_hashes};
const CHUNK_SIZE: usize = 1_000_000;

pub fn local_backup(files_to_backup: Vec<&str>, backup_path: &str) {
  let start = std::time::Instant::now();
  let mut file_exists_in_db = false;

  if (fs::metadata("tracking.db").is_err()) {
    database::create_database().unwrap();
  }

  for file_name in files_to_backup {
    file_exists_in_db = false;

    if (fs::metadata(backup_path.to_owned() + file_name).is_ok()) {
      let file_backup = backup_path.to_owned() + file_name;
      let hashes = file_has_changes(&file_name);

      if (hashes.len() > 0) {
        println!("{} has changes", file_backup);

        let file_size = fs::metadata(file_name).unwrap().len();
        file_exists_in_db = true;

        fs::remove_file(backup_path.to_owned() + file_name).unwrap();
        database::update_hashes_and_delete_file(file_name, file_size, hashes).unwrap();

      } else {
        println!("{} has no changes", file_backup);
        continue;
      }
    }

    copy_file_in_chunks(file_name, backup_path, file_exists_in_db);
  }

  let end = std::time::Instant::now();
  println!("\nBackup took: {:.3} seconds", (end - start).as_secs_f64());
}

fn copy_file_in_chunks(file_name: &str, backup_path: &str, file_exists_in_db: bool) -> Result<()> {
  let mut file = fs::File::open(file_name).expect("File not found");
  let mut backup_file = fs::File::create(backup_path.to_owned() + file_name).expect("Could not create backup file");

  let mut buffer = vec![0; CHUNK_SIZE];
  let mut hasher = DefaultHasher::new();
  let mut file_id = 0;

  if (!file_exists_in_db) {
    file_id = database::insert_file(
      file_name,
      file.metadata().unwrap().len()
    ).unwrap();
  }
  
  loop {
    let bytes_read = file.read(&mut buffer)?;
    if (bytes_read == 0) {
        break;
    }

    backup_file.write_all(&buffer[..bytes_read])?;

    if (!file_exists_in_db) {
      hasher.write(&buffer[..bytes_read]);
      database::insert_hash(file_id, hasher.finish().to_string().as_str()).unwrap();
      hasher = DefaultHasher::new();
    }
  }

  Ok(())
}

fn file_has_changes(file_name: &str) -> Vec<String> {
  let mut buffer = vec![0; CHUNK_SIZE];
  let mut hasher = DefaultHasher::new();
  let mut file = fs::File::open(file_name).expect("File not found");

  let db_hashes = get_file_hashes(file_name).unwrap();
  let mut file_hashes: Vec<String> = Vec::new();

  loop {
    let bytes_read = file.read(&mut buffer).unwrap();
    if (bytes_read == 0) {
        break;
    }

    hasher.write(&buffer[..bytes_read]);
    let hash = hasher.finish().to_string();
    hasher = DefaultHasher::new();

    file_hashes.push(hash);
  }

  println!("\ndb_hashes: {:?}", db_hashes);
  println!("fl_hashes: {:?}", file_hashes);

  if (db_hashes != file_hashes) {
    return file_hashes;
  }

  Vec::new()
}