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
    fn from_str(s: &String) -> Email {
        let mut allheaders = HashMap::new();
        let mut email_lines = s.lines();
        // First, let's try to parse headers in the emails.
        while true{
            let line = email_lines.next();
            match line {
                // Stop parsing headers as soon as we encounter an empty line.
                Some("") => break,
                Some(value) => {
                    let header: Vec<&str> = value.rsplitn(2, ":").collect();
                    allheaders.insert(String::from(header[1]), String::from(header[0].trim_start()));
                },
                None => break,
            }
        };

        let mut body = String::new();
        loop {
            let line = email_lines.next();
            match line {
                Some(value) => body.push_str(value),
                None => break
            }
        }

        Email{
            headers: allheaders,
            body: body,
            children: vec!()
        }
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
fn it_works() {
    let emailstr = String::from("From: maxking@example.com
To: something@person.com
Date: 9th Oct 2019
Subject: This is current email's subject
Message-ID: <messagecanthavespace@localhost.localdomain>

Hello World.
");

    let email = Email::from_str(&emailstr);
    for (header, value) in &email.headers {
        println!("key = {:?}, value = {:?}", header, value);
    }
    assert_eq!(email.body, String::from("Hello World."))
}
