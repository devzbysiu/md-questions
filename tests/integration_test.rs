use md_questions::{Answer, Question, Questions};
use std::fs::read_to_string;

#[test]
fn test_reading_questions() {
    let content = read_to_string("res/QUESTIONS.md").unwrap();
    let questions = Questions::from(content.as_str());
    let first_question = &questions[0];

    assert_eq!(first_question,
        &Question::default()
            .with_number(1)
            .with_text("A developer needs to create a banner component. This component shows an image across the full width of the page. A title is shown on top of the image. This text can be aligned to the left, middle, or right. The core components feature a teaser component which matches almost all requirements, but not all. What is the most maintainable way for the developer to implement these requirements?")
            .with_answer(Answer::new("Use and configure the teaser core component.", false))
            .with_answer(Answer::new("Create a new custom component from scratch.", false))
            .with_answer(Answer::new("Overlay the teaser core component.", false))
            .with_answer(Answer::new("Inherit from the teaser core component.", true))
            .with_category("Templates and Components")
    );
}
