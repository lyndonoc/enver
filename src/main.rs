use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::process::{Command as StdCommand, Stdio};

use clap::{App, Arg, Command};
use regex::Regex;
use tabwriter::TabWriter;

fn main() {
    let matches = App::new("enver CLI")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("run")
                .about("run a command with given environment variables")
                .arg(Arg::with_name("ENV_FILE_PATH").required(true))
                .arg(Arg::with_name("COMMAND_TO_EXECUTE").required(true))
                .arg(Arg::with_name("COMMAND_ARGS").multiple_values(true)),
        )
        .subcommand(
            Command::new("list")
                .about("print the list of provided environment variables")
                .arg(Arg::with_name("ENV_FILE_PATH").required(true)),
        )
        .get_matches();
    match matches.subcommand() {
        Some(("run", sub_matches)) => {
            let raw_envs = parse_raw_env_vars(sub_matches.value_of("ENV_FILE_PATH"));
            let args = sub_matches
                .get_many::<String>("COMMAND_ARGS")
                .unwrap_or_default()
                .map(|v| v.as_str())
                .collect::<Vec<&str>>();
            let env_entries = raw_env_vars_to_map(raw_envs, string_to_env_entry);
            let cmd = sub_matches
                .value_of("COMMAND_TO_EXECUTE")
                .unwrap_or_default();
            _ = StdCommand::new(cmd)
                .envs(env_entries)
                .args(args)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .unwrap()
                .wait();
        }
        Some(("list", sub_matches)) => {
            let raw_envs = parse_raw_env_vars(sub_matches.value_of("ENV_FILE_PATH"));
            let env_vecs = raw_env_vars_to_tuples(raw_envs, string_to_env_entry);
            let env_lines = env_vecs.into_iter().fold(
                String::from("VARIABLE NAME\tVARIABLE VALUE"),
                |acc, (key, value)| format!("{}\n{}\t{}", acc, key, value),
            );
            let mut tw = TabWriter::new(vec![]);
            tw.write_all(env_lines.as_bytes()).unwrap();
            tw.flush().unwrap();
            println!("{}", String::from_utf8(tw.into_inner().unwrap()).unwrap());
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}

fn parse_raw_env_vars(arg: Option<&str>) -> Vec<String> {
    match arg {
        Some(file_path) => {
            let result = fs::read_to_string(file_path)
                .expect(&format!("failed to read the env file: {}", file_path));
            result
                .split("\n")
                .map(|line| line.to_string())
                .collect::<Vec<String>>()
        }
        None => Vec::new(),
    }
}

fn raw_env_vars_to_map(
    raw_env_vars: Vec<String>,
    folder: fn(entry: &str) -> Option<(String, String)>,
) -> HashMap<String, String> {
    raw_env_vars
        .into_iter()
        .fold(HashMap::new(), |mut acc, entry| match folder(&entry) {
            Some((key, value)) => {
                acc.insert(key, value);
                acc
            }
            None => acc,
        })
}

fn raw_env_vars_to_tuples(
    raw_env_vars: Vec<String>,
    folder: fn(entry: &str) -> Option<(String, String)>,
) -> Vec<(String, String)> {
    raw_env_vars
        .into_iter()
        .fold(Vec::new(), |mut acc, entry| match folder(&entry) {
            Some((key, value)) => {
                acc.push((key, value));
                acc
            }
            None => acc,
        })
}

fn string_to_env_entry(entry: &str) -> Option<(String, String)> {
    let rgx = Regex::new(r"[a-zA-Z_]+[a-zA-Z0-9_]*=[a-zA-Z0-9_-]+").unwrap();
    if rgx.is_match(entry) {
        let entry_vec = entry
            .split("=")
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
        if entry_vec.len() != 2 {
            return None;
        }
        let key = entry_vec.get(0).map_or("", |k| k);
        let value = entry_vec.get(1).map_or("", |k| k);
        if key == "" || value == "" {
            return None;
        }
        return Some((key.to_string(), value.to_string()));
    }
    return None;
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::fs::{remove_file, File};

    #[test]
    fn test_parse_raw_env_vars() {
        let mut env_file = File::create(".env").unwrap();
        env_file.write_all(b"DATABASE_URL=postgres").unwrap();
        let raw_envs = parse_raw_env_vars(Some(".env"));
        remove_file(".env").unwrap();
        let folded = raw_envs
            .into_iter()
            .fold(String::from(""), |acc, curr| format!("{}{}", acc, curr));
        assert_eq!(folded, String::from("DATABASE_URL=postgres"));
    }

    #[test]
    fn test_raw_env_vars_to_map() {
        let input_vars = vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
            String::from("d"),
        ];
        fn folder(entry: &str) -> Option<(String, String)> {
            if entry == "c" {
                return Some((entry.to_string(), entry.to_string()));
            }
            return None;
        }
        let map = raw_env_vars_to_map(input_vars, folder);
        let comp = map
            .into_iter()
            .fold(String::from(""), |acc, (a, b)| format!("{}{}{}", acc, a, b));
        assert_eq!(comp, String::from("cc"));
    }

    #[test]
    fn test_raw_env_vars_to_tuples() {
        let input_vars = vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
            String::from("d"),
        ];
        fn folder(entry: &str) -> Option<(String, String)> {
            if entry == "a" || entry == "c" {
                return Some((entry.to_string(), entry.to_string()));
            }
            return None;
        }
        let tuples = raw_env_vars_to_tuples(input_vars, folder);
        let comp = tuples
            .into_iter()
            .fold(String::from(""), |acc, (a, b)| format!("{}{}{}", acc, a, b));
        assert_eq!(comp, String::from("aacc"));
    }

    #[test]
    fn test_valid_entry_parser() {
        let key = "DATABASE_URL";
        let val = "postgresql://asdf@asdf/asdf";
        let valid_env_entry = format!("{}={}", key, val);
        let parsed = string_to_env_entry(&valid_env_entry);
        assert_eq!(parsed, Some((key.to_string(), val.to_string())),)
    }

    #[test]
    fn test_invalid_entry_parser() {
        let invalid_env_entries = vec![
            "DA=TABASE_URL=postgresql://asdf@asdf/asdf",
            "=value",
            "key=",
            "===",
            "=",
        ];
        let filtered = invalid_env_entries
            .into_iter()
            .filter(|entry| string_to_env_entry(entry).is_some())
            .collect::<Vec<&str>>();
        assert_eq!(filtered.len(), 0);
    }
}
