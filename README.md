# muxml-rust

Serialization support for MusicXML in Rust

A Rust library for serializing MusicXML files into Rust native data types.

It is not intended to be comprehensive. The goal is to preserve the most informationally relevant elements from the MusicXML, rather than the complete engraving and formatting information.

To that end, this library may not meet the needs of your applications.

It is part of a larger project (non-open source) related to applying machine learning to music data.

The original release on crates.io (which was yanked) included deserialization support as well. However, that implementation was asymmetric in that the deserialization process used the roxml crate to extract only a subset of relevant data from the input XML to encode it into an intermediate data format.

The goal here is for the final deserialization implementation to use serde/quick-xml for performing the deserialization.

## TODO:
- [x] Add support for encoding single part grand staff
- [x] Make error handling system less intrusive with existing consumers
- [ ] Implement deserialization support
- [ ] Add rustdoc documentation
- [ ] Implement comprehensive unit tests
- [ ] Add support for unpitched note elements / percussion
- [ ] Add support for various MusicXML elements

**Note: This project is in pre-Alpha state and subject to frequent & major breaking API changes.**

## Installation
### Requirements
- Rust 1.56+


### Importing
This crate is available on [crates.io](https://crates.io/crates/muxml). To use it in your application, simply add it to your project's `Cargo.toml`.

```toml
[dependencies]
muxml = "0.1.1"
```

## Usage

```
### Serialize from structs to MusicXML

```rust
use muxml::de::xml_to_ir;
use muxml::error;
use muxml::error::Result;
use muxml::ser::ir_to_xml;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub fn process_xml_multipart(input: PathBuf, output: PathBuf, dump_input: bool) -> Result<()> {
    let outfile = File::create(output).expect("IO Error Occurred");
    let mut writer = BufWriter::new(outfile);

    let score = ScoreBuilder::new()
        .work_title("Eine kleine Nachtmusik".to_string())
        .composer("Mozart".to_string())
        .software("muxml rust crate".to_string())
        .encoding_date("08/22/1983".to_string())
        .build();

    let output_xml = encode_muxml(score);
    writer
        .write_all(output_xml.as_bytes())
        .expect("IO Error occurred on write_all()");
    writer
        .flush()
        .map_err(|e| error::Error::IoKind(e.kind().to_string()))?;

    Ok(())
}

```
