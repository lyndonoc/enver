# ENVER

A super simple command line interface, written in Rust, to run any arbitrary command with a given set of environment variables.

## Usage

### Command line interface

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

This is used to run an arbitrary command (`<COMMAND_TO_EXECUTE> [COMMAND_ARGS]...`) while reading environment variables from a file (`<ENV_FILE_PATH>`).

For example, you can have an `.env.development` file that looks like the following:

```
PORT=4000
DATABASE_URL=postgresql://localhost/mydb?user=other&password=secret
```

And you want your app/program to be able to read these environment variables.

```javascript
console.log(process.env.PORT);
console.log(process.env.DATABASE_URL);
```

You can use `enver` CLI to run this JavaScript application while reading environment variables from `.env` file:

```
enver run .env.development node index.js
```

#### `enver list`

```
USAGE:
    enver list <ENV_FILE_PATH>
```

This is used to print out all the environment variables in `<ENV_FILE_PATH>` that will be picked up by the CLI.

### Environment variable formats

- The ENVER CLI uses a regular expression `[a-zA-Z_]+[a-zA-Z0-9_]*=[a-zA-Z0-9_-]+` to determine whether a line in a given environment file is valid.
- If any invalid line is found in the provided file, they will simply be ignored and not be passed to the command that is being executed.
- You can inspect which environment variables are considered valid and will be picked up by the CLI by running `enver list <ENV_FILE_PATH>`.
