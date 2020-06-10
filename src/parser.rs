use crate::Answer;
use crate::Question;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{char, digit1};
use nom::combinator::{map_res, opt};
use nom::multi::many_m_n;
use nom::sequence::tuple;
use nom::IResult;
use std::num::ParseIntError;

#[allow(dead_code)] // TODO: remove
fn question(i: &str) -> IResult<&str, Question> {
    let (i, (number, category)) = question_header(i)?;
    let (i, _) = new_line(i)?;
    let (i, text) = paragraph(i)?;
    let (i, _) = new_line(i)?;
    let (i, _) = new_line(i)?;
    let (i, _) = answers_header(i)?;
    let (i, _) = new_line(i)?;
    let (i, answers) = answers(i)?;
    let (i, _) = new_line(i)?;
    // let (i, _) = new_line(i)?; // TODO: this should be here to be consistent
    let (i, _) = reading_header(i)?;
    let (i, _) = new_line(i)?;
    let (i, reading) = reading(i)?;
    let (i, _) = new_line(i)?;
    let (i, _) = new_line(i)?;
    Ok((
        i,
        Question {
            number,
            text,
            answers,
            reading,
            category,
        },
    ))
}

fn question_header(i: &str) -> IResult<&str, (u32, String)> {
    let (i, (_, num, _, _, category, _)) = tuple((
        tag("## Question "),
        map_res(digit1, to_int),
        char(' '),
        char('`'),
        take_until("`"),
        char('`'),
    ))(i)?;
    Ok((i, (num, category.into())))
}

fn to_int(i: &str) -> Result<u32, ParseIntError> {
    i.parse::<u32>()
}

fn new_line(i: &str) -> IResult<&str, char> {
    char('\n')(i)
}

fn paragraph(i: &str) -> IResult<&str, String> {
    let (i, text) = take_until("\n\n")(i)?;
    Ok((i, text.into()))
}

fn line(i: &str) -> IResult<&str, String> {
    let (i, text) = take_until("\n")(i)?;
    Ok((i, text.into()))
}

fn answers_header(i: &str) -> IResult<&str, &str> {
    tag("## Answers")(i)
}

fn answers(i: &str) -> IResult<&str, Vec<Answer>> {
    many_m_n(4, 4, answer)(i)
}

fn answer(i: &str) -> IResult<&str, Answer> {
    let (i, (checkbox, _, text, _)) = tuple((answer_checkbox, char(' '), line, char('\n')))(i)?;
    let is_correct = checkbox == "- [x]";
    Ok((i, Answer { text, is_correct }))
}

fn answer_checkbox(i: &str) -> IResult<&str, &str> {
    alt((tag("- [ ]"), tag("- [x]")))(i)
}

fn reading_header(i: &str) -> IResult<&str, Option<&str>> {
    opt(tag("## Reading"))(i)
}

fn reading(i: &str) -> IResult<&str, Option<String>> {
    let (i, reading) = opt(tuple((tag("- [here]("), take_until(")"), tag(")"))))(i)?;
    match reading {
        Some((_, txt, _)) => Ok((i, Some(txt.into()))),
        None => Ok((i, None)),
    }
}

#[cfg(test)]
mod test {
    use super::{
        answer, answer_checkbox, answers, answers_header, line, new_line, question_header, reading,
        reading_header,
    };
    use crate::parser::question;
    use crate::{Answer, Question};

    #[test]
    fn test_question_parser() {
        let input = r#"## Question 1 `Templates and Components`
Some text of the question.
Next part of the question.

## Answers
- [ ] Use and configure the teaser core component.
- [ ] Create a new custom component from scratch.
- [ ] Overlay the teaser core component.
- [x] Inherit from the teaser core component.

## Reading
- [here](reading/question-3-reading.md)

"#;
        assert_eq!(
            question(input),
            Ok((
                "",
                Question {
                    number: 1,
                    text: "Some text of the question.\nNext part of the question.".into(),
                    answers: vec![
                        Answer {
                            text: "Use and configure the teaser core component.".into(),
                            is_correct: false,
                        },
                        Answer {
                            text: "Create a new custom component from scratch.".into(),
                            is_correct: false,
                        },
                        Answer {
                            text: "Overlay the teaser core component.".into(),
                            is_correct: false,
                        },
                        Answer {
                            text: "Inherit from the teaser core component.".into(),
                            is_correct: true,
                        },
                    ],
                    reading: Some("reading/question-3-reading.md".into()),
                    category: "Templates and Components".into()
                }
            ))
        );
    }

    #[test]
    fn test_question_header_parser() {
        let input = "## Question 1 `Templates and Components`";
        assert_eq!(
            question_header(input),
            Ok(("", (1, "Templates and Components".into())))
        );
    }

    #[test]
    fn test_new_line_parser() {
        let input = r#"
"#;
        assert_eq!(new_line(input), Ok(("", '\n')));
    }

    #[test]
    fn test_line_parser() {
        let input = r#"Some text here
"#;
        assert_eq!(line(input), Ok(("\n", "Some text here".into())));
    }

    #[test]
    fn test_answer_parser() {
        let incorrect_answer = r#"- [ ] Some answer
"#;
        assert_eq!(
            answer(incorrect_answer),
            Ok((
                "",
                Answer {
                    text: "Some answer".into(),
                    is_correct: false,
                }
            ))
        );

        let correct_answer = r#"- [x] Some answer
"#;
        assert_eq!(
            answer(correct_answer),
            Ok((
                "",
                Answer {
                    text: "Some answer".into(),
                    is_correct: true,
                }
            ))
        );
    }

    #[test]
    fn test_answers_header_parser() {
        let input = r#"## Answers
"#;
        assert_eq!(answers_header(input), Ok(("\n", "## Answers")))
    }

    #[test]
    fn test_answer_checkbox() {
        let unchecked = "- [ ]";
        assert_eq!(answer_checkbox(unchecked), Ok(("", "- [ ]")));
        let checked = "- [x]";
        assert_eq!(answer_checkbox(checked), Ok(("", "- [x]")));
    }

    #[test]
    fn test_answers_parser() {
        let input = r#"- [ ] Use and configure the teaser core component.
- [ ] Create a new custom component from scratch.
- [ ] Overlay the teaser core component.
- [x] Inherit from the teaser core component.
"#;
        assert_eq!(
            answers(input),
            Ok((
                "",
                vec![
                    Answer {
                        text: "Use and configure the teaser core component.".into(),
                        is_correct: false,
                    },
                    Answer {
                        text: "Create a new custom component from scratch.".into(),
                        is_correct: false,
                    },
                    Answer {
                        text: "Overlay the teaser core component.".into(),
                        is_correct: false,
                    },
                    Answer {
                        text: "Inherit from the teaser core component.".into(),
                        is_correct: true,
                    }
                ]
            ))
        );
    }

    #[test]
    fn test_reading_header_parser() {
        let reading_header_present = r#"## Reading
"#;
        assert_eq!(
            reading_header(reading_header_present),
            Ok(("\n", Some("## Reading")))
        );

        let lack_of_header = r#""#;
        assert_eq!(reading_header(lack_of_header), Ok(("", None)))
    }

    #[test]
    fn test_reading_parser() {
        let reading_present = r#"- [here](reading/question-3-reading.md)
"#;
        assert_eq!(
            reading(reading_present),
            Ok(("\n", Some("reading/question-3-reading.md".into())))
        );

        // let lack_of_reading = r#""#;
        // assert_eq!(reading_header(lack_of_reading), Ok(("", None)))
    }
}
