use quick_xml::se::Serializer;
use serde::Serialize;

use crate::muxml_types::{
    CreatorElement, EncodingElement, IdentificationElement, ScorePartWise, SupportsElement,
    WorkElement,
};
use crate::score::Score;

#[cfg(windows)]
pub const NL: &str = "\r\n";
#[cfg(not(windows))]
pub const NL: &str = "\n";

#[allow(dead_code)]
pub fn encode_muxml(score: Score) -> String {
    let item = ScorePartWise {
        version: score.version().to_string(),
        work: WorkElement {
            work_title: score.work_title().to_string(),
        },
        identification: IdentificationElement {
            creator: CreatorElement {
                r#type: "composer".to_string(),
                value: score.composer().to_string(),
            },
            encoding: EncodingElement {
                software: score.software().to_string(),
                encoding_date: score.encoding_date().to_string(),
                supports: vec![
                    SupportsElement {
                        element: "accidental".to_string(),
                        r#type: "yes".to_string(),
                    },
                    SupportsElement {
                        element: "beam".to_string(),
                        r#type: "yes".to_string(),
                    },
                ],
            },
        },
        part_list: score.complete_parts().part_list().clone(),
        part: score.complete_parts().part_elements().clone(),
    };
    let mut xml_string = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>{0}<!DOCTYPE score-partwise PUBLIC \"-//Recordare//DTD MusicXML 4.0 Partwise//EN\" \"http://www.musicxml.org/dtds/partwise.dtd\">{0}", NL);
    let mut ser = Serializer::new(&mut xml_string);
    ser.indent(' ', 2);
    item.serialize(ser).unwrap();
    xml_string
}
