mod parser;

#[derive(Debug, Eq, PartialEq)]
pub struct Question {
    number: u32,
    text: String,
    answers: Vec<Answer>,
    reading: Option<String>,
    category: String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Answer {
    text: String,
    is_correct: bool,
}

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
