use crate::Answer;
use crate::Question;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::sequence::tuple;
use nom::IResult;
use std::num::ParseIntError;

#[allow(dead_code)] // TODO: remove
fn question(i: &str) -> IResult<&str, Question> {
    let (i, number) = question_header(i)?;
    let (i, _) = new_line(i)?;
    let (i, text) = text(i)?;
    let (i, _) = new_line(i)?;
    let (i, _) = new_line(i)?;
    Ok((
        i,
        Question {
            number,
            text,
            answers: vec![],
            reading: None,
            category: "".into(),
        },
    ))
}

fn question_header(i: &str) -> IResult<&str, u32> {
    let (i, (_, num)) = tuple((tag("## Question "), map_res(digit1, to_int)))(i)?;
    Ok((i, num))
}

fn to_int(i: &str) -> Result<u32, ParseIntError> {
    i.parse::<u32>()
}

fn new_line(i: &str) -> IResult<&str, char> {
    char('\n')(i)
}

fn text(i: &str) -> IResult<&str, String> {
    let (i, text) = take_until("\n")(i)?;
    Ok((i, text.into()))
}

fn answer(i: &str) -> IResult<&str, Answer> {
    let (i, (checkbox, _, text)) = tuple((alt((tag("[ ]"), tag("[x]"))), char(' '), text))(i)?;
    let mut is_correct = false;
    if checkbox == "[x]" {
        is_correct = true;
    }
    Ok((
        i,
        Answer {
            text: text.into(),
            is_correct,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::{answer, new_line, question_header, text};
    use crate::parsers::question;
    use crate::{Answer, Question};

    #[test]
    fn test_question_parser() {
        let input = r#"## Question 1
Some text of the question

"#;
        assert_eq!(
            question(input),
            Ok((
                "",
                Question {
                    number: 1,
                    text: "Some text of the question".into(),
                    answers: vec![],
                    reading: None,
                    category: "".into()
                }
            ))
        );
    }

    #[test]
    fn test_question_number_parser() {
        let input = "## Question 1";
        assert_eq!(question_header(input), Ok(("", 1)));
    }

    #[test]
    fn test_new_line_parser() {
        let input = r#"
"#;
        assert_eq!(new_line(input), Ok(("", '\n')));
    }

    #[test]
    fn test_text_parser() {
        let input = r#"Some text here
"#;
        assert_eq!(text(input), Ok(("\n", "Some text here".into())));
    }

    #[test]
    fn test_answer_parser() {
        let input = r#"[ ] Some answer
"#;
        assert_eq!(
            answer(input),
            Ok((
                "\n",
                Answer {
                    text: "Some answer".into(),
                    is_correct: false,
                }
            ))
        );

        let input = r#"[x] Some answer
"#;
        assert_eq!(
            answer(input),
            Ok((
                "\n",
                Answer {
                    text: "Some answer".into(),
                    is_correct: true,
                }
            ))
        );
    }
}
