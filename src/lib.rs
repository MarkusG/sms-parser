use std::{error::Error, fmt};

use xml::attribute::OwnedAttribute;

#[derive(Debug)]
pub enum SmsError {
    InvalidDirection
}

impl Error for SmsError {}

impl fmt::Display for SmsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SmsError::InvalidDirection => write!(f, "Invalid direction (incoming/outgoing)")
        }
    }
}

fn strip_chars(src: &str, chars: &str) -> String {
    src.chars().filter(|&c| !chars.contains(c)).collect()
}

#[derive(Debug)]
pub struct Sms {
    timestamp: u64,
    outgoing: bool,
    address: String,
    contact: Option<String>,
    body: String
}

impl Sms {
    pub fn new(attributes: &[OwnedAttribute]) -> Result<Sms, Box<dyn Error>> {
        let mut result = Sms { 
            timestamp: 0,
            outgoing: false,
            address: "".to_string(),
            contact: None,
            body: "".to_string() };
        for a in attributes {
            match a.name.local_name.as_str() {
                "date" => result.timestamp = a.value.parse::<u64>()?,
                "type" => result.outgoing = match a.value.as_str() {
                    "1" => false,
                    "2" => true,
                    _ => return Err(Box::new(SmsError::InvalidDirection))
                },
                "address" => result.address = strip_chars(&a.value, "()+- "),
                "contact_name" => result.contact = match a.value.as_str() {
                    "(Unknown" => None,
                    _ => Some(a.value.clone())
                },
                "body" => result.body = a.value.clone(),
                _ => ()
            }
        }

        Ok(result)
    }
}
