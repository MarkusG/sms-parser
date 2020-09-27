extern crate xml;

use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

fn strip_chars(src: &str, chars: &str) -> String {
    src.chars().filter(|&c| !chars.contains(c)).collect()
}

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
                let mut timestamp = String::from("");
                let mut ts_readable = String::from("");
                let mut in_out = String::from("");
                let mut addr = String::from("");
                let mut contact = String::from("");
                let mut body = String::from("");
                for a in attributes {
                    match a.name.local_name.as_str() {
                        "date" => timestamp = a.value,
                        "readable_date" => ts_readable = a.value,
                        "type" => in_out = match a.value.as_str() {
                            "1" => String::from("I"),
                            "2" => String::from("O"),
                            _ => String::from("X")
                        },
                        "address" => addr = strip_chars(&a.value, "()+- "),
                        "contact_name" => contact = a.value,
                        "body" => body = a.value,
                        _ => ()
                    }
                }
                println!("{} {} {} {:14} {:20} {}",
                         timestamp,
                         ts_readable,
                         in_out, 
                         addr,
                         contact,
                         body);
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
