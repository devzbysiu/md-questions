mod parsers;

#[derive(Debug, Eq, PartialEq)]
struct Question {
    number: u32,
    text: String,
    answers: Vec<Answer>,
    reading: Option<Reading>,
    category: String,
}

#[derive(Debug, Eq, PartialEq)]
struct Answer {
    text: String,
    is_correct: bool,
}

#[derive(Debug, Eq, PartialEq)]
struct Reading {
    url: String,
}
