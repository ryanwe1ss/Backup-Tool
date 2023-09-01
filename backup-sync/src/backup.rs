use std::fs::File;
use std::io::{Read, Write, Result};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hasher, Hash};

use crate::database;
const CHUNK_SIZE: usize = 1_000_000;

pub fn local_backup(files_to_backup: Vec<&str>, backup_path: &str) {
  let start = std::time::Instant::now();
  database::create_database();

  for file_path in files_to_backup {
      copy_file_in_chunks(file_path, backup_path);
  }

  let end = std::time::Instant::now();
  println!("Backup took: {:.3} seconds", (end - start).as_secs_f64());
}

fn copy_file_in_chunks(file_path: &str, backup_path: &str) -> Result<()> {
  let mut file = File::open(file_path)?;
  let mut backup_file = File::create(backup_path.to_owned() + file_path)?;
  let mut buffer = vec![0; CHUNK_SIZE];
  let mut hasher = DefaultHasher::new();

  let file_id = database::insert_file(file_path).unwrap();

  loop {
    let bytes_read = file.read(&mut buffer)?;
    if (bytes_read == 0) {
        break;
    }
    backup_file.write_all(&buffer[..bytes_read])?;

    hasher.write(&buffer[..bytes_read]);
    let chunk_hash = hasher.finish();
    database::insert_hash(file_id, chunk_hash.to_string().as_str()).unwrap();
    hasher = DefaultHasher::new();
  }

  Ok(())
}