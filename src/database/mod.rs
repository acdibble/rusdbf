mod field_type;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};

pub use crate::database::field_type::*;

#[derive(Debug)]
pub struct Field {
  name: String,
  length: usize,
  decimal_precision: u8,
  flag: u8,
  field_type: FieldType,
  is_primary: bool,
}

impl FieldType {}

pub struct Database {
  path: String,
  version: u8,
  record_count: u32,
  first_record_offset: u32,
  record_length: usize,
  pub fields: Vec<Field>,
  primary_key: String,
  primary_type: FieldType,
}

impl Database {
  pub fn new(path: String, primary_key: String, primary_type: FieldType) -> Database {
    Database {
      path,
      version: 0,
      record_count: 0,
      first_record_offset: 0,
      record_length: 0,
      fields: Vec::new(),
      primary_key,
      primary_type,
    }
  }

  pub fn initialize(&mut self) {
    self.parse_headers()
  }

  fn open(&self) -> std::fs::File {
    println!("Opening file at path: {}", &self.path);
    File::open(&self.path).expect("failed to open DBF file")
  }

  fn parse_subrecords(&mut self, reader: &mut BufReader<File>) {
    let mut subrecord = [0; 32];

    loop {
      reader
        .read_exact(&mut subrecord)
        .expect("failed to read data for subrecord");
      let name = std::str::from_utf8(&subrecord[0..10]);
      if subrecord[0] == 0x0D || name.is_err() {
        println!("found end of subrecords");
        return;
      }

      let name = String::from(name.unwrap().trim_end_matches(char::from(0)));
      let field_type = FieldType::from_char(char::from(subrecord[11]));
      let is_primary = name == self.primary_key;
      let field_length = subrecord[16];
      let decimal_precision = subrecord[17];
      let flag = subrecord[18];
      let new_field = Field {
        name,
        is_primary: is_primary && field_type == self.primary_type,
        field_type,
        length: field_length as usize,
        decimal_precision,
        flag,
      };

      self.fields.push(new_field);
    }
  }

  fn parse_headers(&mut self) {
    let file = self.open();
    let mut reader = BufReader::new(file);

    let mut version = [0; 1];
    reader.read(&mut version).expect("failed to read version");
    self.version = version[0];
    println!("Version: {:?}", &self.version);

    // file last updated at
    reader.consume(3);

    let mut record_count = [0; 4];
    reader
      .read(&mut record_count)
      .expect("failed to read record count");
    self.record_count = u32::from_le_bytes(record_count);
    println!("Number of records: {:?}", &self.record_count);

    let mut first_record_offset = [0; 2];
    reader
      .read(&mut first_record_offset)
      .expect("failed to read offset for first file");
    self.first_record_offset = u16::from_le_bytes(first_record_offset).into();
    println!("First record offset: {:?}", &self.first_record_offset);

    let mut record_length = [0; 2];
    reader
      .read(&mut record_length)
      .expect("failed to read length of one record");
    self.record_length = u16::from_le_bytes(record_length).into();
    println!("Record length: {:?}", &self.record_length);

    // some reserved bytes
    reader.consume(16);
    let mut flag = [0; 1];
    reader.read(&mut flag).expect("fail to read table flag");
    println!("Flag: {:?}", flag);

    // more unneeded
    reader.consume(3);

    self.parse_subrecords(&mut reader)
  }

  pub fn parse_record(&self, buffer: &Vec<u8>) -> HashMap<&String, Value> {
    let mut position = 0;
    let slice = buffer.as_slice();
    let mut record = HashMap::default();

    for field in &self.fields {
      let data_for_field = &slice[position..(position + field.length)];
      position += field.length;
      record.insert(&field.name, field.field_type.to_value(data_for_field));
    }

    return record;
  }

  pub fn iterate_all_records(&self) {
    let mut file = self.open();
    file
      .seek(SeekFrom::Start(self.first_record_offset as u64))
      .expect("failed to jump to first record");
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::with_capacity(self.record_length);

    for _ in 0..self.record_count {
      reader
        .by_ref()
        .take(self.record_length as u64)
        .read_to_end(&mut buffer)
        .expect("failed to read record data");

      let record = self.parse_record(&buffer);
      println!("{:?}", record);
    }
  }
}
