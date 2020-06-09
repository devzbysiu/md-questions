use crate::Question;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::sequence::tuple;
use nom::IResult;
use std::num::ParseIntError;

#[allow(dead_code)] // TODO: remove
fn question(i: &str) -> IResult<&str, Question> {
    let (i, number) = question_number(i)?;
    let (i, _) = char('\n')(i)?;
    Ok((
        i,
        Question {
            number,
            text: "".into(),
            answers: vec![],
            reading: None,
            category: "".into(),
        },
    ))
}

fn question_number(i: &str) -> IResult<&str, u32> {
    let (i, (_, num)) = tuple((tag("# Question "), map_res(digit1, to_int)))(i)?;
    Ok((i, num))
}

fn to_int(i: &str) -> Result<u32, ParseIntError> {
    i.parse::<u32>()
}

#[cfg(test)]
mod test {
    use super::question_number;
    use crate::parsers::question;
    use crate::Question;

    #[test]
    fn test_question_parser() {
        let input = r#"# Question 1
"#;
        assert_eq!(
            question(input),
            Ok((
                "",
                Question {
                    number: 1,
                    text: "".into(),
                    answers: vec![],
                    reading: None,
                    category: "".into()
                }
            ))
        );
    }

    #[test]
    fn test_question_number_parser() {
        let input = "# Question 1";
        assert_eq!(question_number(input), Ok(("", 1)));
    }
}
