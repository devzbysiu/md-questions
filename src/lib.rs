mod parser;

#[derive(Debug, Eq, PartialEq)]
struct Question {
    number: u32,
    text: String,
    answers: Vec<Answer>,
    reading: Option<String>,
    category: String,
}

#[derive(Debug, Eq, PartialEq)]
struct Answer {
    text: String,
    is_correct: bool,
}
