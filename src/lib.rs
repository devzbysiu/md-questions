use parser::questions;
use std::ops::Index;

mod parser;

/// Contains all questions parsed from markdown.
///
/// Example usage:
/// ```
/// # use std::error::Error;
/// # use std::fs::read_to_string;
/// use md_questions::Questions;
///
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let content = read_to_string("res/QUESTIONS.md")?;
/// let questions = Questions::from(content.as_str());
/// # Ok(())
/// # }
/// ```
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

/// Single question.
#[derive(Debug, Eq, PartialEq)]
pub struct Question {
    /// Number of the question in order.
    number: u32,
    /// Questions content.
    text: String,
    /// Possible answers. Multiple answers can be correct.
    answers: Vec<Answer>,
    /// Additional materials. Optional.
    reading: Option<String>,
    /// Category of the question.
    category: String,
}

impl Question {
    #[must_use]
    pub fn with_number(mut self, number: u32) -> Self {
        self.number = number;
        self
    }

    #[must_use]
    pub fn with_text<S: Into<String>>(mut self, text: S) -> Self {
        self.text = text.into();
        self
    }

    #[must_use]
    pub fn with_answer(mut self, answer: Answer) -> Self {
        self.answers.push(answer);
        self
    }

    #[must_use]
    pub fn with_reading<S: Into<String>>(mut self, reading: S) -> Self {
        self.reading = Some(reading.into());
        self
    }

    #[must_use]
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

/// One of the answers available in a question.
#[derive(Debug, Eq, PartialEq)]
pub struct Answer {
    /// Answer text.
    text: String,
    /// Is question correct?
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
