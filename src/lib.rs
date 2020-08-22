use std::fs::File;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug)]
struct FieldType(char);

#[derive(Debug)]
pub struct Field {
    name: String,
    length: u8,
    number_of_decimal_places: u8,
    flag: u8,
    field_type: FieldType,
}

pub struct Database {
    path: String,
    version: u8,
    record_count: u32,
    first_record_offset: u32,
    record_length: u32,
    pub fields: Vec<Field>,
}

impl Database {
    pub fn new(path: String) -> Database {
        Database {
            path,
            version: 0,
            record_count: 0,
            first_record_offset: 0,
            record_length: 0,
            fields: Vec::new(),
        }
    }

    pub fn initialize(&mut self) -> std::io::Result<()> {
        self.parse_headers()
    }

    fn open(&self) -> std::io::Result<std::fs::File> {
        println!("Opening file at path: {}", &self.path);
        File::open(&self.path)
    }

    fn parse_subrecords(&mut self, reader: &mut BufReader<File>) -> std::io::Result<()> {
        let mut subrecord = [0; 32];

        loop {
            reader.read_exact(&mut subrecord)?;
            if subrecord[0] == 0x0D {
                println!("found end of subrecords");
                return Ok(());
            }

            let name = std::str::from_utf8(&subrecord[0..10]);
            if name.is_err() {
                return Ok(());
            }
            let field_type = subrecord[11];
            // let field_offset = &subrecord[12..15];
            let field_length = subrecord[16];
            let number_of_decimal_places = subrecord[17];
            let flag = subrecord[18];
            self.fields.push(Field {
                name: String::from(name.unwrap().trim_end_matches(char::from(0))),
                field_type: FieldType(char::from(field_type)),
                length: field_length,
                number_of_decimal_places,
                flag,
            });
        }
    }

    fn parse_headers(&mut self) -> std::io::Result<()> {
        let file = self.open()?;
        let mut reader = BufReader::new(file);

        let mut version = [0; 1];
        reader.read(&mut version)?;
        self.version = version[0];
        println!("Version: {:?}", &self.version);

        // file last updated at
        reader.consume(3);

        let mut record_count = [0; 4];
        reader.read(&mut record_count)?;
        self.record_count = u32::from_le_bytes(record_count);
        println!("Number of records: {:?}", &self.record_count);

        let mut first_record_offset = [0; 2];
        reader.read(&mut first_record_offset)?;
        self.first_record_offset = u16::from_le_bytes(first_record_offset).into();
        println!("First record offset: {:?}", &self.first_record_offset);

        let mut record_length = [0; 2];
        reader.read(&mut record_length)?;
        self.record_length = u16::from_le_bytes(record_length).into();
        println!("Record length: {:?}", &self.record_length);

        // some reserved bytes
        reader.consume(16);
        let mut flag = [0; 1];
        reader.read(&mut flag)?;
        println!("Flag: {:?}", flag);

        // more unneeded
        reader.consume(3);

        self.parse_subrecords(&mut reader)
    }
}
