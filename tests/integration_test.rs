use anyhow::Result;
use md_questions::{ClosedAnswer, MdQuestions, Question};
use std::fs::read_to_string;

#[test]
fn test_reading_checkbox_questions_from_file() -> Result<()> {
    let content = read_to_string("res/checkbox-questions.md").unwrap();
    let questions = MdQuestions::from(content.as_str());
    let first_question = &questions[0];

    assert_eq!(first_question,
        &Question::closed()
            .number(1)
            .text("A developer needs to create a banner component. This component shows an image across the full \
              width of the page. A title is shown on top of the image. This text can be aligned to the left, middle, \
              or right. The core components feature a teaser component which matches almost all requirements, \
              but not all. What is the most maintainable way for the developer to implement these requirements?")
            .answers(vec![
            ClosedAnswer::new("Use and configure the teaser core component.", false),
            ClosedAnswer::new("Create a new custom component from scratch.", false),
            ClosedAnswer::new("Overlay the teaser core component.", false),
            ClosedAnswer::new("Inherit from the teaser core component.", true)
        ])
            .category("Templates and Components").build()?.into()
    );

    let multiline_question = &questions[17];
    assert_eq!(multiline_question,  
        &Question::closed()
            .number(18)
            .text(r#"A developer is creating a new OSGi bundle `com.custom.package.b` to expose new services. `com.custom.package.a` is already installed and active in the system and has the following package definition:
  ```
  Export-Package: com.custom.package.a;version="2.0"
  Import-Package: com.sample.package.a;version="[1,2]"
  Classpath: .,com.sample.package.b-3.0.jar
  ```
  The system console shows the following package availability:
  ```
  com.sample.package.a;version="1.5"
  com.sample.package.c;version="3.0"
  ```
  Bundle com.custom.package.b to be installed has the following package definition:
  ```
  Export-Package: com.custom.package.b;version="1.0"
  Import-Package: com.custom.package.a;version="[1,2)",com.sample.package.b;version="[3.0,3.0]",com.sample.package.c;version="[2,3)"
  ```
  What will happen when the developer uploads the bundle com.custom.package.b into the system?"#)
              .answers(vec![
                ClosedAnswer::new("The bundle will install but fail the activation due to unsatisfied dependencies \
                    `com.sample.package.b` and `com.sample.package.c`.", true),
                ClosedAnswer::new("The bundle will install but fail the activation due to unsatisfied dependency \
                    `com.sample.package.c`.", false),
                ClosedAnswer::new("The bundle will install and activate successfully.", false),
                ClosedAnswer::new("The bundle will install but fail the activation due to unsatisfied dependency \
                    `com.sample.package.b`.", false)]
            )
            .category("OSGi Services").build()?.into()
      );

    assert_eq!(questions.count(), 58);

    Ok(())
}
