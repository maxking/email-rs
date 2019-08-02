use std::collections::HashMap;

/// Email represents an Email object and contains all the properties and data
/// related to an email
#[derive(Debug)]
pub struct Email {
    /// All the Headers for top level Email.
    pub headers: HashMap<String, String>,

    /// body of the email is going to be stored as a string for now since we are
    /// going to parse only simple emails.
    pub body: EmailBody,

    /// Children are child Email objects in case of multipart emails.
    pub children: Vec<Email>,
}


/// For simplicity Email's body is now just going to be string.
type EmailBody = String;

impl Email {
    /// generate an Email from string object.
    pub fn from_str(s: String) -> Email {
        let mut allheaders = HashMap::new();

        let val: Vec<&str> = s.rsplitn(2, "\n\n").collect();
        let mut headers = Some(val[1]);


        while let Some(_) = headers {
            match Email::get_one_header(headers.unwrap()) {
                (Some(headerval), rest) => {
                    let key_val: Vec<&str> = headerval.rsplitn(2, ':').collect();
                    allheaders.insert(String::from(key_val[1]),
                                      String::from(key_val[0].trim_start()));
                    headers = rest;
                },
                _ => break,
            }
        }

        Email{
            headers: allheaders,
            body: val[0].to_string(),
            children: vec!()
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
                if bytes[i+1] == b' ' {
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

        let mut rest = Some(&_s[last+1..]);
        if last+1 == _s.len() {
            rest = None;
        }
        (Some(header_line), rest)
    }

    /// generate an Email from raw bytes.
    fn from_bytes() -> Email {
        unimplemented!();
    }

    /// generate an email from a file path.
    fn from_file() -> Email {
        unimplemented!();
    }

    /// Create a new email.
    fn new() -> Email {
        unimplemented!();
    }
}



#[test]
fn test_get_one_simple_header() {
    let headers = "From: Someone
To: Person
Date: Today";
    assert_eq!(Email::get_one_header(headers),
               (Some("From: Someone".to_string()), Some("To: Person\nDate: Today")))
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
    assert_eq!(header, Some("Subject: This is a complex \
                             header which goes to 2nd line identified by whitespace at \
                             the start of each next line of header.".to_string()));
    assert_eq!(rest, None);
}
