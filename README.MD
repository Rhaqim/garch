# Garch

Garch is a command-line interface (CLI) application built with Rust. It is used to generate Go boilerplate code for different architectures.

## Table of Contents

- [Garch](#garch)
  - [Table of Contents](#table-of-contents)
  - [Installation](#installation)
  - [Usage](#usage)
  - [Architectures](#architectures)
    - [Hexagonal Architecture](#hexagonal-architecture)
  - [License](#license)
  - [Acknowledgements](#acknowledgements)
  - [Contributing](#contributing)

## Installation

To install Garch, you need to have Rust and Cargo installed on your system. Once you have them set up, you can install Garch by running the following command:

- via Cargo:

```bash
cargo install garch
```

- via Git:

```bash
git clone https://github.com/garch.git
cd garch
cargo install --path .
```

## Usage

To use Garch, you need to run the following command:

```bash
garch --help
```

This will display the help message, which will show you how to use Garch.

## Architectures

Garch supports the following architectures:

- [Hexagonal Architecture](#hexagonal-architecture)
- Clean Architecture (coming soon)
- Onion Architecture (coming soon)

### [Hexagonal Architecture](https://en.wikipedia.org/wiki/Hexagonal_architecture_(software))

The Hexagonal Architecture is a software design philosophy that separates the software into layers. It was introduced by Alistair Cockburn in 2005. The Hexagonal Architecture is composed of the following layers:

```bash
.
├── LICENSE
├── README.md
├── cmd
│   └── main.go
├── go.mod
├── internal
│   ├── adapters
│   │   ├── cache
│   │   │   └── cache.go
│   │   ├── handler
│   │   │   └── handler.go
│   │   ├── repository
│   │   │   └── repository.go
│   │   └── token
│   │       └── token.go
│   └── core
│       ├── domain
│       │   ├── entity.go
│       │   └── value_object.go
│       ├── port
│       │   ├── input_port.go
│       │   └── output_port.go
│       ├── service
│       │   └── service.go
│       └── util
│           ├── error.go
│           └── logger.go
├── pkg
└── testdata
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [Rust Language](https://www.rust-lang.org/)
- [Rustup](https://rustup.rs/)
- [Cargo](https://doc.rust-lang.org/cargo/)
- [Clap](https://clap.rs/)
- [Rust Community](https://www.rust-lang.org/community)
<!-- - [Askama](
- [Handlebars](
- [Rustfmt](
- [Rustdoc]( -->

## Contributing

If you would like to contribute to this project, please read the [CONTRIBUTING.md](CONTRIBUTING.md) file for details on our code of conduct, and the process for submitting pull requests to us.
