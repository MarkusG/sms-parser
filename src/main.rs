extern crate xml;

use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

use sms_parser::{Sms, Mms};

fn main() {
    let file = File::open("sms.xml").unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut mms = Mms::blank();
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                match name.local_name.as_str() {
                    "sms" => {
                        // let sms = Sms::new(&attributes);
                        // println!("{:?}", sms);
                    },
                    "mms" => {
                        mms = Mms::new(&attributes).unwrap();
                    },
                    "part" => {
                        mms.add_part(&attributes);
                    },
                    "addr" => {
                        mms.add_addr(&attributes);
                    },
                    _ => ()
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name == "mms" {
                    print!("{} ", mms.timestamp);
                    for a in mms.addresses {
                        print!("{} ", a);
                    }
                    println!();
                    for p in mms.parts {
                        if p.content_type == "text/plain" {
                            println!("{:?}", p);
                        }
                    }
                    mms = Mms::blank();
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}
