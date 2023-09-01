#[allow(warnings)]
pub mod backup;
pub mod database;

fn main() {
  let files_to_backup = vec!["enwik9.txt"];
  let backup_path = "backup/";

  backup::local_backup(files_to_backup, backup_path);
}