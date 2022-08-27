# OpenAI CLI

This project is inspired in this
[video](https://www.youtube.com/watch?v=5WhJQMnJjik&t=565s)
from `Code to the Moon` YouTube channel, but refactored to handle
abstractions for both addings tests and generalizing the code to work with
others AI providers in the future.

<iframe width="560" height="315" src="https://www.youtube.com/embed/5WhJQMnJjik" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>

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
