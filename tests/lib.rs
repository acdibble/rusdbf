extern crate rusdbf;
#[cfg(test)]
mod tests {
    use rusdbf;
    use std::collections::HashMap;
    use std::path::{Path, PathBuf};

    fn get_test_fixture(path: &str) -> PathBuf {
        let directory = Path::new(file!()).parent().unwrap();

        directory.join("fixtures").join(path)
    }

    fn get_value(record: &mut Vec<(String, rusdbf::Value)>, field: &str) -> rusdbf::Value {
        let position = record
            .iter()
            .position(|(name, _value)| name == field)
            .expect("failed to find value in record");

        record.remove(position).1
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
        let mut record = database.get_by_id(5071232).expect("record is not none");
        assert_eq!(
            get_value(&mut record, "row_number"),
            rusdbf::Value::RowNumber(13)
        );
        assert_eq!(
            get_value(&mut record, "deleted"),
            rusdbf::Value::Deleted(false)
        );
        assert_eq!(
            get_value(&mut record, "Point_ID"),
            rusdbf::Value::Character(String::from("05071232"))
        );
        assert_eq!(
            get_value(&mut record, "Type"),
            rusdbf::Value::Character(String::from("CMP"))
        );
        assert_eq!(
            get_value(&mut record, "Shape"),
            rusdbf::Value::Character(String::from("circular"))
        );
        assert_eq!(
            get_value(&mut record, "Circular_D"),
            rusdbf::Value::Character(String::from("12"))
        );
        assert_eq!(
            get_value(&mut record, "Non_circul"),
            rusdbf::Value::Character(String::from(""))
        );
        assert_eq!(
            get_value(&mut record, "Flow_prese"),
            rusdbf::Value::Character(String::from("no"))
        );
        assert_eq!(
            get_value(&mut record, "Condition"),
            rusdbf::Value::Character(String::from("Plugged"))
        );
        assert_eq!(
            get_value(&mut record, "Comments"),
            rusdbf::Value::Character(String::from(""))
        );
        assert_eq!(
            get_value(&mut record, "Date_Visit"),
            rusdbf::Value::Date(String::from("20050712"))
        );
        assert_eq!(
            get_value(&mut record, "Time"),
            rusdbf::Value::Character(String::from("12:55:47pm"))
        );
        assert_eq!(
            get_value(&mut record, "Max_PDOP"),
            rusdbf::Value::Numeric(3.5)
        );
        assert_eq!(
            get_value(&mut record, "Max_HDOP"),
            rusdbf::Value::Numeric(1.7)
        );
        assert_eq!(
            get_value(&mut record, "Corr_Type"),
            rusdbf::Value::Character(String::from("Postprocessed Code"))
        );
        assert_eq!(
            get_value(&mut record, "Rcvr_Type"),
            rusdbf::Value::Character(String::from("GeoXT"))
        );
        assert_eq!(
            get_value(&mut record, "GPS_Date"),
            rusdbf::Value::Date(String::from("20050712"))
        );
        assert_eq!(
            get_value(&mut record, "GPS_Time"),
            rusdbf::Value::Character(String::from("12:55:47pm"))
        );
        assert_eq!(
            get_value(&mut record, "Update_Sta"),
            rusdbf::Value::Character(String::from("New"))
        );
        assert_eq!(
            get_value(&mut record, "Feat_Name"),
            rusdbf::Value::Character(String::from("Driveway"))
        );
        assert_eq!(
            get_value(&mut record, "Datafile"),
            rusdbf::Value::Character(String::from("050712TR2819.cor"))
        );
        assert_eq!(
            get_value(&mut record, "Unfilt_Pos"),
            rusdbf::Value::Numeric(2.0)
        );
        assert_eq!(
            get_value(&mut record, "Filt_Pos"),
            rusdbf::Value::Numeric(2.0)
        );
        assert_eq!(
            get_value(&mut record, "Data_Dicti"),
            rusdbf::Value::Character(String::from("MS4"))
        );
        assert_eq!(
            get_value(&mut record, "GPS_Week"),
            rusdbf::Value::Numeric(1331.0)
        );
        assert_eq!(
            get_value(&mut record, "GPS_Second"),
            rusdbf::Value::Numeric(233760.0)
        );
        assert_eq!(
            get_value(&mut record, "GPS_Height"),
            rusdbf::Value::Numeric(1101.939)
        );
        assert_eq!(
            get_value(&mut record, "Vert_Prec"),
            rusdbf::Value::Numeric(2.1)
        );
        assert_eq!(
            get_value(&mut record, "Horz_Prec"),
            rusdbf::Value::Numeric(1.1)
        );
        assert_eq!(
            get_value(&mut record, "Std_Dev"),
            rusdbf::Value::Numeric(1.223112)
        );
        assert_eq!(
            get_value(&mut record, "Northing"),
            rusdbf::Value::Numeric(559870.4)
        );
        assert_eq!(
            get_value(&mut record, "Easting"),
            rusdbf::Value::Numeric(2213662.0)
        );
        assert_eq!(
            get_value(&mut record, "Point_ID"),
            rusdbf::Value::Numeric(432.0)
        );

        assert!(database.get_by_id(1000).is_none());
        assert!(database.get_by_id(507121).is_some());
        assert!(database.get_by_id(507122).is_some());
        assert!(database.get_by_id(507123).is_some());
        assert!(database.get_by_id(507125).is_some());
        assert!(database.get_by_id(5071210).is_some());
        assert!(database.get_by_id(5071216).is_some());
        assert!(database.get_by_id(5071217).is_some());
        assert!(database.get_by_id(5071219).is_some());
        assert!(database.get_by_id(5071224).is_some());
        assert!(database.get_by_id(5071225).is_some());
        assert!(database.get_by_id(5071229).is_some());
        assert!(database.get_by_id(5071231).is_some());
        assert!(database.get_by_id(5071232).is_some());
        assert!(database.get_by_id(5071236).is_some());
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
        let record = database.get_by_id(93).expect("record is not none");
        let record: HashMap<String, rusdbf::Value> = record.into_iter().collect();
        assert_eq!(record.get("DESC").unwrap(), &rusdbf::Value::Memo(77));
        assert_eq!(
            record.get("THUMBNAIL").unwrap(),
            &rusdbf::Value::Character(String::from("graphics/00000001/t_D-1001.jpg"))
        );
        assert_eq!(
            record.get("row_number").unwrap(),
            &rusdbf::Value::RowNumber(66)
        );
        assert_eq!(record.get("PRICE").unwrap(), &rusdbf::Value::Numeric(28.95));
        assert_eq!(record.get("ACTIVE").unwrap(), &rusdbf::Value::Logical(true));
        assert_eq!(
            record.get("AGRPCOUNT").unwrap(),
            &rusdbf::Value::Numeric(0.0)
        );
        assert_eq!(record.get("COST").unwrap(), &rusdbf::Value::Numeric(0.0));
        assert_eq!(record.get("ID").unwrap(), &rusdbf::Value::Numeric(93.0));
        assert_eq!(
            record.get("TAXABLE").unwrap(),
            &rusdbf::Value::Logical(false)
        );
        assert_eq!(
            record.get("IMAGE").unwrap(),
            &rusdbf::Value::Character(String::from("graphics/00000001/D-1001.jpg"))
        );
        assert_eq!(record.get("ORDER").unwrap(), &rusdbf::Value::Numeric(93.0));
        assert_eq!(
            record.get("CATCOUNT").unwrap(),
            &rusdbf::Value::Numeric(2.0)
        );
        assert_eq!(
            record.get("PGRPCOUNT").unwrap(),
            &rusdbf::Value::Numeric(0.0)
        );
        assert_eq!(
            record.get("CODE").unwrap(),
            &rusdbf::Value::Character(String::from("D-1001"))
        );
        assert_eq!(record.get("WEIGHT").unwrap(), &rusdbf::Value::Numeric(0.0));
        assert_eq!(
            record.get("deleted").unwrap(),
            &rusdbf::Value::Deleted(false)
        );
        assert_eq!(
            record.get("NAME").unwrap(),
            &rusdbf::Value::Character(String::from("Demitasse Christmas Petits Fours"))
        );

        assert!(database.get_by_id(10000).is_none());
        assert!(database.get_by_id(87).is_some());
        assert!(database.get_by_id(26).is_some());
        assert!(database.get_by_id(27).is_some());
        assert!(database.get_by_id(28).is_some());
        assert!(database.get_by_id(29).is_some());
        assert!(database.get_by_id(30).is_some());
        assert!(database.get_by_id(31).is_some());
        assert!(database.get_by_id(32).is_some());
        assert!(database.get_by_id(33).is_some());
        assert!(database.get_by_id(34).is_some());
        assert!(database.get_by_id(35).is_some());
        assert!(database.get_by_id(36).is_some());
        assert!(database.get_by_id(37).is_some());
        assert!(database.get_by_id(38).is_some());
        assert!(database.get_by_id(39).is_some());
        assert!(database.get_by_id(40).is_some());
        assert!(database.get_by_id(41).is_some());
        assert!(database.get_by_id(42).is_some());
        assert!(database.get_by_id(43).is_some());
        assert!(database.get_by_id(44).is_some());
        assert!(database.get_by_id(45).is_some());
        assert!(database.get_by_id(46).is_some());
        assert!(database.get_by_id(47).is_some());
        assert!(database.get_by_id(48).is_some());
        assert!(database.get_by_id(49).is_some());
        assert!(database.get_by_id(50).is_some());
        assert!(database.get_by_id(51).is_some());
        assert!(database.get_by_id(52).is_some());
        assert!(database.get_by_id(53).is_some());
        assert!(database.get_by_id(54).is_some());
        assert!(database.get_by_id(55).is_some());
        assert!(database.get_by_id(56).is_some());
        assert!(database.get_by_id(57).is_some());
        assert!(database.get_by_id(58).is_some());
        assert!(database.get_by_id(59).is_some());
        assert!(database.get_by_id(60).is_some());
        assert!(database.get_by_id(61).is_some());
        assert!(database.get_by_id(62).is_some());
        assert!(database.get_by_id(63).is_some());
        assert!(database.get_by_id(64).is_some());
        assert!(database.get_by_id(65).is_some());
        assert!(database.get_by_id(66).is_some());
        assert!(database.get_by_id(67).is_some());
        assert!(database.get_by_id(69).is_some());
        assert!(database.get_by_id(70).is_some());
        assert!(database.get_by_id(71).is_some());
        assert!(database.get_by_id(72).is_some());
        assert!(database.get_by_id(73).is_some());
        assert!(database.get_by_id(74).is_some());
        assert!(database.get_by_id(75).is_some());
        assert!(database.get_by_id(76).is_some());
        assert!(database.get_by_id(77).is_some());
        assert!(database.get_by_id(78).is_some());
        assert!(database.get_by_id(79).is_some());
        assert!(database.get_by_id(80).is_some());
        assert!(database.get_by_id(81).is_some());
        assert!(database.get_by_id(82).is_some());
        assert!(database.get_by_id(83).is_some());
        assert!(database.get_by_id(84).is_some());
        assert!(database.get_by_id(85).is_some());
        assert!(database.get_by_id(86).is_some());
        assert!(database.get_by_id(88).is_some());
        assert!(database.get_by_id(89).is_some());
        assert!(database.get_by_id(90).is_some());
        assert!(database.get_by_id(91).is_some());
        assert!(database.get_by_id(93).is_some());
        assert!(database.get_by_id(94).is_some());
    }
}
