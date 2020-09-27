extern crate xml;

use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

use sms_parser::Sms;

fn main() {
    let file = File::open("sms.xml").unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                if name.local_name != "sms" {
                    continue;
                }

                let sms = Sms::new(&attributes);
                println!("{:?}", sms);
            }
            // Ok(XmlEvent::EndElement { name }) => {
            // }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}
