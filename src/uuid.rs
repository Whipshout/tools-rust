//! Generates uuids
//!
//! Provides an abstraction of C bindings to use this module directly from Typescript.<br>
//! When the abstraction is used, there are these advantages:
//! - Faster
//! - Safer
//!
//! To achieve this abstraction, we use the NAPI crate:
//! - [`NAPI Crate`]
//! - [`NAPI Doc`]
//! - [`NAPI GitHub`]
//! - [`NAPI Examples`]
//!
//! [`NAPI Crate`]: https://crates.io/crates/napi/2.0.0-alpha.4
//! [`NAPI Doc`]: https://docs.rs/napi/2.0.0-alpha.4/napi/
//! [`NAPI GitHub`]: https://github.com/napi-rs/napi-rs
//! [`NAPI Examples`]: https://github.com/napi-rs/napi-rs/tree/main/examples/napi

use sha2::{Digest, Sha256};
use uuid::Uuid;

/// Exported function to Typescript that returns a uuid
///
/// The function has an optional parameter of type string.
/// If called without a parameter, it returns a random uuid.
/// If called with a string type parameter, it returns a uuid based on a Sha256 hashed input.
///
/// # Panics
///
/// The function works without parameter and with a string type parameter.
/// The function will panic if is called from Typescript with a non-string type parameter.
///
/// # Examples (Typescript code)
///
/// ```typescript
/// import { uuid } from './index'
///
/// // Generate a uuid from a string input
/// const id = uuid("1cbf5655eeb58bf905f4b1958ad0b71a2855")
/// console.log(id)
///
/// // Generate a random uuid
/// const id = uuid()
/// console.log(id)
/// ```
#[napi]
fn uuid(s: Option<String>) -> String {
    // Check if there is a string parameter or not
    match s {
        Some(s) => generate_uuid_with_input(s),
        None => generate_uuid_without_input(),
    }
}

/// Returns a uuid based on a hashed input
///
/// The function generates a uuid based on a Sha256 hashed input of string type.
///
/// # Panics
///
/// The function cannot panic.
///
/// # Examples
///
/// ```rust
/// // Generate a uuid from a string input
/// let id = generate_uuid_with_input("1cbf5655eeb58bf905f4b1958ad0b71a2855");
/// println!("{}", id);
/// ```
fn generate_uuid_with_input(s: String) -> String {
    // Hash the string using Sha256
    let hash = Sha256::digest(s.as_bytes());

    // Format hash to text
    let hash_text = format!("{:x}", hash);

    // ------------ Format hash to uuid style ------------
    // Change length to 36
    let (first_half, _) = hash_text.split_at(36);
    let mut uuid = first_half.to_string();

    // Change some characters to dash
    uuid.replace_range(8..9, "-");
    uuid.replace_range(13..14, "-");
    uuid.replace_range(18..19, "-");
    uuid.replace_range(23..24, "-");

    // Change some characters to specific numbers
    uuid.replace_range(14..15, "4");
    uuid.replace_range(19..20, "8");
    // ------------ Format hash to uuid style ------------

    uuid
}

/// Returns a random uuid
///
/// The function generates a random uuid of string type.
///
/// # Panics
///
/// The function cannot panic.
///
/// # Examples
///
/// ```rust
/// // Generate a random uuid
/// let id = generate_uuid_without_input();
/// println!("{}", id);
/// ```
fn generate_uuid_without_input() -> String {
    // Generates random uuid
    Uuid::new_v4().to_string()
}

#[cfg(test)]
mod tests {
    use fake::{Fake, StringFaker};
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;
    use std::ops::Index;

    use super::*;

    #[derive(Debug, Clone)]
    // Valid string structure only for test purposes
    struct ValidString(pub String);

    // Possible string charset
    const ASCII: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    // Add arbitrary trait to ValidString struct
    impl Arbitrary for ValidString {
        /// Add a mutable component to generate stuff
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            // Constructor for strings with a specific charset and length
            let faker = StringFaker::with(Vec::from(ASCII), 36);
            // Generates fake strings with the constructor options and mutable each interaction
            let string: String = faker.fake_with_rng(g);

            Self(string)
        }
    }

    // Generates a random string with a specific charset and length
    fn generate_random_string(charset: &str, length: usize) -> String {
        // Constructor for strings with a specific charset and length
        let faker = StringFaker::with(Vec::from(charset), length);
        // Generates fake strings with the constructor options
        faker.fake()
    }

    #[quickcheck]
    // Checks if a correct uuid is generated with an input
    // We use quickcheck to do 100 times this test with ValidString type
    // ValidString structure implements arbitrary and generates random data
    fn generate_correct_uuid_with_input(s: ValidString) {
        // Input
        let uuid = generate_uuid_with_input(s.0);

        // Asserts
        assert_eq!(uuid.len(), 36, "Uuid length should be 36");
        assert_eq!(uuid.matches('-').count(), 4, "Uuid must have 4 '-'");
        assert_eq!(
            uuid.find('-'),
            Some(8),
            "Uuid must have the first '-' in the position 8 of the string"
        );
        assert_eq!(
            uuid.rfind('-'),
            Some(23),
            "Uuid must have the last '-' in the position 23 of the string"
        );
        assert_eq!(
            uuid.index(13..14),
            "-",
            "Uuid must have a '-' in the position 13"
        );
        assert_eq!(
            uuid.index(18..19),
            "-",
            "Uuid must have a '-' in the position 18"
        );
        assert_eq!(
            uuid.index(14..15),
            "4",
            "Uuid must have a '4' in the position 14"
        );
        assert_eq!(
            uuid.index(19..20),
            "8",
            "Uuid must have a '8' in the position 19"
        );
    }

    #[test]
    // Checks if a correct uuid is generated with an input
    fn generate_2_correct_uuids_with_same_input() {
        // Generates data
        let random_string = generate_random_string(ASCII, 36);

        // Inputs
        let uuid = generate_uuid_with_input(random_string.clone());
        let uuid2 = generate_uuid_with_input(random_string);

        // Asserts
        assert!(uuid.is_ascii());
        assert_eq!(uuid, uuid2, "Both uuids must be the same");
        assert_eq!(uuid.len(), 36, "Uuid length should be 36");
        assert_eq!(uuid.matches('-').count(), 4, "Uuid must have 4 '-'");
        assert_eq!(
            uuid.find('-'),
            Some(8),
            "Uuid must have the first '-' in the position 8 of the string"
        );
        assert_eq!(
            uuid.rfind('-'),
            Some(23),
            "Uuid must have the last '-' in the position 23 of the string"
        );
        assert_eq!(
            uuid.index(13..14),
            "-",
            "Uuid must have a '-' in the position 13"
        );
        assert_eq!(
            uuid.index(18..19),
            "-",
            "Uuid must have a '-' in the position 18"
        );
        assert_eq!(
            uuid.index(14..15),
            "4",
            "Uuid must have a '4' in the position 14"
        );
        assert_eq!(
            uuid.index(19..20),
            "8",
            "Uuid must have a '8' in the position 19"
        );
    }

    #[test]
    // Checks if a correct uuid is generated without input
    fn generate_correct_uuid_without_input() {
        // Input
        let uuid = generate_uuid_without_input();

        // Asserts
        assert!(uuid.is_ascii());
        assert_eq!(uuid.len(), 36, "Uuid length should be 36");
        assert_eq!(uuid.matches('-').count(), 4, "Uuid must have 4 '-'");
        assert_eq!(
            uuid.find('-'),
            Some(8),
            "Uuid must have the first '-' in the position 8 of the string"
        );
        assert_eq!(
            uuid.rfind('-'),
            Some(23),
            "Uuid must have the last '-' in the position 8 of the string"
        );
        assert_eq!(
            uuid.index(13..14),
            "-",
            "Uuid must have a '-' in the position 13"
        );
        assert_eq!(
            uuid.index(18..19),
            "-",
            "Uuid must have a '-' in the position 18"
        );
    }
}
