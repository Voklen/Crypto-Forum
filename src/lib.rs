pub mod custom_types;

#[path = "backend/encrypt_decrypt.rs"]
pub mod encrypt_decrypt;
#[path = "backend/read.rs"]
pub mod read;
#[path = "backend/read_serde.rs"]
pub mod read_serde;
#[path = "backend/write_serde.rs"]
pub mod write_serde;

#[cfg(test)]
#[path = "tests/unit_tests/mod.rs"]
pub mod unit;