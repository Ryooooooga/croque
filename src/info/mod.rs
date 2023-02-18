use base64::Engine;

pub mod git;

fn base64_engine() -> impl Engine {
    base64::engine::general_purpose::GeneralPurpose::new(
        &base64::alphabet::STANDARD,
        base64::engine::general_purpose::PAD,
    )
}

pub fn encode_base64(bytes: &[u8]) -> String {
    base64_engine().encode(bytes)
}

pub fn decode_base64(encoded: &str) -> Result<Vec<u8>, base64::DecodeError> {
    base64_engine().decode(encoded)
}
