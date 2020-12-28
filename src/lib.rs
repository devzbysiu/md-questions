use parser::questions;
use std::ops::Index;

mod parser;

/// Contains all questions parsed from markdown.
///
/// Example usage:
/// ```
/// # use std::error::Error;
/// # use std::fs::read_to_string;
/// use md_questions::MdQuestions;
///
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let content = read_to_string("res/checkbox-questions.md")?;
/// let questions = MdQuestions::from(content.as_str());
/// let first_question = &questions[0];
/// println!("First question: {}", first_question.text());
/// # Ok(())
/// # }
/// ```
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
            .unwrap_or_else(|| panic!("filed to get question from idx: {}", idx))
    }
}

/// Single question.
///
/// Example of usage:
/// ```
/// # use std::error::Error;
/// use md_questions::{Question, Answer};
///
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let question = Question::default()
///     .with_number(1)
///     .with_text("Why the sky is blue?")
///     .with_answer(Answer::new("Because of oxygen", false))
///     .with_answer(Answer::new("Because of oceans", false))
///     .with_answer(Answer::new("Because of atmosphere", true))
///     .with_answer(Answer::new("It's not", false))
///     .with_category("General")
///     .with_reading("https://spaceplace.nasa.gov/blue-sky/en/");
/// # Ok(())
/// # }
/// ```
#[derive(Default, Debug, Eq, PartialEq, Clone)]
pub struct Question {
    /// Number of the question in order.
    number: u32,
    /// Question's content.
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

    #[must_use]
    pub fn number(&self) -> u32 {
        self.number
    }

    #[must_use]
    pub fn text(&self) -> String {
        self.text.clone()
    }

    #[must_use]
    pub fn answers(&self) -> &[Answer] {
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
    pub fn answer(&self, idx: usize) -> Option<&Answer> {
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

/// One of the answers available in a question.
///
/// Example usage:
/// ```
/// # use std::error::Error;
/// use md_questions::Answer;
///
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let answer = Answer::new("Rust is great", true);
/// # Ok(())
/// # }
/// ```
#[derive(Default, Debug, Eq, PartialEq, Clone)]
pub struct Answer {
    /// Answer text.
    text: String,
    /// Is answer correct?
    is_correct: bool,
}

impl Answer {
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
