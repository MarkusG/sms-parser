use std::{error::Error, fmt};

use base64;
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
    pub timestamp: u64,
    pub outgoing: bool,
    pub address: String,
    pub contact: Option<String>,
    pub body: String
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

#[derive(Debug)]
pub struct Mms {
    pub timestamp: u64,
    pub addresses: Vec<String>,
    pub parts: Vec<Part>
}

#[derive(Debug)]
pub struct Part {
    pub content_type: String,
    pub text: Option<String>,
    pub data: Option<Vec<u8>>
}

impl Mms {
    pub fn blank() -> Mms {
        Mms {
            timestamp: 0,
            addresses: Vec::new(),
            parts: Vec::new()
        }
    }

    pub fn new(attributes: &[OwnedAttribute]) -> Result<Mms, Box<dyn Error>> {
        let mut result = Mms { 
            timestamp: 0,
            addresses: Vec::new(),
            parts: Vec::new(),
        };
        for a in attributes {
            match a.name.local_name.as_str() {
                "date" => result.timestamp = a.value.parse::<u64>()?,
                _ => ()
            }
        }
        Ok(result)
    }

    pub fn add_addr(&mut self, attributes: &[OwnedAttribute]) -> Result<(), Box<dyn Error>> {
        for a in attributes {
            match a.name.local_name.as_str() {
                "address" => self.addresses.push(a.value.clone()),
                _ => ()
            }
        }
        Ok(())
    }

    pub fn add_part(&mut self, attributes: &[OwnedAttribute]) -> Result<(), Box<dyn Error>> {
        let mut part = Part { 
            content_type: "".to_string(),
            text: None,
            data: None
        };
        for a in attributes {
            match a.name.local_name.as_str() {
                "ct" => part.content_type = a.value.clone(),
                "text" => part.text = if a.value == "null" { None } else { Some(a.value.clone()) },
                "data" => part.data = Some(base64::decode(&a.value)?),
                _ => ()
            }
        }
        self.parts.push(part);
        Ok(())
    }
}
