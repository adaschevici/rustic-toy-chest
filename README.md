## This is a silly collection of rust projects that I am hacking on sometimes

## Projects

- [x] Todo list using perseus and axum
- [] Todo list using leptos and tailwind and axum
- [] Next up: a quasi prod rest api with axum and diesel
- [] ...

## How to run:

```bash
# Clone the repo
git clone git@github.com:adaschevici/rustic-toy-chest.git
cd rustic-toy-chest
bash bootstrap.sh # this installs any binaries that are needed with cargo
# list the projects available for running
cargo make --list-category-steps "workspace"
# choose one and run it
# for example: cargo make run-perseus-todo
cargo make <project-name>
```
