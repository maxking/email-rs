use crate::header_value_parser::{create_header, EmailHeader};
use crate::tests::helpers;
use std::collections::HashMap;

/// Email represents an Email object and contains all the properties and data
/// related to an email.
#[derive(Debug)]
pub struct Email {
    /// All the Headers for top level Email.
    pub headers: HashMap<String, EmailHeader>,

    /// body of the email is going to be stored as a string for now since we are
    /// going to parse only simple emails.
    pub body: EmailBody,
    // Children are child Email objects in case of multipart emails.
    // pub children: Vec<Email>,
}

/// For simplicity Email's body is now just going to be string.
type EmailBody = String;

impl Email {
    /// generate an Email from string object.
    pub fn from_str(s: String) -> Email {
        let mut allheaders = HashMap::new();

        let val: Vec<&str> = s.split("\n\n").collect();
        println!("{:?}", val);
        let mut headers = Some(val[0]);

        while let Some(_) = headers {
            match Email::get_one_header(headers.unwrap()) {
                (Some(headerval), rest) => {
                    let key_val: Vec<&str> = headerval.rsplitn(2, ':').collect();
                    allheaders.insert(
                        String::from(key_val[1].to_lowercase()),
                        create_header(key_val[1], key_val[0].trim_start()),
                    );
                    headers = rest;
                }
                _ => break,
            }
        }

        if let Some(EmailHeader::ContentType {
            maintype,
            subtype,
            value,
        }) = allheaders.get("content-type")
        {
            if maintype == "multipart" {
                println!("Found a multipart email.")
            }
        }

        Email {
            headers: allheaders,
            body: val[1].to_string(),
        }
    }

    /// get_one_header tries to parse one header line from the provided buffer
    /// and returns the rest back.
    fn get_one_header(_s: &str) -> (Option<String>, Option<&str>) {
        let mut header_line = String::new();
        let mut last = 0;
        let bytes = _s.as_bytes();
        for (i, &x) in bytes.iter().enumerate() {
            last = i;
            if x == b'\n' {
                if bytes[i + 1] == b' ' {
                    // If the next line starts with a whitespace, we continue
                    // parsing as a part of this same header.
                    continue;
                } else {
                    break;
                }
            } else {
                header_line.push(x as char);
            }
        }

        let mut rest = Some(&_s[last + 1..]);
        if last + 1 == _s.len() {
            rest = None;
        }
        (Some(header_line), rest)
    }

    /// Create a new email.
    pub fn new() -> Email {
        Email {
            body: String::from(""),
            headers: HashMap::new(),
        }
    }

    /// Add a new header.
    pub fn add_header(&mut self, key: String, value: EmailHeader) -> &mut Self {
        self.headers.insert(key, value);
        self
    }

    /// Set the To: header.
    pub fn to(&mut self, value: String) -> &mut Self {
        self.add_header(String::from("To"), EmailHeader::To(value))
    }

    /// Set the From: header.
    pub fn from(&mut self, value: String) -> &mut Self {
        self.add_header(String::from("From"), EmailHeader::From(value))
    }

    /// Add the Content-Type header.
    pub fn content_type(&mut self, maintype: String, subtype: String, value: String) -> &mut Self {
        self.add_header(
            String::from("Content-Type"),
            EmailHeader::ContentType {
                maintype,
                subtype,
                value,
            },
        )
    }

    /// Add the Subject.
    pub fn subject(&mut self, value: String) -> &mut Self {
        self.add_header(String::from("Subject"), EmailHeader::Subject(value))
    }

    /// Set the body of the Email.
    pub fn content(&mut self, value: String) -> &mut Self {
        self.body = value;
        self
    }
}

impl ToString for Email {
    fn to_string(&self) -> String {
        let mut serialized = String::new();
        for header in self.headers.iter() {
            let headerstr = match header {
                (_, EmailHeader::To(value)) => format!("To: {}", value),
                (_, EmailHeader::From(value)) => format!("From: {}", value),
                (_, EmailHeader::Date(value)) => format!("Date: {}", value),
                (_, EmailHeader::Subject(value)) => format!("Subject: {}", value),
                (_, EmailHeader::MessageID(value)) => format!("Message-ID: {}", value),
                (
                    _,
                    EmailHeader::ContentType {
                        maintype,
                        subtype,
                        value,
                    },
                ) => {
                    format!("Content-Type: {}/{}; {}", maintype, subtype, value)
                }
                (_, EmailHeader::ContentTransferEncoding(value)) => {
                    format!("Content-Transfer-Encoding: {:?}", value)
                }
                (key, EmailHeader::Generic(value)) => format!("{}: {}", key, value),
            };
            serialized.push_str(&headerstr);
            serialized.push_str("\r\n")
        }
        serialized.push_str("\r\n");
        serialized.push_str(&self.body);
        serialized
    }
}

#[test]
fn test_create_simple_email() {
    let mut newmail = Email::new();

    newmail
        .from("maxking@example.com".to_string())
        .to("testing@example.com".to_string())
        .subject("Welcome to the new library.".to_string())
        .content_type("text".to_string(), "plain".to_string(), "".to_string())
        .content("Hello World".to_string())
        .add_header(
            "x-mailfrom".to_string(),
            EmailHeader::Generic("maxking@example.com".to_string()),
        );

    helpers::check_defects(&newmail.to_string());
    println!("{}", newmail.to_string());
}

#[test]
#[ignore]
fn test_email_with_long_lines() {
    let mut mail = Email::new();
    mail.from("someone@example.com".to_string())
        .to("aperson@example.com".to_string())
        .add_header("X-RandomHeader".to_string(),
                    EmailHeader::Generic(String::from_utf8(vec![b'X'; 1000]).unwrap()))
        .content(String::from_utf8(vec![b'H'; 1000]).unwrap());
    helpers::check_defects(&mail.to_string());
    println!("{}", mail.to_string())
}

#[test]
fn test_get_one_simple_header() {
    let headers = "From: Someone
To: Person
Date: Today";
    assert_eq!(
        Email::get_one_header(headers),
        (
            Some("From: Someone".to_string()),
            Some("To: Person\nDate: Today")
        )
    )
}

#[test]
fn test_get_one_multiline_header() {
    let headers = "From: acomplexheader
Subject: This is a complex header which goes to
 2nd line identified by whitespace at the
 start of each next line of header.";
    let (header, rest) = Email::get_one_header(headers);
    assert_eq!(header, Some("From: acomplexheader".to_string()));
    assert_eq!(rest.is_some(), true);
    let (header, rest) = Email::get_one_header(rest.unwrap());
    assert_eq!(
        header,
        Some(
            "Subject: This is a complex \
             header which goes to 2nd line identified by whitespace at \
             the start of each next line of header."
                .to_string()
        )
    );
    assert_eq!(rest, None);
}
