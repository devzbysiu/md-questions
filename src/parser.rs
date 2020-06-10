use crate::{Answer, Question, Questions};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{char, digit1};
use nom::combinator::{map_res, opt};
use nom::multi::{many1, many_m_n};
use nom::sequence::tuple;
use nom::IResult;
use std::num::ParseIntError;

#[allow(dead_code)] // TODO: remove
fn questions(i: &str) -> IResult<&str, Questions> {
    let (i, questions) = many1(question)(i)?;
    Ok((i, Questions::new(questions)))
}

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
    let (i, _) = opt_reading_header(i)?;
    let (i, _) = opt_new_line(i)?;
    let (i, reading) = opt_reading(i)?;
    let (i, _) = opt_new_line(i)?;
    let (i, _) = opt_new_line(i)?;
    let (i, _) = horizontal_rule(i)?;
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

fn opt_new_line(i: &str) -> IResult<&str, Option<char>> {
    opt(char('\n'))(i)
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
    many_m_n(4, 5, answer)(i)
}

fn answer(i: &str) -> IResult<&str, Answer> {
    let (i, (checkbox, _, text, _)) = tuple((answer_checkbox, char(' '), line, char('\n')))(i)?;
    let is_correct = checkbox == "- [x]";
    Ok((i, Answer { text, is_correct }))
}

fn answer_checkbox(i: &str) -> IResult<&str, &str> {
    alt((tag("- [ ]"), tag("- [x]")))(i)
}

fn opt_reading_header(i: &str) -> IResult<&str, Option<&str>> {
    opt(tag("## Reading"))(i)
}

fn opt_reading(i: &str) -> IResult<&str, Option<String>> {
    let (i, reading) = opt(tuple((tag("- [here]("), take_until(")"), tag(")"))))(i)?;
    match reading {
        Some((_, txt, _)) => Ok((i, Some(txt.into()))),
        None => Ok((i, None)),
    }
}

fn horizontal_rule(i: &str) -> IResult<&str, &str> {
    tag("---")(i)
}

#[cfg(test)]
mod test {
    use super::{
        answer, answer_checkbox, answers, answers_header, horizontal_rule, line, new_line,
        opt_reading, opt_reading_header, question_header, questions,
    };
    use crate::parser::question;
    use crate::{Answer, Question, Questions};

    #[test]
    fn test_questions_parser() {
        let input = r#"## Question 1 `Templates and Components`
A developer needs to create a banner component. This component shows an image across the full width of the page. A title is shown on top of the image. This text can be aligned to the left, middle, or right. The core components feature a teaser component which matches almost all requirements, but not all. What is the most maintainable way for the developer to implement these requirements?

## Answers
- [ ] Use and configure the teaser core component.
- [ ] Create a new custom component from scratch.
- [ ] Overlay the teaser core component.
- [x] Inherit from the teaser core component.

---

## Question 2 `OSGi Services`
A developer is working on a complex project with multiple bundles. One bundle provides an OSGi service for other bundles. Which two options are necessary to ensure that the other bundles can reference that OSGi service? (Choose two.)

## Answers
- [x] The bundles consuming the service need to import the fully qualified name of the service interface.
- [ ] The service needs to correctly declare metatype information.
- [ ] The bundle providing the service needs to contain a whitelist of allowed consumer bundles.
- [ ] The bundle providing the service needs to contain an adequate SCR descriptor file.
- [x] The bundle providing the service needs to export the java package of the service interface.

---

## Question 3 `Templates and Components`
The structure section of an editable template has a locked component. What happens to the content of that component when a developer unlocks it?

## Answers
- [ ] The content stays in the same place but it ignored on pages using the template.
- [x] The content is moved to the initial section of the editable template.
- [ ] The content is deleted after confirmation from the template author.
- [ ] The content is copied to the initial section of the editable template.

## Reading
- [here](reading/question-3-reading.md)

---

"#;
        assert_eq!(
            questions(input),
            Ok((
                "",
                Questions {
                    questions: vec![
                        Question {
                            number: 1,
                            text: "A developer needs to create a banner component. This component shows an image across the full width of the page. A title is shown on top of the image. This text can be aligned to the left, middle, or right. The core components feature a teaser component which matches almost all requirements, but not all. What is the most maintainable way for the developer to implement these requirements?".into(),
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
                        },
                        Question {
                            number: 2,
                            text: "A developer is working on a complex project with multiple bundles. One bundle provides an OSGi service for other bundles. Which two options are necessary to ensure that the other bundles can reference that OSGi service? (Choose two.)".into(),
                            answers: vec![
                                Answer {
                                    text: "The bundles consuming the service need to import the fully qualified name of the service interface.".into(),
                                    is_correct: true,
                                },
                                Answer {
                                    text: "The service needs to correctly declare metatype information.".into(),
                                    is_correct: false,
                                },
                                Answer {
                                    text: "The bundle providing the service needs to contain a whitelist of allowed consumer bundles.".into(),
                                    is_correct: false,
                                },
                                Answer {
                                    text: "The bundle providing the service needs to contain an adequate SCR descriptor file.".into(),
                                    is_correct: false,
                                },
                                Answer {
                                    text: "The bundle providing the service needs to export the java package of the service interface.".into(),
                                    is_correct: true,
                                }
                            ],
                            reading: None,
                            category: "OSGi Services".into()
                        },
                        Question {
                            number: 3,
                            text: "The structure section of an editable template has a locked component. What happens to the content of that component when a developer unlocks it?".into(),
                            answers: vec![
                                Answer {
                                    text: "The content stays in the same place but it ignored on pages using the template.".into(),
                                    is_correct: false,
                                },
                                Answer {
                                    text: "The content is moved to the initial section of the editable template.".into(),
                                    is_correct: true,
                                },
                                Answer {
                                    text: "The content is deleted after confirmation from the template author.".into(),
                                    is_correct: false,
                                },
                                Answer {
                                    text: "The content is copied to the initial section of the editable template.".into(),
                                    is_correct: false,
                                },
                            ],
                            reading: Some("reading/question-3-reading.md".into()),
                            category: "Templates and Components".into()
                        },
                    ]
                }
            ))
        );
    }

    #[test]
    fn test_question_parser() {
        let input = r#"## Question 1 `Templates and Components`
A developer needs to create a banner component. This component shows an image across the full width of the page. A title is shown on top of the image. This text can be aligned to the left, middle, or right. The core components feature a teaser component which matches almost all requirements, but not all. What is the most maintainable way for the developer to implement these requirements?

## Answers
- [ ] Use and configure the teaser core component.
- [ ] Create a new custom component from scratch.
- [ ] Overlay the teaser core component.
- [x] Inherit from the teaser core component.

## Reading
- [here](reading/question-3-reading.md)

---

"#;
        assert_eq!(
            question(input),
            Ok((
                "",
                Question {
                    number: 1,
                    text: "A developer needs to create a banner component. This component shows an image across the full width of the page. A title is shown on top of the image. This text can be aligned to the left, middle, or right. The core components feature a teaser component which matches almost all requirements, but not all. What is the most maintainable way for the developer to implement these requirements?".into(),
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
            opt_reading_header(reading_header_present),
            Ok(("\n", Some("## Reading")))
        );

        let lack_of_header = r#""#;
        assert_eq!(opt_reading_header(lack_of_header), Ok(("", None)))
    }

    #[test]
    fn test_reading_parser() {
        let reading_present = r#"- [here](reading/question-3-reading.md)
"#;
        assert_eq!(
            opt_reading(reading_present),
            Ok(("\n", Some("reading/question-3-reading.md".into())))
        );

        let lack_of_reading = r#""#;
        assert_eq!(opt_reading(lack_of_reading), Ok(("", None)))
    }

    #[test]
    fn test_horizontal_rule_parser() {
        let input = r#"---
"#;
        assert_eq!(horizontal_rule(input), Ok(("\n", "---")));
    }
}
