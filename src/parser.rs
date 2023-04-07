use crate::{
    ClosedAnswer, ClosedQuestion, MdQuestions, OpenAnswer, OpenQuestion, Question,
    QuestionMetadata, QuestionType,
};

use log::{debug, warn};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{char, digit1};
use nom::combinator::{map_res, opt};
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;
use std::num::ParseIntError;

const CHECKED: &str = "- [X] ";
const UNCHECKED: &str = "- [ ] ";

pub(crate) fn questions(i: &str) -> IResult<&str, MdQuestions> {
    let (i, questions) = many1(question)(i)?;
    Ok((i, MdQuestions::new(questions)))
}

fn question(i: &str) -> IResult<&str, Question> {
    let _ = pretty_env_logger::try_init();
    let (i, metadata) = opt(question_metadata)(i)?;
    if let Some(metadata) = metadata {
        debug!("found question with type: {:?}", metadata.question_type());
        let (i, _) = empty_line(i)?;
        match metadata.question_type() {
            QuestionType::Closed => {
                let (i, question) = closed_question(i)?;
                return Ok((i, Question::from_closed(question)));
            }
            QuestionType::Open => {
                let (i, question) = open_question(i)?;
                return Ok((i, Question::from_open(question)));
            }
        }
    }
    let (i, question) = closed_question(i)?;
    Ok((i, Question::from_closed(question)))
}

fn question_metadata(i: &str) -> IResult<&str, QuestionMetadata> {
    let (i, (_, key, _, value, _)) = tuple((
        tag("[//]: # ("),
        take_until(":"),
        tag(": "),
        take_until(")"),
        tag(")"),
    ))(i)?;
    let mut metadata = QuestionMetadata::default();
    match key {
        "type" => metadata = metadata.with_question_type(QuestionType::from(value)),
        _ => panic!("not supported key in question metadata: {key}"),
    }
    Ok((i, metadata))
}

fn closed_question(i: &str) -> IResult<&str, ClosedQuestion> {
    let (i, (number, category)) = question_header(i)?;
    let (i, _) = new_line(i)?;
    let (i, text) = paragraph(i)?;
    let (i, _) = empty_line(i)?;
    let (i, _) = answers_header(i)?;
    let (i, _) = new_line(i)?;
    let (i, answers) = closed_answers(i)?;
    let (i, _) = new_line(i)?;
    // let (i, _) = new_line(i)?; // TODO: this should be here to be consistent
    let (i, reading) = opt(reading_header)(i)?;
    let (i, _) = opt(empty_line)(i)?;
    let (i, _) = horizontal_rule(i)?;
    let (i, _) = empty_line(i)?;
    let question = ClosedQuestion {
        number,
        text,
        answers,
        reading,
        category,
    };
    debug!("full question: {:#?}", question);
    Ok((i, question))
}

fn open_question(i: &str) -> IResult<&str, OpenQuestion> {
    let (i, (number, category)) = question_header(i)?;
    let (i, _) = new_line(i)?;
    let (i, text) = paragraph(i)?;
    let (i, _) = empty_line(i)?;
    let (i, _) = answer_header(i)?;
    let (i, _) = new_line(i)?;
    let (i, answer) = open_answer(i)?;
    let (i, _) = new_line(i)?;
    // let (i, _) = new_line(i)?; // TODO: this should be here to be consistent
    let (i, reading) = opt(reading_header)(i)?;
    let (i, _) = opt(empty_line)(i)?;
    let (i, _) = horizontal_rule(i)?;
    let (i, _) = empty_line(i)?;
    let question = OpenQuestion {
        number,
        text,
        answer,
        reading,
        category,
    };
    debug!("full question: {:#?}", question);
    Ok((i, question))
}

fn question_header(i: &str) -> IResult<&str, (u32, String)> {
    let mut i = i;
    let (input, num, category) = loop {
        debug!("loop with input: {}", i);
        let (input, (num, category)) = number_and_category(i)?;
        let (input, marker) = opt(marker)(input)?;
        debug!("found marker: {:#?}", marker);
        match marker {
            Some(marker) if marker.to_lowercase() != "ignore" => break (input, num, category),
            None => break (input, num, category),
            Some(marker) => warn!("not supported marker: {}", marker),
        }
        debug!("ignoring");
        let (input, _) = take_until("---")(input)?;
        let (input, _) = horizontal_rule(input)?;
        let (input, _) = empty_line(input)?;
        debug!("rest of te input");
        i = input;
    };

    Ok((input, (num, category)))
}

fn number_and_category(i: &str) -> IResult<&str, (u32, String)> {
    let (i, (_, num, _, category, _)) = tuple((
        tag("## Question "),
        map_res(digit1, to_int),
        tag(" `"),
        take_until("`"),
        char('`'),
    ))(i)?;
    Ok((i, (num, category.into())))
}

fn marker(i: &str) -> IResult<&str, String> {
    let (i, (_, marker, _)) = tuple((tag(" `"), take_until("`"), char('`')))(i)?;
    Ok((i, marker.into()))
}

fn to_int(i: &str) -> Result<u32, ParseIntError> {
    i.parse::<u32>()
}

fn empty_line(i: &str) -> IResult<&str, String> {
    let (i, _) = new_line(i)?;
    let (i, _) = new_line(i)?;
    Ok((i, "\n\n".into()))
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

fn answer_header(i: &str) -> IResult<&str, &str> {
    tag("## Answer")(i)
}

fn closed_answers(i: &str) -> IResult<&str, Vec<ClosedAnswer>> {
    many1(closed_answer)(i)
}

fn closed_answer(i: &str) -> IResult<&str, ClosedAnswer> {
    let (i, (checkbox, text, _)) = tuple((answer_checkbox, line, char('\n')))(i)?;
    let is_correct = matches!(checkbox, CHECKED);
    Ok((i, ClosedAnswer::new(text, is_correct)))
}

fn open_answer(i: &str) -> IResult<&str, OpenAnswer> {
    let (i, text) = paragraph(i)?;
    Ok((i, OpenAnswer::new(text)))
}

fn answer_checkbox(i: &str) -> IResult<&str, &str> {
    alt((tag(UNCHECKED), tag(CHECKED)))(i)
}

fn reading_header(i: &str) -> IResult<&str, String> {
    let (i, (_, txt, _)) = tuple((tag("## [Reading]("), take_until(")"), tag(")")))(i)?;
    Ok((i, txt.into()))
}

fn horizontal_rule(i: &str) -> IResult<&str, &str> {
    tag("---")(i)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::question;
    use nom::error::ErrorKind::TakeUntil;
    use nom::Err::Error;

    #[test]
    fn test_questions_parser() {
        let _ = pretty_env_logger::try_init();
        let input = r#"## Question 1 `Templates and Components`
A developer needs to create a banner component. This component shows an image across the full width of the page. A title is shown on top of the image. This text can be aligned to the left, middle, or right. The core components feature a teaser component which matches almost all requirements, but not all. What is the most maintainable way for the developer to implement these requirements?

## Answers
- [ ] Use and configure the teaser core component.
- [ ] Create a new custom component from scratch.
- [ ] Overlay the teaser core component.
- [X] Inherit from the teaser core component.

---

## Question 2 `OSGi Services`
A developer is working on a complex project with multiple bundles. One bundle provides an OSGi service for other bundles. Which two options are necessary to ensure that the other bundles can reference that OSGi service? (Choose two.)

## Answers
- [X] The bundles consuming the service need to import the fully qualified name of the service interface.
- [ ] The service needs to correctly declare metatype information.
- [ ] The bundle providing the service needs to contain a whitelist of allowed consumer bundles.
- [ ] The bundle providing the service needs to contain an adequate SCR descriptor file.
- [X] The bundle providing the service needs to export the java package of the service interface.

---

## Question 3 `Templates and Components`
The structure section of an editable template has a locked component. What happens to the content of that component when a developer unlocks it?

## Answers
- [ ] The content stays in the same place but it ignored on pages using the template.
- [X] The content is moved to the initial section of the editable template.
- [ ] The content is deleted after confirmation from the template author.
- [ ] The content is copied to the initial section of the editable template.

## [Reading](reading/question-3-reading.md)

---

"#;
        assert_eq!(
            questions(input),
            Ok((
                "",
                MdQuestions::new(vec![
                    Question::closed()
                        .with_number(1)
                        .with_text("A developer needs to create a banner component. This component shows an image \
                          across the full width of the page. A title is shown on top of the image. This text can be \
                          aligned to the left, middle, or right. The core components feature a teaser component which \
                          matches almost all requirements, but not all. What is the most maintainable way for the \
                          developer to implement these requirements?")
                        .with_answer(ClosedAnswer::new("Use and configure the teaser core component.", false))
                        .with_answer(ClosedAnswer::new("Create a new custom component from scratch.", false))
                        .with_answer(ClosedAnswer::new("Overlay the teaser core component.", false))
                        .with_answer(ClosedAnswer::new("Inherit from the teaser core component.", true))
                        .with_category("Templates and Components").into(),
                    Question::closed()
                        .with_number(2)
                        .with_text("A developer is working on a complex project with multiple bundles. One bundle \
                          provides an OSGi service for other bundles. Which two options are necessary to ensure that \
                          the other bundles can reference that OSGi service? (Choose two.)")
                        .with_answer(ClosedAnswer::new( "The bundles consuming the service need to import the fully \
                            qualified name of the service interface.", true))
                        .with_answer(ClosedAnswer::new("The service needs to correctly declare metatype information.", false))
                        .with_answer(ClosedAnswer::new("The bundle providing the service needs to contain a whitelist of \
                            allowed consumer bundles.", false))
                        .with_answer(ClosedAnswer::new("The bundle providing the service needs to contain an adequate SCR \
                            descriptor file.", false))
                        .with_answer(ClosedAnswer::new("The bundle providing the service needs to export the java package of \
                            the service interface.", true))
                        .with_category("OSGi Services").into(),
                    Question::closed()
                        .with_number(3)
                        .with_text("The structure section of an editable template has a locked component. What happens \
                          to the content of that component when a developer unlocks it?")
                        .with_answer(ClosedAnswer::new("The content stays in the same place but it ignored on pages using \
                            the template.", false))
                        .with_answer(ClosedAnswer::new("The content is moved to the initial section of the editable template.", true))
                        .with_answer(ClosedAnswer::new("The content is deleted after confirmation from the template author.", false))
                        .with_answer(ClosedAnswer::new("The content is copied to the initial section of the editable template.", false))
                        .with_reading("reading/question-3-reading.md")
                        .with_category("Templates and Components").into()
                ])
            ))
        );
    }

    #[test]
    fn test_question_parser_with_closed_question() {
        let _ = pretty_env_logger::try_init();
        let input = r#"## Question 1 `Templates and Components`
A developer needs to create a banner component. This component shows an image across the full width of the page. A title is shown on top of the image. This text can be aligned to the left, middle, or right. The core components feature a teaser component which matches almost all requirements, but not all. What is the most maintainable way for the developer to implement these requirements?

## Answers
- [ ] Use and configure the teaser core component.
- [ ] Create a new custom component from scratch.
- [ ] Overlay the teaser core component.
- [X] Inherit from the teaser core component.

## [Reading](reading/question-3-reading.md)

---

"#;
        assert_eq!(
            question(input),
            Ok((
                "",
                Question::closed()
                    .with_number(1)
                    .with_text("A developer needs to create a banner component. This component shows an image across the full width of the page. A title is shown on top of the image. This text can be aligned to the left, middle, or right. The core components feature a teaser component which matches almost all requirements, but not all. What is the most maintainable way for the developer to implement these requirements?")
                    .with_answer(ClosedAnswer::new("Use and configure the teaser core component.", false))
                    .with_answer(ClosedAnswer::new("Create a new custom component from scratch.", false))
                    .with_answer(ClosedAnswer::new("Overlay the teaser core component.", false))
                    .with_answer(ClosedAnswer::new("Inherit from the teaser core component.", true))
                    .with_category("Templates and Components")
                    .with_reading("reading/question-3-reading.md").into())
            )
        );
    }

    #[test]
    fn test_question_parser_with_question_metadata() {
        let _ = pretty_env_logger::try_init();
        let input = r#"[//]: # (type: closed)

## Question 1 `Templates and Components`
A developer needs to create a banner component. This component shows an image across the full width of the page. A title is shown on top of the image. This text can be aligned to the left, middle, or right. The core components feature a teaser component which matches almost all requirements, but not all. What is the most maintainable way for the developer to implement these requirements?

## Answers
- [ ] Use and configure the teaser core component.
- [ ] Create a new custom component from scratch.
- [ ] Overlay the teaser core component.
- [X] Inherit from the teaser core component.

## [Reading](reading/question-3-reading.md)

---

"#;
        assert_eq!(
            question(input),
            Ok((
                "",
                Question::closed()
                    .with_number(1)
                    .with_text("A developer needs to create a banner component. This component shows an image across the full width of the page. A title is shown on top of the image. This text can be aligned to the left, middle, or right. The core components feature a teaser component which matches almost all requirements, but not all. What is the most maintainable way for the developer to implement these requirements?")
                    .with_answer(ClosedAnswer::new("Use and configure the teaser core component.", false))
                    .with_answer(ClosedAnswer::new("Create a new custom component from scratch.", false))
                    .with_answer(ClosedAnswer::new("Overlay the teaser core component.", false))
                    .with_answer(ClosedAnswer::new("Inherit from the teaser core component.", true))
                    .with_category("Templates and Components")
                    .with_reading("reading/question-3-reading.md").into())
            )
        );
    }

    #[test]
    #[ignore] // TODO: Open question should be done in a different way
    fn test_question_parser_with_open_question() {
        let _ = pretty_env_logger::try_init();
        let input = r#"## Question 1 `Trees`
Describe rooted tree.

## Answer
- Rooted tree is a tree which all edges are going in to the root node or all of them go out of the tree node.

## [Reading](reading/question-3-reading.md)

---

"#;
        assert_eq!(
            question(input),
            Ok((
                "",
                Question::closed()
                    .with_number(1)
                    .with_text("Describe rooted tree.")
                    .with_answer(ClosedAnswer::new("Rooted tree is a tree which all edges are going in to the root node or all of them go out of the tree node.", true))
                    .with_category("Trees")
                    .with_reading("reading/question-3-reading.md").into())
            )
        );
    }

    #[test]
    fn test_question_metadata_parser() {
        let _ = pretty_env_logger::try_init();
        assert_eq!(
            question_metadata(r#"[//]: # (type: closed)"#),
            Ok((
                "",
                QuestionMetadata::default().with_question_type(QuestionType::Closed)
            ))
        );
    }

    #[test]
    #[should_panic]
    fn test_question_metadata_parser_with_incorrect_type() {
        let _ = pretty_env_logger::try_init();
        question_metadata(r#"[//]: # (type: incorrect)"#).unwrap(); // should panic
    }

    #[test]
    #[should_panic]
    fn test_question_metadata_parser_with_incorrect_key() {
        let _ = pretty_env_logger::try_init();
        question_metadata(r#"[//]: # (incorrect-key: closed)"#).unwrap(); // should panic
    }

    #[test]
    fn test_question_header_parser_with_correct_input() {
        let _ = pretty_env_logger::try_init();
        assert_eq!(
            question_header("## Question 1 `Templates and Components`"),
            Ok(("", (1, "Templates and Components".into())))
        );
    }

    #[test]
    fn test_number_and_category_parser_with_correct_input() {
        let _ = pretty_env_logger::try_init();
        assert_eq!(
            number_and_category("## Question 1 `OSGi Services`"),
            Ok(("", (1, "OSGi Services".into())))
        );
    }

    #[test]
    #[should_panic]
    fn test_number_and_category_parser_without_number() {
        let _ = pretty_env_logger::try_init();
        number_and_category("## Question `OSGi Services`").unwrap(); // should panic
    }

    #[test]
    #[should_panic]
    fn test_number_and_category_parser_without_category() {
        let _ = pretty_env_logger::try_init();
        number_and_category("## Question 1").unwrap(); // should panic
    }

    #[test]
    fn test_number_and_category_parser_with_empty_category() {
        let _ = pretty_env_logger::try_init();
        assert_eq!(
            number_and_category("## Question 1 ``"),
            Ok(("", (1, String::new())))
        );
    }

    #[test]
    fn test_number_and_category_parser_with_max_number() {
        let _ = pretty_env_logger::try_init();
        assert_eq!(
            number_and_category("## Question 4294967295 `Category`"),
            Ok(("", (4_294_967_295, "Category".into())))
        );
    }

    #[test]
    #[should_panic]
    fn test_number_and_category_parser_with_too_big_number() {
        let _ = pretty_env_logger::try_init();
        number_and_category("## Question 4294967296``").unwrap(); // should panic
    }

    #[test]
    fn test_marker_parser_with_correct_input() {
        let _ = pretty_env_logger::try_init();
        assert_eq!(marker(" `Ignored`"), Ok(("", "Ignored".into())));
    }

    #[test]
    #[should_panic]
    fn test_marker_parser_with_not_opened_marker() {
        let _ = pretty_env_logger::try_init();
        marker(" Ignored`").unwrap(); // should panic
    }

    #[test]
    #[should_panic]
    fn test_marker_parser_without_space_in_front() {
        let _ = pretty_env_logger::try_init();
        marker("`Ignored`").unwrap(); // should panic
    }

    #[test]
    #[should_panic]
    fn test_marker_parser_with_not_closed_marker() {
        let _ = pretty_env_logger::try_init();
        marker(" `Ignored").unwrap(); // should panic
    }

    #[test]
    fn test_question_header_parser_with_ignored_question() {
        let _ = pretty_env_logger::try_init();
        assert_eq!(
            question_header("## Question 1 `OSGi Services` `Ignore`"),
            Err(Error(nom::error::Error::new("", TakeUntil)))
        );
    }

    #[test]
    fn test_space_between_parser() {
        let _ = pretty_env_logger::try_init();
        assert_eq!(empty_line("\n\n"), Ok(("", "\n\n".into())));
    }

    #[test]
    #[should_panic]
    fn test_space_between_parser_with_text_between_new_lines() {
        let _ = pretty_env_logger::try_init();
        empty_line("\nsome text\n").unwrap(); // should panic
    }

    #[test]
    fn test_new_line_parser() {
        let _ = pretty_env_logger::try_init();
        assert_eq!(new_line("\n"), Ok(("", '\n')));
    }

    #[test]
    fn test_line_parser() {
        let _ = pretty_env_logger::try_init();
        assert_eq!(
            line("Some text here\n"),
            Ok(("\n", "Some text here".into()))
        );
    }

    #[test]
    fn test_answer_parser() {
        let _ = pretty_env_logger::try_init();
        assert_eq!(
            closed_answer("- [ ] Some answer\n"),
            Ok(("", ClosedAnswer::new("Some answer", false)))
        );
        assert_eq!(
            closed_answer("- [X] Some answer\n"),
            Ok(("", ClosedAnswer::new("Some answer", true)))
        );
    }

    #[test]
    fn test_answers_header_parser() {
        let _ = pretty_env_logger::try_init();
        assert_eq!(answers_header("## Answers\n"), Ok(("\n", "## Answers")));
    }

    #[test]
    fn test_answer_checkbox() {
        let _ = pretty_env_logger::try_init();
        assert_eq!(answer_checkbox("- [ ] "), Ok(("", "- [ ] ")));
        assert_eq!(answer_checkbox("- [X] "), Ok(("", "- [X] ")));
    }

    #[test]
    fn test_answers_parser_with_many_answers() {
        let _ = pretty_env_logger::try_init();
        let input = r#"- [ ] Use and configure the teaser core component.
- [ ] Create a new custom component from scratch.
- [ ] Overlay the teaser core component.
- [X] Inherit from the teaser core component.
"#;
        assert_eq!(
            closed_answers(input),
            Ok((
                "",
                vec![
                    ClosedAnswer::new("Use and configure the teaser core component.", false),
                    ClosedAnswer::new("Create a new custom component from scratch.", false),
                    ClosedAnswer::new("Overlay the teaser core component.", false),
                    ClosedAnswer::new("Inherit from the teaser core component.", true)
                ]
            ))
        );
    }

    #[test]
    fn test_answers_parser_with_one_answer() {
        let _ = pretty_env_logger::try_init();
        let input = r#"- [X] Use and configure the teaser core component.
"#;
        assert_eq!(
            closed_answers(input),
            Ok((
                "",
                vec![ClosedAnswer::new(
                    "Use and configure the teaser core component.",
                    true
                )]
            ))
        );
    }

    #[test]
    fn test_reading_header_parser_with_correct_input() {
        let _ = pretty_env_logger::try_init();
        assert_eq!(
            reading_header("## [Reading](reading/question-3-reading.md)\n"),
            Ok(("\n", "reading/question-3-reading.md".into()))
        );
    }

    #[test]
    #[should_panic]
    fn test_reading_header_parser_with_url_not_closed() {
        let _ = pretty_env_logger::try_init();
        reading_header("## [Reading](reading/question-3-reading.md\n").unwrap();
        // should panic
    }

    #[test]
    #[should_panic]
    fn test_reading_header_parser_with_url_not_opened() {
        let _ = pretty_env_logger::try_init();
        reading_header("## [Reading]reading/question-3-reading.md)\n").unwrap();
        // should panic
    }

    #[test]
    #[should_panic]
    fn test_reading_header_parser_with_label_not_opened() {
        let _ = pretty_env_logger::try_init();
        reading_header("## Reading](reading/question-3-reading.md)\n").unwrap();
        // should panic
    }

    #[test]
    #[should_panic]
    fn test_reading_header_parser_with_label_not_closed() {
        let _ = pretty_env_logger::try_init();
        reading_header("## [Reading(reading/question-3-reading.md)\n").unwrap();
        // should panic
    }

    #[test]
    fn test_reading_header_parser_with_broken_reading_header() {
        let _ = pretty_env_logger::try_init();
        let res = reading_header("## [Reading-broken](reading/question-3-reading.md)\n");
        assert!(res.is_err());

        let res = reading_header("## [Reading-broken](reading/question-3-reading.md)\n");
        assert!(res.is_err());
    }

    #[test]
    fn test_reading_header_parser_with_empty_url() {
        let _ = pretty_env_logger::try_init();
        assert_eq!(
            reading_header("## [Reading]()\n"),
            Ok(("\n", String::new()))
        );
    }

    #[test]
    fn test_horizontal_rule_parser() {
        let _ = pretty_env_logger::try_init();
        assert_eq!(horizontal_rule("---\n"), Ok(("\n", "---")));
    }
}
