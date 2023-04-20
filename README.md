# `enver`: A Rust-based Command Line Interface to Run Commands with Specific Environment Variables

`enver` is a command line interface tool written in Rust that enables the execution of arbitrary commands with a specified set of environment variables. This document provides an overview of how to use `enver`, its command line interface, and some examples of how to use it.

## Usage

### Command line interface

enver's command line interface provides the following options:

```
enver CLI 

USAGE:
    enver [SUBCOMMAND]

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    help    Print this message or the help of the given subcommand(s)
    list    Print the list of provided environment variables
    run     Run a command with given environment variables
```

### Examples

#### `enver run`

```
USAGE:
    enver run <ENV_FILE_PATH> <COMMAND_TO_EXECUTE> [COMMAND_ARGS]...
```

The `enver run` command is used to execute an arbitrary command (`<COMMAND_TO_EXECUTE> [COMMAND_ARGS]...`) while reading environment variables from a file (`<ENV_FILE_PATH>`).

For instance, you can create an `.env.development` file containing environment variables, such as:

```
PORT=4000
DATABASE_URL=postgresql://localhost/mydb?user=other&password=secret
```

And you want your app/program to be able to read these environment variables.

```javascript
console.log(process.env.PORT);
console.log(process.env.DATABASE_URL);
```

To read these environment variables, you can use enver CLI to run this JavaScript application while reading environment variables from the `.env.development` file:

```
enver run .env.development node index.js
```

#### `enver list`

```
USAGE:
    enver list <ENV_FILE_PATH>
```

The `enver list` command is used to print out all the environment variables in `<ENV_FILE_PATH>` that will be picked up by the CLI.

### Environment variable formats

- `enver` uses a regular expression `[a-zA-Z_]+[a-zA-Z0-9_]*=[a-zA-Z0-9_-]+` to validate whether a line in an environment file is valid.
- If any invalid lines are found in the provided file, they will be ignored and not be passed to the executed command.
- You can check which environment variables are considered valid and will be picked up by the CLI by running `enver list <ENV_FILE_PATH>`.
