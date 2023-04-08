<div align="center">

  <h1><code>md-questions</code></h1>

  <h3>
    <strong>Parses markdown to get questions based on a convention</strong>
  </h3>

  <p>
    <img src="https://img.shields.io/github/actions/workflow/status/devzbysiu/md-questions/ci.yml?style=for-the-badge" alt="CI status badge" />
    <a href="https://codecov.io/gh/devzbysiu/md-questions">
      <img src="https://img.shields.io/codecov/c/github/devzbysiu/md-questions?style=for-the-badge" alt="Code coverage"/>
    </a>
    <img src="https://img.shields.io/badge/license-MIT%2FAPACHE--2.0-blue?style=for-the-badge" alt="License"/>
  </p>

  <h3>
    <a href="#about">About</a>
    <span> | </span>
    <a href="#installation">Installation</a>
    <span> | </span>
    <a href="#license">License</a>
    <span> | </span>
    <a href="#contribution">Contribution</a>
  </h3>

  <sub><h4>Built with 🦀</h4></sub>
</div>

# <p id="about">About</p>

The idea is that you write questions in markdown file using specified convention, then you can use
this library to parse the markdown. Here are example questions with the description of the question
structure:

```markdown
## Question 1 `Programming Language` `Ignore`  // 1. Question Header: ## Question <number> `<category>` `<marker>`
What languages are used in this repository?    // 2. Question Content. Can be multi-line.
Pick two.                                      //
                                               // 3. New line.
## Answers                                     // 4. Answers Header: ## Answers
- [x] Rust                                     // 5. Answers.
- [ ] Java                                     //    Only single-line answers.
- [ ] Kotlin                                   //    Correct answer: - [x] <text>
- [ ] Go                                       //    Incorrect answer: - [ ] <text>
- [x] Bash                                     //    Multiple answers supported.
                                               // 6. New line.
## [Reading](reading/question-1.md)            // 7. Optional Reading Header: ## [Reading](<url>)
                                               // 8. New line.
---                                            // 9. Questions Separator.
                                               // 10. New line.
## Question 2 `Clean Code`
What letter S in SOLID acronym stands for?

## Answers                                     // As many answers as you want
- [ ] Substitution Principle
- [ ] Sub Dependency Principle
- [x] Single Responsibility Principle
- [ ] System Inversion Principle
- [ ] Super Closed Principle

---

## Question 3 `Microservice Architecture`      // Open questions supported
Describe Event Sourcing.

## Answer
Event Sourcing is a technique where instead
of storing domain data as a whole, we keep
track of all the operations made on that data
(and we can restore the current state based
on that log of operations if needed).

```

Reading section is optional. Marker field is optional.
All the rest is required to correctly parse the markdown.

Then you can read the questions as following:
```rust
let content = read_to_string("./QUESTIONS.md")?;
let questions = Questions::from(content.as_str());

println!("First question: {}", &questions[0].text());
```

You can use marker to mark a question. Currently, only `Ignore` marker is supported (casing is not
important).
It allows skipping the parsing of the question.
This way we can keep more advanced questions in the markdown but still use the library.

```markdown
## Question 1 `Some category` `Ignore`
This multi-line aswers question is not yet supported.

## Answers
- [x] Multi-line
      answer 1
- [ ] Multi-line
      answer 2
- [ ] Multi-line
      answer 3
- [ ] Multi-line
      answer 4

## [Reading](reading/question-1.md)

---

```

# <p id="installation">Installation</p>

Add
```toml
md_questions = { git = "https://github.com/devzbysiu/md-questions", rev = "5fb14b63b30324e026148592354713620aef7983" }
```
to your `Cargo.toml`

# <p id="license">License</p>

This project is licensed under either of

- Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

# <p id="contribution">Contribution</p>


Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
