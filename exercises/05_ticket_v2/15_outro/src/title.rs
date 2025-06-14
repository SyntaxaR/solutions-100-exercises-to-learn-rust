// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `TicketTitle` type,
//   enforcing that the title is not empty and is not longer than 50 bytes.
//   Implement the traits required to make the tests pass too.

#[derive(thiserror::Error, Debug)]
pub enum TicketTitleParseError {
    #[error("The title cannot be empty")]
    EmptyTitle,
    #[error("The title cannot be longer than 50 bytes")]
    TitleTooLong
}

#[derive(Debug, Clone, PartialEq)]
pub struct TicketTitle(String);

impl TryFrom<String> for TicketTitle {
    type Error = TicketTitleParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if &value == "" { return Err(TicketTitleParseError::EmptyTitle) }
        if value.len() > 50 { return Err(TicketTitleParseError::TitleTooLong )}
        Ok(Self(value))
    }
}

impl TryFrom<&str> for TicketTitle {
    type Error = TicketTitleParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "" { return Err(TicketTitleParseError::EmptyTitle) }
        if value.len() > 50 { return Err(TicketTitleParseError::TitleTooLong )}
        Ok(Self(String::from(value)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let title = TicketTitle::try_from("A title".to_string()).unwrap();
        assert_eq!(title.0, "A title");
    }

    #[test]
    fn test_try_from_empty_string() {
        let err = TicketTitle::try_from("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be empty");
    }

    #[test]
    fn test_try_from_long_string() {
        let title =
            "A title that's definitely longer than what should be allowed in a development ticket"
                .to_string();
        let err = TicketTitle::try_from(title).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be longer than 50 bytes");
    }

    #[test]
    fn test_try_from_str() {
        let title = TicketTitle::try_from("A title").unwrap();
        assert_eq!(title.0, "A title");
    }
}
