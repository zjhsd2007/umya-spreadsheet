use std::result;
use quick_xml::Reader;
use quick_xml::events::{Event};
use tempdir::TempDir;
use super::XlsxError;

use super::super::structs::spreadsheet::Spreadsheet;

const FILE_PATH: &'static str = "docProps/core.xml";

pub(crate) fn read(dir: &TempDir, spreadsheet:&mut Spreadsheet) -> result::Result<(), XlsxError> {
    let path = dir.path().join(FILE_PATH);
    let mut reader = Reader::from_file(path)?;
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut string_value: String = String::from("");
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Text(e)) => string_value = e.unescape_and_decode(&reader).unwrap(),
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"dc:title" => {spreadsheet.get_properties_mut().set_title(string_value.clone());},
                    b"dc:subject" => {spreadsheet.get_properties_mut().set_subject(string_value.clone());},
                    b"dc:creator" => {spreadsheet.get_properties_mut().set_creator(string_value.clone());},
                    b"cp:keywords" => {spreadsheet.get_properties_mut().set_keywords(string_value.clone());},
                    b"dc:description" => {spreadsheet.get_properties_mut().set_description(string_value.clone());},
                    b"cp:lastModifiedBy" => {spreadsheet.get_properties_mut().set_last_modified_by(string_value.clone());},
                    b"cp:revision" => {spreadsheet.get_properties_mut().set_revision(string_value.clone());},
                    b"dcterms:created" => {spreadsheet.get_properties_mut().set_created(string_value.clone());},
                    b"dcterms:modified" => {spreadsheet.get_properties_mut().set_modified(string_value.clone());},
                    b"cp:category" => {spreadsheet.get_properties_mut().set_category(string_value.clone());},
                    b"cp:version" => {spreadsheet.get_properties_mut().set_version(string_value.clone());},
                _ => (),
                }
                string_value = String::from("");
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
    Ok(())
}