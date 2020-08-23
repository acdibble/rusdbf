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
    let fixture_path = get_test_fixture("dbase_03.dbf");

    let mut database = rusdbf::Database::new(
      fixture_path.to_string_lossy().to_string(),
      "Point_ID".to_string(),
      rusdbf::FieldType::Character,
    );

    database.initialize();
    database.index_database();
    // for field in database.fields.iter() {
    //   println!("{:?}", &field);
    // }
  }

  #[test]
  fn it_also_works() {
    let fixture_path = get_test_fixture("dbase_83.dbf");

    let mut database = rusdbf::Database::new(
      fixture_path.to_string_lossy().to_string(),
      "ID".to_string(),
      rusdbf::FieldType::Numeric,
    );

    database.initialize();
    database.index_database();
    let record = database.get_by_id(93);
    println!("{:?}", record);
    // for field in database.fields.iter() {
    //   println!("{:?}", &field);
    // }
  }
}
