/// Validate a JSON document.
pub fn validate(input: &str) -> Result<(), serde_json::Error> {
    let _: serde_json::Value = serde_json::from_str(input)?;
    Ok(())
}

/// Format valid JSON with standard pretty-printing.
pub fn format(input: &str) -> Result<String, serde_json::Error> {
    let value: serde_json::Value = serde_json::from_str(input)?;
    serde_json::to_string_pretty(&value)
}

/// Serialize valid JSON without insignificant whitespace.
pub fn compact(input: &str) -> Result<String, serde_json::Error> {
    let value: serde_json::Value = serde_json::from_str(input)?;
    serde_json::to_string(&value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_and_formats_json() {
        assert!(validate(r#"{"ready":true}"#).is_ok());
        assert_eq!(
            format(r#"{"ready":true}"#).unwrap(),
            "{\n  \"ready\": true\n}"
        );
    }

    #[test]
    fn rejects_invalid_json() {
        assert!(validate("{broken}").is_err());
    }

    #[test]
    fn compact_removes_whitespace() {
        assert_eq!(
            compact("{ \"message\" : \"hola\" }").unwrap(),
            r#"{"message":"hola"}"#
        );
    }
}
