# rember-cli

A clone of Ember-CLI written in Rust.

## What this is

I am currently learning Rust and I'm honestly not good at it.

So, as a learning project I'm creating a clone of Ember CLI in Rust.

Expect the code to be awful - I'm learning as I go...

## What this is not

* Stable - it probably won't work on your machine.
* An official project
* A replacement of Ember CLI

## How to install

`cargo install rember`

Goal for version 1 (classic app structure):

| Feature        | Status                                                                            |
|----------------|-----------------------------------------------------------------------------------|
| ember new      | ⚠ `rember new project-name` works in most cases, but is missing support for flags. |
| ember generate component `name` | ✅                                                                                 |
| ember generate component-class `name` | ✅                                                                                 |
| ember serve    | ❌                                                                                 |


Read more in [concerns.md](concerns.md)

## Want to help?

* Give the supported features a try and report bugs
* Learning Rust as well? Reach out to me and we can work on this together
* Are you a Rust expert? Please review my code and provide feedback.
