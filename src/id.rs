use serde::{Deserialize, Serialize};

/// Represents JSON-RPC request id.
///
/// An identifier established by the Client that MUST contain a String, Number, or NULL value if included.
///
/// The Server **MUST** reply with the same value in the Response object if included.
/// This member is used to correlate the context between the two objects.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Id {
    /// Null id
    ///
    /// The value SHOULD normally not be `Null`.
    ///
    /// The use of Null as a value for the id member in a Request object is discouraged,
    /// because this specification uses a value of Null for Responses with an unknown id.
    /// Also, because JSON-RPC 1.0 uses an id value of Null for Notifications this could cause
    /// confusion in handling.
    Null,
    /// Numeric id
    Num(u64),
    /// String id
    ///
    /// Fractional parts may be problematic, since many decimal fractions cannot be represented exactly as binary fractions.
    Str(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_serialization() {
        let cases = vec![
            (Id::Null, r#"null"#),
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
                Id::Null,
                Id::Num(0),
                Id::Str("1".to_owned()),
                Id::Str("test".to_owned()),
            ])
            .unwrap(),
            r#"[null,0,"1","test"]"#
        );
    }
}
