// Parser for email header values.
//
// This returned structured parsing of the Email headers.

/// EmailHeaders represents all the different types of headers we can have.
#[derive(Debug)]
pub enum EmailHeader {
    /// Content-Transfer-Encoding header value.
    ContentTransferEncoding(String),

    /// Content-Type header value.
    /// maintype: represents the maintype of the maintype/subtype. If this
    /// value is 'multipart', then this email will have childrens, otherwise
    /// not.
    ContentType {
        maintype: String,
        subtype: String,
        value: String,
    },

    /// Date header.
    Date(String),

    /// To header
    To(String),

    /// From header
    From(String),

    /// Subject header
    Subject(String),

    /// Message-ID header.
    MessageID(String),

    /// A GenericHeader represents all the headers we don't have special value
    /// parsers for. It includes both the key and value of the email header.
    Generic(String),
}

/// Returns one of the EmailHeader based on the "key" type. The header's value
/// might be further parsed depending on the type of the header.
pub fn create_header(key: &str, value: &str) -> EmailHeader {
    match key.to_lowercase().as_ref() {
        "to" => parse_to(value),
        "from" => parse_from(value),
        "date" => parse_date(value),
        "subject" => parse_subject(value),
        "content-type" => parse_content_type(value),
        "content-transfer-encoding" => parse_cte(value),
        "message-id" => parse_message_id(value),
        _ => parse_generic_header(value),
    }
}

/// Parse the value of a To: header returning an email header object.
fn parse_to(value: &str) -> EmailHeader {
    EmailHeader::To(value.to_string())
}

fn parse_from(value: &str) -> EmailHeader {
    EmailHeader::From(value.to_string())
}

fn parse_date(value: &str) -> EmailHeader {
    EmailHeader::Date(value.to_string())
}

fn parse_subject(value: &str) -> EmailHeader {
    EmailHeader::Subject(value.to_string())
}

fn parse_content_type(value: &str) -> EmailHeader {
    let ctype: &str;
    let mime_params: &str;

    if value.contains(";") {
        let vals: Vec<&str> = value.split(";").collect();
        ctype = vals[0];
    } else {
        ctype = value;
    }
    let split_type: Vec<&str> = ctype.split("/").collect();

    EmailHeader::ContentType {
        maintype: split_type[0].to_string(),
        subtype: split_type[1].to_string(),
        value: value.to_string(),
    }
}

fn parse_cte(value: &str) -> EmailHeader {
    EmailHeader::ContentTransferEncoding(value.to_string())
}

fn parse_message_id(value: &str) -> EmailHeader {
    EmailHeader::MessageID(value.to_string())
}

fn parse_generic_header(value: &str) -> EmailHeader {
    EmailHeader::Generic(value.to_string())
}
