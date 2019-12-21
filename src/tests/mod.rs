use super::email::Email;
use super::header_value_parser::EmailHeader;

#[cfg(test)]
#[test]
fn test_parse_simple_email() {
    let emailstr = String::from(
        "From: maxking@example.com
To: something@person.com
Date: 9th Oct 2019
Subject: This is current email's subject
Message-ID: <messagecanthavespace@localhost.localdomain>

Hello World.
",
    );

    let email = Email::from_str(emailstr);
    assert_eq!(email.body, String::from("Hello World.\n"));
    // println!("{:?}", email.headers);
    assert_eq!(email.headers.len(), 5);
    for header in email.headers.iter() {
        match header {
            (_, EmailHeader::Subject(value)) => {
                assert_eq!(value, "This is current email's subject")
            }
            (_, EmailHeader::To(value)) => assert_eq!(value, "something@person.com"),
            (_, EmailHeader::From(value)) => assert_eq!(value, "maxking@example.com"),
            (_, EmailHeader::MessageID(value)) => {
                assert_eq!(value, "<messagecanthavespace@localhost.localdomain>")
            }
            (_, EmailHeader::Date(value)) => assert_eq!(value, "9th Oct 2019"),
            (_, EmailHeader::GenericHeader { key, value }) => {
                println!("Found unrecognized header {:?} with value {:?}", key, value)
            }
            _ => panic!("Unrecognized header type {:?}", header),
        }
    }
}

#[cfg(test)]
#[test]
fn test_parse_emails_with_multiline_headers() {
    let emailstr = "From: maxking
To: Someone
Subject: This is a multiline subject
 which goes on for a while because I chose
 to fold it.

This is the body of the email"
        .to_string();
    let email = Email::from_str(emailstr);
    assert_eq!(email.body, "This is the body of the email".to_string());
    assert_eq!(email.headers.len(), 3);
    // println!("{:?}", email.headers);
    for header in email.headers.iter() {
        match header {
            (_, EmailHeader::To(value)) => assert_eq!(value, "Someone"),
            (_, EmailHeader::From(value)) => assert_eq!(value, "maxking"),
            (_, EmailHeader::Subject(value)) => assert_eq!(
                value,
                "This is a multiline subject which goes on for a while because I chose to fold it."
            ),
            _ => println!("Unrecognized header {:?}", header),
        }
    }
}

#[cfg(test)]
#[test]
fn test_parse_simple_multipart_email() {
    let emailstr = "From: maxking@example.com
To: aperson@example.com
Subject: A multipart/alternative email with a HTML alternative.
Content-Type: multipart/alternative; boundary=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa

--aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
Content-Transfer-Encoding: quoted-printable
Content-Type: text/plain; charset=UTF-8
Mime-Version: 1.0

This is the plaintext alternative of the email.

--aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
Content-Transfer-Encoding: quoted-printable
Content-Type: text/html; charset=UTF-8
Mime-Version: 1.0

<h1> Hello World </h1>

<footer> Thanks for reading! </footer>"
        .to_string();

    let email = Email::from_str(emailstr);
    assert_eq!(email.headers.len(), 4);
    match email.headers.get("to") {
        Some(EmailHeader::To(value)) => assert_eq!(value, "aperson@example.com"),
        _ => panic!("Failed to get header value."),
    }
    match email.headers.get("content-type") {
        Some(EmailHeader::ContentType {
            maintype,
            subtype,
            value,
        }) => {
            assert_eq!(maintype, "multipart");
            assert_eq!(subtype, "alternative");
        }
        val => panic!("Expected ContentType header found: {:?}", val),
    }
    // assert_eq!(email.children.len(), 2)
}
