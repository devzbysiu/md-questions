use crate::Answer;
use crate::Question;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{char, digit1};
use nom::combinator::map_res;
use nom::multi::many_m_n;
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
    let (i, _) = answers_header(i)?;
    let (i, answers) = answers(i)?;
    let (i, _) = new_line(i)?;
    let (i, category) = category(i)?;
    Ok((
        i,
        Question {
            number,
            text,
            answers,
            reading: None,
            category,
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

fn answers_header(i: &str) -> IResult<&str, &str> {
    let (i, (header, _)) = tuple((tag("## Answers"), char('\n')))(i)?;
    Ok((i, header))
}

fn answers(i: &str) -> IResult<&str, Vec<Answer>> {
    many_m_n(4, 4, answer)(i)
}

fn answer(i: &str) -> IResult<&str, Answer> {
    let (i, (checkbox, _, text, _)) = tuple((answer_checkbox, char(' '), text, char('\n')))(i)?;
    let mut is_correct = false;
    if checkbox == "- [x]" {
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

fn answer_checkbox(i: &str) -> IResult<&str, &str> {
    alt((tag("- [ ]"), tag("- [x]")))(i)
}

fn category(i: &str) -> IResult<&str, String> {
    let (i, (_, category)) = tuple((tag("> "), text))(i)?;
    Ok((i, category))
}

#[cfg(test)]
mod test {
    use super::{
        answer, answer_checkbox, answers, answers_header, category, new_line, question_header, text,
    };
    use crate::parser::question;
    use crate::{Answer, Question};

    #[test]
    fn test_question_parser() {
        let input = r#"## Question 1
Some text of the question

## Answers
- [ ] Use and configure the teaser core component.
- [ ] Create a new custom component from scratch.
- [ ] Overlay the teaser core component.
- [x] Inherit from the teaser core component.

> Templates and Components
"#;
        assert_eq!(
            question(input),
            Ok((
                "\n",
                Question {
                    number: 1,
                    text: "Some text of the question".into(),
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
                    reading: None,
                    category: "Templates and Components".into()
                }
            ))
        );
    }

    #[test]
    fn test_question_header_parser() {
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
        let input = r#"- [ ] Some answer
"#;
        assert_eq!(
            answer(input),
            Ok((
                "",
                Answer {
                    text: "Some answer".into(),
                    is_correct: false,
                }
            ))
        );

        let input = r#"- [x] Some answer
"#;
        assert_eq!(
            answer(input),
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
        assert_eq!(answers_header(input), Ok(("", "## Answers")))
    }

    #[test]
    fn test_answer_checkbox() {
        let input = "- [ ]";
        assert_eq!(answer_checkbox(input), Ok(("", "- [ ]")));
        let input = "- [x]";
        assert_eq!(answer_checkbox(input), Ok(("", "- [x]")));
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
        )
    }

    #[test]
    fn test_category_parser() {
        let input = r#"> Templates and Components
"#;
        assert_eq!(
            category(input),
            Ok(("\n", "Templates and Components".into()))
        )
    }
}
