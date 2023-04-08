use getset::CopyGetters;

#[derive(Default, Debug, CopyGetters, Eq, PartialEq, Clone)]
pub struct ClosedAnswer {
    #[getset(get = "pub")]
    text: String,
    #[getset(get_copy = "pub")]
    is_correct: bool,
}

impl ClosedAnswer {
    pub fn new<S: Into<String>>(text: S, is_correct: bool) -> Self {
        Self {
            text: text.into(),
            is_correct,
        }
    }
}

#[derive(Default, Debug, Eq, PartialEq, Clone)]
pub struct OpenAnswer {
    text: String,
}

impl OpenAnswer {
    pub fn new<S: Into<String>>(text: S) -> Self {
        Self { text: text.into() }
    }
}
