use crate::answer::{ClosedAnswer, OpenAnswer};

use derive_builder::Builder;
use either::Either;
use getset::Getters;

#[derive(Debug, Eq, PartialEq)]
pub struct Question {
    q: Either<ClosedQuestion, OpenQuestion>,
}

impl Question {
    #[must_use]
    pub fn from_closed(q: ClosedQuestion) -> Self {
        Self { q: Either::Left(q) }
    }

    #[must_use]
    pub fn from_open(q: OpenQuestion) -> Self {
        Self {
            q: Either::Right(q),
        }
    }

    #[must_use]
    pub fn closed() -> ClosedQuestionBuilder {
        ClosedQuestionBuilder::default()
    }

    #[must_use]
    pub fn open() -> OpenQuestionBuilder {
        OpenQuestionBuilder::default()
    }

    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.q.is_right()
    }

    #[must_use]
    pub fn as_closed(&self) -> Option<ClosedQuestion> {
        self.q.clone().left()
    }

    #[must_use]
    pub fn as_open(self) -> Option<OpenQuestion> {
        self.q.right()
    }
}

impl From<ClosedQuestion> for Question {
    fn from(q: ClosedQuestion) -> Self {
        Self { q: Either::Left(q) }
    }
}

impl From<OpenQuestion> for Question {
    fn from(q: OpenQuestion) -> Self {
        Self {
            q: Either::Right(q),
        }
    }
}

#[derive(Default, Getters, Builder, Debug, Eq, PartialEq, Clone)]
#[get = "pub"]
#[builder(setter(into))]
pub struct ClosedQuestion {
    number: i32,

    text: String,

    answers: Vec<ClosedAnswer>,

    #[builder(setter(into), default)]
    reading: Option<String>,

    category: String,
}

impl ClosedQuestion {
    #[must_use]
    pub fn answers_count(&self) -> usize {
        self.answers.len()
    }

    #[must_use]
    pub fn answer(&self, idx: usize) -> Option<&ClosedAnswer> {
        self.answers.get(idx)
    }

    #[must_use]
    pub fn is_multi(&self) -> bool {
        let correct_answers = self
            .answers
            .iter()
            .filter(|&answer| answer.is_correct())
            .count();
        correct_answers > 1
    }
}

#[derive(Default, Builder, Getters, Debug, Eq, PartialEq, Clone)]
pub struct OpenQuestion {
    pub(crate) number: i32,

    #[builder(setter(into))]
    pub(crate) text: String,

    pub(crate) answer: OpenAnswer,

    #[builder(setter(into, strip_option), default)]
    pub(crate) reading: Option<String>,

    #[builder(setter(into))]
    pub(crate) category: String,
}
