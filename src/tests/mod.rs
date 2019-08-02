use super::email::Email;

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

#[cfg(test)]
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
               Some(&"This is a multiline subject which goes on for a while because I chose to fold it.".to_string()));


}
