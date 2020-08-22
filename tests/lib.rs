extern crate rusdbf;
#[cfg(test)]
mod tests {
  use rusdbf;
  use std::path::{Path, PathBuf};

  fn get_test_fixture(path: &str) -> PathBuf {
    let directory = Path::new(file!()).parent().unwrap();

    directory.join("fixtures").join(path)
  }

  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
    let fixture_path = get_test_fixture("dbase_03.dbf");

    let mut database = rusdbf::Database::new(fixture_path.to_string_lossy().to_string());
    database.initialize().unwrap();
    for field in database.fields.iter() {
      println!("{:?}", &field);
    }
  }
}
