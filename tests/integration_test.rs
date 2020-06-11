use md_questions::{Answer, Question, Questions};
use std::fs::read_to_string;

#[test]
fn test_reading_questions() {
    let content =
        read_to_string("/home/zbychu/learning/aem-sites-exam/ace-aem-sites-developer/QUESTIONS.md")
            .unwrap();
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

    let last_question = &questions[questions.len() - 1];
    assert_eq!(last_question,
        &Question::default()
            .with_number(17)
            .with_text("A developer is creating templates and/or components using CRXDE Lite. The developer needs to check the files into source control. Which tool should the developer use to achieve this goal?")
            .with_answer(Answer::new("vlt command", true))
            .with_answer(Answer::new("Content Explorer", false))
            .with_answer(Answer::new("http://localhost:4502/crx/checkout", false))
            .with_answer(Answer::new("mvn command", false))
            .with_category("Templates and Components")
            .with_reading("https://docs.adobe.com/content/help/en/experience-manager-65/developing/devtools/ht-vlttool.html")
    );
}
