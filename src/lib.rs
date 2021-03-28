//! email-rs is intedned to be Rust interface to create and parse Email message
//! formats. It is in very early stages of development right now and the
//! initial idea to first support creating a serialized email message and then
//! maybe support parsing later.
//!
//! RFCs
//! ====
//!
//! Currently, the aim of this library is to generate serialized emails that
//! are complaint with [RFC 5322](https://tools.ietf.org/html/rfc5322).
//!
//! Support for MIME and Multipart emails will be added in future.
//!
//! Usage
//! =====
//!
//! The first entrypoint to this library is struct [Email](email/struct.Email.html).
//!
//! ```rust
//! use email_rs::Email;
//!
//! let mut mail = Email::new();
//! mail.from("maxking@example.com".to_string())
//!     .to("testing@example.com".to_string())
//!     .subject("Welcome to the new library.".to_string())
//!     .content_type("text".to_string(), "plain".to_string(), "".to_string())
//!     .content("Hello World".to_string())
//!     .add_header(
//!         "x-mailfrom".to_string(),
//!         EmailHeader::Generic("maxking@example.com".to_string()),
//!     );
//!
//! println!("{}", mail.to_string())
//! ```
//! The above will print:
//!
//! ```
//! Subject: Welcome to the new library.
//! x-mailfrom: maxking@example.com
//! Content-Type: text/plain;
//! To: testing@example.com
//! From: maxking@example.com
//!
//! Hello World
//! ```
pub mod email;
pub mod header_value_parser;

mod tests;

pub use self::email::Email;
