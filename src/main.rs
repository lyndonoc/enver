use std::collections::HashMap;
use std::fs;
use std::process::{Command, Stdio};

use clap::{App, Arg};
use regex::Regex;

fn main() {
    let matches = App::new("enver CLI")
        .arg_required_else_help(true)
        .arg(Arg::with_name("ENV_FILE_PATH").required(true))
        .arg(Arg::with_name("COMMAND_TO_EXECUTE").required(true))
        .arg(Arg::with_name("COMMAND_ARGS").multiple_values(true))
        .get_matches();
    let raw_envs = match matches.value_of("ENV_FILE_PATH") {
        Some(file_path) => {
            let result = fs::read_to_string(file_path)
                .expect(&format!("failed to read the env file: {}", file_path));
            result
                .split("\n")
                .map(|line| line.to_string())
                .collect::<Vec<String>>()
        }
        None => Vec::new(),
    };
    let args = matches
        .get_many::<String>("COMMAND_ARGS")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();
    let env_entries = raw_envs.into_iter().fold(HashMap::new(), |mut acc, entry| {
        match string_to_env_entry(&entry) {
            Some((key, value)) => {
                acc.insert(key, value);
                acc
            }
            None => acc,
        }
    });
    let cmd = matches.value_of("COMMAND_TO_EXECUTE").unwrap_or_default();
    _ = Command::new(cmd)
        .envs(env_entries)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap()
        .wait();
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
