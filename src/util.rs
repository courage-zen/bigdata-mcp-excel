//! Small shared helpers.

pub fn base64_encode(bytes: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(bytes)
}

#[cfg(test)]
mod tests {
    #[test]
    fn base64_roundtrip() {
        let raw = b"hello xlsx";
        let encoded = super::base64_encode(raw);
        use base64::Engine;
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(encoded)
            .unwrap();
        assert_eq!(decoded, raw);
    }
}
