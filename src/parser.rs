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
pub(crate) fn questions(i: &str) -> IResult<&str, Questions> {
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
    Ok((i, Answer::new(text, is_correct)))
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
                Questions::new(vec![
                    Question::default()
                        .with_number(1)
                        .with_text("A developer needs to create a banner component. This component shows an image across the full width of the page. A title is shown on top of the image. This text can be aligned to the left, middle, or right. The core components feature a teaser component which matches almost all requirements, but not all. What is the most maintainable way for the developer to implement these requirements?")
                        .with_answer(Answer::new("Use and configure the teaser core component.", false))
                        .with_answer(Answer::new("Create a new custom component from scratch.", false))
                        .with_answer(Answer::new("Overlay the teaser core component.", false))
                        .with_answer(Answer::new("Inherit from the teaser core component.", true))
                        .with_category("Templates and Components"),
                    Question::default()
                        .with_number(2)
                        .with_text("A developer is working on a complex project with multiple bundles. One bundle provides an OSGi service for other bundles. Which two options are necessary to ensure that the other bundles can reference that OSGi service? (Choose two.)")
                        .with_answer(Answer::new( "The bundles consuming the service need to import the fully qualified name of the service interface.", true))
                        .with_answer(Answer::new("The service needs to correctly declare metatype information.", false))
                        .with_answer(Answer::new("The bundle providing the service needs to contain a whitelist of allowed consumer bundles.", false))
                        .with_answer(Answer::new("The bundle providing the service needs to contain an adequate SCR descriptor file.", false))
                        .with_answer(Answer::new("The bundle providing the service needs to export the java package of the service interface.", true))
                        .with_category("OSGi Services"),
                    Question::default()
                        .with_number(3)
                        .with_text("The structure section of an editable template has a locked component. What happens to the content of that component when a developer unlocks it?")
                        .with_answer(Answer::new("The content stays in the same place but it ignored on pages using the template.", false))
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
    fn test_question_header_parser() {
        assert_eq!(
            question_header("## Question 1 `Templates and Components`"),
            Ok(("", (1, "Templates and Components".into())))
        );
    }

    #[test]
    fn test_new_line_parser() {
        assert_eq!(new_line("\n"), Ok(("", '\n')));
    }

    #[test]
    fn test_line_parser() {
        assert_eq!(
            line("Some text here\n"),
            Ok(("\n", "Some text here".into()))
        );
    }

    #[test]
    fn test_answer_parser() {
        assert_eq!(
            answer("- [ ] Some answer\n"),
            Ok(("", Answer::new("Some answer", false)))
        );
        assert_eq!(
            answer("- [x] Some answer\n"),
            Ok(("", Answer::new("Some answer", true)))
        );
    }

    #[test]
    fn test_answers_header_parser() {
        assert_eq!(answers_header("## Answers\n"), Ok(("\n", "## Answers")))
    }

    #[test]
    fn test_answer_checkbox() {
        assert_eq!(answer_checkbox("- [ ]"), Ok(("", "- [ ]")));
        assert_eq!(answer_checkbox("- [x]"), Ok(("", "- [x]")));
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
                    Answer::new("Use and configure the teaser core component.", false),
                    Answer::new("Create a new custom component from scratch.", false),
                    Answer::new("Overlay the teaser core component.", false),
                    Answer::new("Inherit from the teaser core component.", true)
                ]
            ))
        );
    }

    #[test]
    fn test_reading_header_parser() {
        assert_eq!(
            opt_reading_header("## Reading\n"),
            Ok(("\n", Some("## Reading")))
        );
        assert_eq!(opt_reading_header(""), Ok(("", None)))
    }

    #[test]
    fn test_reading_parser() {
        assert_eq!(
            opt_reading("- [here](reading/question-3-reading.md)\n"),
            Ok(("\n", Some("reading/question-3-reading.md".into())))
        );

        assert_eq!(opt_reading(""), Ok(("", None)))
    }

    #[test]
    fn test_horizontal_rule_parser() {
        assert_eq!(horizontal_rule("---\n"), Ok(("\n", "---")));
    }
}
