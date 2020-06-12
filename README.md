<div align="center">

  <h1><code>md-questions</code></h1>

  <p>
    <strong>Parses markdown to get questions based on a convention</strong>
  </p>

  <p>
    <img src="https://github.com/devzbysiu/md-questions/workflows/Main/badge.svg" alt="CI status
    badge" />
    <a href="https://codecov.io/gh/devzbysiu/md-questions">
      <img src="https://img.shields.io/codecov/c/github/devzbysiu/md-questions?color=%2388C0D0&logoColor=%234C566A&style=flat-square&token=bfdc4b9d55534910ae48fba0b8e984d0" alt="Code coverage"/>
    </a>
    <a href="https://crates.io/crates/md-questions">
      <img src="https://img.shields.io/crates/l/md-questions?color=%2388C0D0&logoColor=%234C566A&style=flat-square" alt="License"/>
    </a>
    <a href="https://docs.rs/md-questions">
      <img src="https://img.shields.io/badge/docs-latest-blue.svg?color=%2388C0D0&logoColor=%234C566A&style=flat-square" alt="docs.rs docs" />
    </a>
  </p>

  <h4>
    <a href="#about">About</a>
    <span> | </span>
    <a href="#installation">Installation</a>
    <span> | </span>
    <a href="#license">License</a>
    <span> | </span>
    <a href="#contribution">Contribution</a>
  </h3>

  <sub>Built with 🦀</sub>
</div>

# <p id="about">About</p>

The idea is simple: you write questions in markdown file using some convention, then you can use this library to
parse this markdown. Here are example questions with question structure description:

```markdown
## Question 1 `Programming Language`             // 1. Question Header: ## Question <number> `<category>`
What language was used to write this library?    // 2. Question Content. Can be multi-line.
Pick one.                                        //
                                                 // 3. New line.
## Answers                                       // 4. Answers Header: ## Answers
- [x] Rust                                       // 5. Answers.
- [ ] Java                                       //    Only single-line answers.
- [ ] Kotlin                                     //    Correct answer: - [x] <text>
- [ ] Go                                         //    Incorrect answer: - [ ] <text>
                                                 // 6. New line.
## [Reading](reading/question-1.md)              // 7. Optional Reading Header: ## [Reading](<url>)
                                                 // 8. New line.
---                                              // 9. Questions Separator.
                                                 // 10. New line.
## Question 2 `Clean Code`
What letter S in SOLID acronym stands for?

## Answers
- [ ] Substitution Principle
- [ ] Sub Dependency Principle
- [x] Single Responsibility Principle
- [ ] System Inversion Principle
- [ ] Super Closed Principle


```

Reading section is optional. All the rest is required to correctly parse the markdown.

Then you can read the questions as following:
```rust
let content = read_to_string("./QUESTIONS.md")?;
let questions = Questions::from(content.as_str());

println!("First question: {}", &questions[0].text());
```

# <p id="installation">Installation</p>

Add
```toml
md_questions = "0.1.0"
```
to your `Cargo.toml`

# <p id="license">License</p>

This project is licensed under either of

- Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

# <p id="contribution">Contribution</p>


Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
