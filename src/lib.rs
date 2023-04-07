use derive_builder::Builder;
use either::Either;
use parser::questions;
use std::ops::Index;

mod parser;

#[derive(Debug, Eq, PartialEq, Default)]
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

    #[must_use]
    pub fn questions(&self) -> &[Question] {
        &self.questions
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

#[derive(Debug, Default, PartialEq, Eq)]
pub struct QuestionMetadata {
    question_type: QuestionType,
}

impl QuestionMetadata {
    #[must_use]
    pub fn with_question_type(mut self, question_type: QuestionType) -> Self {
        self.question_type = question_type;
        self
    }

    #[must_use]
    pub fn question_type(&self) -> &QuestionType {
        &self.question_type
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum QuestionType {
    Closed,
    Open,
}

impl From<&str> for QuestionType {
    fn from(value: &str) -> Self {
        match value {
            "closed" => Self::Closed,
            "open" => Self::Open,
            _ => panic!("not supported question type: {value}"),
        }
    }
}

impl Default for QuestionType {
    fn default() -> Self {
        Self::Closed
    }
}

#[derive(Default, Builder, Debug, Eq, PartialEq, Clone)]
pub struct ClosedQuestion {
    number: u32,

    #[builder(setter(into))]
    text: String,

    answers: Vec<ClosedAnswer>,

    #[builder(setter(into, strip_option), default)]
    reading: Option<String>,

    #[builder(setter(into))]
    category: String,
}

impl ClosedQuestion {
    #[must_use]
    pub fn number(&self) -> u32 {
        self.number
    }

    #[must_use]
    pub fn text(&self) -> String {
        self.text.clone()
    }

    #[must_use]
    pub fn answers(&self) -> &[ClosedAnswer] {
        &self.answers
    }

    #[must_use]
    pub fn reading(&self) -> Option<&String> {
        self.reading.as_ref()
    }

    #[must_use]
    pub fn category(&self) -> String {
        self.category.clone()
    }

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
            .answers()
            .iter()
            .filter(|&answer| answer.is_correct())
            .count();
        correct_answers > 1
    }
}

#[derive(Default, Builder, Debug, Eq, PartialEq, Clone)]
pub struct OpenQuestion {
    number: u32,

    #[builder(setter(into))]
    text: String,

    answer: OpenAnswer,

    #[builder(setter(into, strip_option), default)]
    reading: Option<String>,

    #[builder(setter(into))]
    category: String,
}

impl OpenQuestion {
    #[must_use]
    pub fn number(&self) -> u32 {
        self.number
    }

    #[must_use]
    pub fn text(&self) -> String {
        self.text.clone()
    }

    #[must_use]
    pub fn answer(&self) -> &OpenAnswer {
        &self.answer
    }

    #[must_use]
    pub fn reading(&self) -> Option<&String> {
        self.reading.as_ref()
    }

    #[must_use]
    pub fn category(&self) -> String {
        self.category.clone()
    }
}

#[derive(Default, Debug, Eq, PartialEq, Clone)]
pub struct ClosedAnswer {
    text: String,
    is_correct: bool,
}

impl ClosedAnswer {
    pub fn new<S: Into<String>>(text: S, is_correct: bool) -> Self {
        Self {
            text: text.into(),
            is_correct,
        }
    }

    #[must_use]
    pub fn text(&self) -> String {
        self.text.clone()
    }

    #[must_use]
    pub fn is_correct(&self) -> bool {
        self.is_correct
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
