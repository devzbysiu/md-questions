use anyhow::Result;
use md_questions::{ClosedAnswer, MdQuestions, Question};
use std::fs::read_to_string;

#[test]
fn test_reading_closed_questions_from_file() -> Result<()> {
    let content = read_to_string("res/closed-questions.md").unwrap();
    let questions = MdQuestions::from(content.as_str());
    let first_question = &questions[0];

    assert_eq!(first_question, &Question::closed()
        .number(1)
        .text("A developer needs to create a banner component. This component shows an image across the full \
              width of the page. A title is shown on top of the image. This text can be aligned to the left, middle, \
              or right. The core components feature a teaser component which matches almost all requirements, \
              but not all. What is the most maintainable way for the developer to implement these requirements?")
        .answers(vec![
            ClosedAnswer::incorrect("Use and configure the teaser core component."),
            ClosedAnswer::incorrect("Create a new custom component from scratch."),
            ClosedAnswer::incorrect("Overlay the teaser core component."),
            ClosedAnswer::correct("Inherit from the teaser core component.")
        ])
        .category("Templates and Components").build()?.into()
    );

    assert_eq!(questions.count(), 58);

    Ok(())
}
