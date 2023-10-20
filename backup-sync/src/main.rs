#[allow(warnings)]
mod backup;
mod database;

fn main() {
  let files_to_backup = vec![];
  let backup_path = "backup/";

  backup::local_backup(files_to_backup, backup_path);
}