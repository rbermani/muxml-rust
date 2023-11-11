use crate::score::ScoreBuilder;
use crate::ser::encode_muxml;
use normalize_line_endings::normalized;
use std::fs;
use std::iter::FromIterator;
use std::path::PathBuf;

#[test]
fn test_encode_simple_musicxml() {
    let simple_data_path = PathBuf::from("test/data/simple.musicxml");
    let simple_data_string = &String::from_iter(normalized(
        fs::read_to_string(simple_data_path).unwrap().chars(),
    ));

    let score = ScoreBuilder::new()
        .work_title("Eine kleine Nachtmusik".to_string())
        .composer("Mozart".to_string())
        .software("muxml rust crate".to_string())
        .encoding_date("08/22/1983".to_string())
        .build();

    let output_xml = &String::from_iter(normalized(encode_muxml(score).chars()));
    //println!("{}", output_xml);
    //assert_eq!(true, true);
    assert_eq!(output_xml, simple_data_string.as_str());
}
