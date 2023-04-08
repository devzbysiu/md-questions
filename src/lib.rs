#![allow(clippy::module_name_repetitions)]

use getset::Getters;
use parser::questions;
use std::ops::Index;

pub use crate::answer::{ClosedAnswer, OpenAnswer};
pub use crate::question::{ClosedQuestion, OpenQuestion, Question};

mod answer;
mod parser;
mod question;

#[derive(Debug, Getters, Eq, PartialEq, Default)]
pub struct MdQuestions {
    questions: Vec<Question>,
}

impl MdQuestions {
    fn new(questions: Vec<Question>) -> Self {
        Self { questions }
    }

    #[must_use]
    pub fn count(&self) -> usize {
        self.questions.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.questions.is_empty()
    }
}

impl From<&str> for MdQuestions {
    fn from(content: &str) -> Self {
        let (_, questions) = questions(content).expect("failed to parse questions");
        questions
    }
}

impl Index<usize> for MdQuestions {
    type Output = Question;

    fn index(&self, idx: usize) -> &Self::Output {
        self.questions
            .get(idx)
            .unwrap_or_else(|| panic!("filed to get question from idx: {idx}"))
    }
}
