mod parser;

#[derive(Debug, Eq, PartialEq)]
pub struct Question {
    number: u32,
    text: String,
    answers: Vec<Answer>,
    reading: Option<String>,
    category: String,
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

impl Default for Answer {
    fn default() -> Self {
        Self {
            text: String::new(),
            is_correct: false,
        }
    }
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
