use serde::{Deserialize, Serialize};

/// Represents JSON-RPC request id.
///
/// An identifier established by the Client that MUST contain a String, Number,
/// or NULL value if included, If it is not included it is assumed to be a notification.
/// The value SHOULD normally not be Null and Numbers SHOULD NOT contain fractional parts.
///
/// The Server **MUST** reply with the same value in the Response object if included.
/// This member is used to correlate the context between the two objects.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Id {
    /// Numeric id
    Num(u64),
    /// String id
    Str(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_serialization() {
        let cases = vec![
            (Id::Num(0), r#"0"#),
            (Id::Str("1".into()), r#""1""#),
            (Id::Str("test".into()), r#""test""#),
        ];

        for (id, expect) in cases {
            assert_eq!(serde_json::to_string(&id).unwrap(), expect);
            assert_eq!(id, serde_json::from_str(expect).unwrap());
        }

        assert_eq!(
            serde_json::to_string(&vec![
                Id::Num(0),
                Id::Str("1".to_owned()),
                Id::Str("test".to_owned()),
            ])
            .unwrap(),
            r#"[0,"1","test"]"#
        );
    }
}
