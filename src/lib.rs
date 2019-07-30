use std::collections::HashMap;

// Email represents an Email object and contains all the properties and data
// related to an email
#[derive(Debug)]
struct Email {
    // All the Headers for top level Email.
    headers: HashMap<String, String>,

    // body of the email is going to be stored as a string for now since we are
    // going to parse only simple emails.
    body: EmailBody,

    // Children are child Email objects in case of multipart emails.
    children: Vec<Email>,
}


// For simplicity Email's body is now just going to be string.
type EmailBody = String;


impl Email {
    pub fn from_str(s: String) -> Email {
        let mut allheaders = HashMap::new();

        let val: Vec<&str> = s.rsplitn(2, "\n\n").collect();
        let mut headers = Some(val[1]);


        while let Some(_) = headers {
            match Email::get_one_header(headers.unwrap()) {
                (Some(headerval), rest) => {
                    let key_val: Vec<&str> = headerval.rsplitn(2, ':').collect();
                    allheaders.insert(String::from(key_val[1]), String::from(key_val[0].trim_start()));
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

    fn from_bytes() -> Email {
        unimplemented!();
    }

    fn from_file() -> Email {
        unimplemented!();
    }

    fn new() -> Email {
        unimplemented!();
    }
}


#[cfg(test)]
#[test]
fn test_parse_simple_email() {
    let emailstr = String::from("From: maxking@example.com
To: something@person.com
Date: 9th Oct 2019
Subject: This is current email's subject
Message-ID: <messagecanthavespace@localhost.localdomain>

Hello World.
");

    let email = Email::from_str(emailstr);
    assert_eq!(email.body, String::from("Hello World.\n"));
    println!("{:?}", email.headers.keys());
    assert_eq!(email.headers.len(), 5);
    assert_eq!(email.headers.get("To"), Some(&"something@person.com".to_string()));
    assert_eq!(email.headers.get("Date"), Some(&"9th Oct 2019".to_string()));
    assert_eq!(email.headers.get("Subject"), Some(&"This is current email's subject".to_string()));
    assert_eq!(email.headers.get("From"), Some(&"maxking@example.com".to_string()));
    assert_eq!(email.headers.get("Message-ID"), Some(&"<messagecanthavespace@localhost.localdomain>".to_string()));

}

#[test]
fn test_parse_emails_with_multiline_headers() {
    let emailstr = "From: maxking
To: Someone
Subject: This is a multiline subject
 which goes on for a while because I chose
 to fold it.

This is the body of the email".to_string();
    let email = Email::from_str(emailstr);
    assert_eq!(email.body, "This is the body of the email".to_string());
    assert_eq!(email.headers.len(), 3);
    assert_eq!(email.headers.get("To"), Some(&"Someone".to_string()));
    assert_eq!(email.headers.get("From"), Some(&"maxking".to_string()));
    assert_eq!(email.headers.get("Subject"),
               Some(&"This is a multiline subject\n which goes on for a while because I chose\n to fold it.".to_string()));


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
                                 header which goes to\n 2nd line identified by whitespace at \
                                 the\n start of each next line of header.".to_string()));
        assert_eq!(rest, None);
    }
