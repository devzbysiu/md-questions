use crate::{Answer, MdQuestions, Question, QuestionMetadata, QuestionType};
use log::debug;
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
const NO_CHECKBOX: &str = "- ";

pub(crate) fn questions(i: &str) -> IResult<&str, MdQuestions> {
    let (i, questions) = many1(question)(i)?;
    Ok((i, MdQuestions::new(questions)))
}

fn question(i: &str) -> IResult<&str, Question> {
    let _ = pretty_env_logger::try_init();
    let (i, metadata) = opt(question_metadata)(i)?;
    if let Some(question_metadata) = metadata {
        match question_metadata.question_type() {
            QuestionType::Checkbox => {
                debug!("found question type: checkbox");
                let (i, _) = new_line(i)?;
                let (i, _) = new_line(i)?;
                let (i, question) = checkbox_question(i)?;
                return Ok((i, question));
            }
        }
    }
    let (i, question) = checkbox_question(i)?;
    Ok((i, question))
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
        _ => panic!("not supported key in question metadata: {}", key),
    }
    Ok((i, metadata))
}

fn checkbox_question(i: &str) -> IResult<&str, Question> {
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
    let (i, reading) = opt_reading_header(i)?;
    let (i, _) = opt_new_line(i)?;
    let (i, _) = opt_new_line(i)?;
    let (i, _) = horizontal_rule(i)?;
    let (i, _) = new_line(i)?;
    let (i, _) = new_line(i)?;
    let question = Question {
        number,
        text,
        answers,
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
        let (input, marker) = opt_marker(input)?;
        debug!("found marker: {}", marker);
        if marker.to_lowercase() != "ignore" {
            break (input, num, category);
        }
        debug!("ignoring");
        let (input, _) = take_until("---")(input)?;
        let (input, _) = horizontal_rule(input)?;
        let (input, _) = new_line(input)?;
        let (input, _) = new_line(input)?;
        debug!("rest of te input");
        i = input;
    };

    Ok((input, (num, category)))
}

fn number_and_category(i: &str) -> IResult<&str, (u32, String)> {
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

fn opt_marker(i: &str) -> IResult<&str, String> {
    let (i, marker) = opt(tuple((char(' '), char('`'), take_until("`"), char('`'))))(i)?;
    match marker {
        Some((_, _, marker, _)) => Ok((i, marker.into())),
        None => Ok((i, "".into())),
    }
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
    many1(answer)(i)
}

fn answer(i: &str) -> IResult<&str, Answer> {
    let (i, (checkbox, text, _)) =
        tuple((alt((answer_checkbox, tag(NO_CHECKBOX))), line, char('\n')))(i)?;
    let is_correct = matches!(checkbox, CHECKED | NO_CHECKBOX);
    Ok((i, Answer::new(text, is_correct)))
}

fn answer_checkbox(i: &str) -> IResult<&str, &str> {
    alt((tag(UNCHECKED), tag(CHECKED)))(i)
}

fn opt_reading_header(i: &str) -> IResult<&str, Option<String>> {
    let (i, reading) = opt(tuple((tag("## [Reading]("), take_until(")"), tag(")"))))(i)?;
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
                    Question::default()
                        .with_number(1)
                        .with_text("A developer needs to create a banner component. This component shows an image \
                          across the full width of the page. A title is shown on top of the image. This text can be \
                          aligned to the left, middle, or right. The core components feature a teaser component which \
                          matches almost all requirements, but not all. What is the most maintainable way for the \
                          developer to implement these requirements?")
                        .with_answer(Answer::new("Use and configure the teaser core component.", false))
                        .with_answer(Answer::new("Create a new custom component from scratch.", false))
                        .with_answer(Answer::new("Overlay the teaser core component.", false))
                        .with_answer(Answer::new("Inherit from the teaser core component.", true))
                        .with_category("Templates and Components"),
                    Question::default()
                        .with_number(2)
                        .with_text("A developer is working on a complex project with multiple bundles. One bundle \
                          provides an OSGi service for other bundles. Which two options are necessary to ensure that \
                          the other bundles can reference that OSGi service? (Choose two.)")
                        .with_answer(Answer::new( "The bundles consuming the service need to import the fully \
                            qualified name of the service interface.", true))
                        .with_answer(Answer::new("The service needs to correctly declare metatype information.", false))
                        .with_answer(Answer::new("The bundle providing the service needs to contain a whitelist of \
                            allowed consumer bundles.", false))
                        .with_answer(Answer::new("The bundle providing the service needs to contain an adequate SCR \
                            descriptor file.", false))
                        .with_answer(Answer::new("The bundle providing the service needs to export the java package of \
                            the service interface.", true))
                        .with_category("OSGi Services"),
                    Question::default()
                        .with_number(3)
                        .with_text("The structure section of an editable template has a locked component. What happens \
                          to the content of that component when a developer unlocks it?")
                        .with_answer(Answer::new("The content stays in the same place but it ignored on pages using \
                            the template.", false))
                        .with_answer(Answer::new("The content is moved to the initial section of the editable template.", true))
                        .with_answer(Answer::new("The content is deleted after confirmation from the template author.", false))
                        .with_answer(Answer::new("The content is copied to the initial section of the editable template.", false))
                        .with_reading("reading/question-3-reading.md")
                        .with_category("Templates and Components")
                ])
            ))
        );
    }

    #[test]
    fn test_question_parser_with_checkbox_question() {
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
                Question::default()
                    .with_number(1)
                    .with_text("A developer needs to create a banner component. This component shows an image across the full width of the page. A title is shown on top of the image. This text can be aligned to the left, middle, or right. The core components feature a teaser component which matches almost all requirements, but not all. What is the most maintainable way for the developer to implement these requirements?")
                    .with_answer(Answer::new("Use and configure the teaser core component.", false))
                    .with_answer(Answer::new("Create a new custom component from scratch.", false))
                    .with_answer(Answer::new("Overlay the teaser core component.", false))
                    .with_answer(Answer::new("Inherit from the teaser core component.", true))
                    .with_category("Templates and Components")
                    .with_reading("reading/question-3-reading.md")
            ))
        );
    }

    #[test]
    fn test_question_parser_with_question_metadata() {
        let _ = pretty_env_logger::try_init();
        let input = r#"[//]: # (type: checkbox)

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
                Question::default()
                    .with_number(1)
                    .with_text("A developer needs to create a banner component. This component shows an image across the full width of the page. A title is shown on top of the image. This text can be aligned to the left, middle, or right. The core components feature a teaser component which matches almost all requirements, but not all. What is the most maintainable way for the developer to implement these requirements?")
                    .with_answer(Answer::new("Use and configure the teaser core component.", false))
                    .with_answer(Answer::new("Create a new custom component from scratch.", false))
                    .with_answer(Answer::new("Overlay the teaser core component.", false))
                    .with_answer(Answer::new("Inherit from the teaser core component.", true))
                    .with_category("Templates and Components")
                    .with_reading("reading/question-3-reading.md")
            ))
        );
    }

    #[test]
    fn test_question_parser_with_open_question() {
        let _ = pretty_env_logger::try_init();
        let input = r#"## Question 1 `Trees`
Describe rooted tree.

## Answers
- Rooted tree is a tree which all edges are going in to the root node or all of them go out of the tree node.

## [Reading](reading/question-3-reading.md)

---

"#;
        assert_eq!(
            question(input),
            Ok((
                "",
                Question::default()
                    .with_number(1)
                    .with_text("Describe rooted tree.")
                    .with_answer(Answer::new("Rooted tree is a tree which all edges are going in to the root node or all of them go out of the tree node.", true))
                    .with_category("Trees")
                    .with_reading("reading/question-3-reading.md")
            ))
        );
    }

    #[test]
    fn test_question_metadata_parser() {
        let _ = pretty_env_logger::try_init();
        assert_eq!(
            question_metadata(r#"[//]: # (type: checkbox)"#),
            Ok((
                "",
                QuestionMetadata::default().with_question_type(QuestionType::Checkbox)
            ))
        );
    }

    #[test]
    #[should_panic]
    fn test_question_metadata_parser_with_incorrect_type() {
        let _ = pretty_env_logger::try_init();
        assert_eq!(
            question_metadata(r#"[//]: # (type: incorrect)"#),
            Ok((
                "",
                QuestionMetadata::default().with_question_type(QuestionType::Checkbox)
            ))
        );
    }

    #[test]
    #[should_panic]
    fn test_question_metadata_parser_with_incorrect_key() {
        let _ = pretty_env_logger::try_init();
        assert_eq!(
            question_metadata(r#"[//]: # (incorrect-key: checkbox)"#),
            Ok((
                "",
                QuestionMetadata::default().with_question_type(QuestionType::Checkbox)
            ))
        );
    }

    #[test]
    fn test_question_header_parser() {
        let _ = pretty_env_logger::try_init();
        assert_eq!(
            question_header("## Question 1 `Templates and Components`"),
            Ok(("", (1, "Templates and Components".into())))
        );
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
            answer("- [ ] Some answer\n"),
            Ok(("", Answer::new("Some answer", false)))
        );
        assert_eq!(
            answer("- [X] Some answer\n"),
            Ok(("", Answer::new("Some answer", true)))
        );
        assert_eq!(
            answer("- Some answer\n"),
            Ok(("", Answer::new("Some answer", true)))
        );
    }

    #[test]
    fn test_answers_header_parser() {
        let _ = pretty_env_logger::try_init();
        assert_eq!(answers_header("## Answers\n"), Ok(("\n", "## Answers")))
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
            answers(input),
            Ok((
                "",
                vec![
                    Answer::new("Use and configure the teaser core component.", false),
                    Answer::new("Create a new custom component from scratch.", false),
                    Answer::new("Overlay the teaser core component.", false),
                    Answer::new("Inherit from the teaser core component.", true)
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
            answers(input),
            Ok((
                "",
                vec![Answer::new(
                    "Use and configure the teaser core component.",
                    true
                )]
            ))
        );
    }

    #[test]
    fn test_reading_header_parser() {
        let _ = pretty_env_logger::try_init();
        assert_eq!(
            opt_reading_header("## [Reading](reading/question-3-reading.md)\n"),
            Ok(("\n", Some("reading/question-3-reading.md".into())))
        );
        assert_eq!(opt_reading_header(""), Ok(("", None)))
    }

    #[test]
    fn test_horizontal_rule_parser() {
        let _ = pretty_env_logger::try_init();
        assert_eq!(horizontal_rule("---\n"), Ok(("\n", "---")));
    }
}
