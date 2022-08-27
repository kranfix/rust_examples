# OpenAI CLI

This project is inspired in this
[video](https://www.youtube.com/watch?v=5WhJQMnJjik)
from `Code to the Moon` YouTube channel, but refactored to handle
abstractions for both addings tests and generalizing the code to work with
others AI providers in the future.

[![Inspiration vide](https://img.youtube.com/vi/5WhJQMnJjik/0.jpg)](https://www.youtube.com/watch?v=5WhJQMnJjik)

The motivation to refactor this was that there are many countries that are
not allowed to use OpenAI, but the example must be able to be excecuted anybody.

## Running

To run the faked example, run:

```
cargo run --example faked_answers
```

Write anything to get a faked asnwer and enter `fail` to generate an error.

To run the OpenAI implementation, run:

```
OAI_TOKEN=writeYourToken
cargo run --example open_ai_cli
```
