use parser::questions;
use std::ops::Index;

mod parser;

#[derive(Debug, Eq, PartialEq)]
pub struct Questions {
    questions: Vec<Question>,
}

impl Questions {
    fn new(questions: Vec<Question>) -> Self {
        Self { questions }
    }
}

impl Default for Questions {
    fn default() -> Self {
        Self { questions: vec![] }
    }
}

impl From<&str> for Questions {
    fn from(content: &str) -> Self {
        let (_, questions) = questions(content).expect("failed to parse questions");
        questions
    }
}

impl Index<usize> for Questions {
    type Output = Question;

    fn index(&self, idx: usize) -> &Self::Output {
        self.questions
            .get(idx)
            .unwrap_or_else(|| panic!("filed to get question from idx: {}", idx))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Question {
    number: u32,
    text: String,
    answers: Vec<Answer>,
    reading: Option<String>,
    category: String,
}

impl Question {
    pub fn with_number(mut self, number: u32) -> Self {
        self.number = number;
        self
    }

    pub fn with_text<S: Into<String>>(mut self, text: S) -> Self {
        self.text = text.into();
        self
    }

    pub fn with_answer(mut self, answer: Answer) -> Self {
        self.answers.push(answer);
        self
    }

    pub fn with_reading<S: Into<String>>(mut self, reading: S) -> Self {
        self.reading = Some(reading.into());
        self
    }

    pub fn with_category<S: Into<String>>(mut self, category: S) -> Self {
        self.category = category.into();
        self
    }
}

impl Default for Question {
    fn default() -> Self {
        Self {
            number: 0,
            text: String::new(),
            answers: vec![],
            reading: None,
            category: String::new(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Answer {
    text: String,
    is_correct: bool,
}

impl Answer {
    pub fn new<S: Into<String>>(text: S, is_correct: bool) -> Self {
        Self {
            text: text.into(),
            is_correct,
        }
    }
}

impl Default for Answer {
    fn default() -> Self {
        Self {
            text: String::new(),
            is_correct: false,
        }
    }
}
