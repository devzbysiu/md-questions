use indoc::indoc;
use md_questions::{ClosedAnswer, MdQuestions, OpenAnswer};
use std::fs::read_to_string;

#[test]
fn test_reading_closed_questions_from_file() {
    let content = read_to_string("res/closed-questions.md").unwrap();
    let questions = MdQuestions::from(content.as_str());
    let first_question = &questions[0];

    let Some(closed_question) = first_question.as_closed() else {
        panic!("Should not happen");
    };
    assert_eq!(*closed_question.number(), 1);
    assert_eq!(closed_question.category(), "Templates and Components");
    assert_eq!(
        closed_question.text(),
        indoc! {
            "A developer needs to create a banner component. This component shows an image
            across the full width of the page. A title is shown on top of the image. This
            text can be aligned to the left, middle, or right. The core components feature a
            teaser component which matches almost all requirements, but not all. What is the
            most maintainable way for the developer to implement these requirements?"
        }
    );
    assert_eq!(
        closed_question.answers(),
        &[
            ClosedAnswer::incorrect("Use and configure the teaser core component."),
            ClosedAnswer::incorrect("Create a new custom component from scratch."),
            ClosedAnswer::incorrect("Overlay the teaser core component."),
            ClosedAnswer::correct("Inherit from the teaser core component.")
        ]
    );
    assert!(closed_question.reading().is_none());
}

#[test]
fn test_reading_open_questions_from_file() {
    let content = read_to_string("res/open-questions.md").unwrap();
    let questions = MdQuestions::from(content.as_str());
    let first_question = &questions[0];

    let Some(open_question) = first_question.as_open() else {
        panic!("Should not happen");
    };
    assert_eq!(*open_question.number(), 1);
    assert_eq!(open_question.category(), "Introduction");
    assert_eq!(open_question.text(), "Enumerate common types of graphs");
    assert_eq!(
        open_question.answer(),
        &OpenAnswer::new(indoc! {
            "1. Undirected Graph
             2. Directed Graph (Digraph)
             3. Weighted Graph
             4. Tree
             5. Rooted Tree
             6. Directed Acyclic Graph (DAG)
             7. Bipartite Graph
             8. Complete Graph"
        })
    );
    assert!(open_question.reading().is_none());
}
