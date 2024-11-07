use std::{fs, io};
use std::path::{self, PathBuf};
use std::ffi::OsStr;

pub struct Config {
    pub path: String,
    pub name: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>,) -> Result<Config, &'static str> {
        args.next();

        let path = match args.next() {
            Some(arg) => arg,
            None => ".".to_string()
        };

        let name = match args.next() {
            Some(arg) => if arg == "-name" {
                match args.next() {
                    Some(arg) => arg,
                    None => return Err("No name given for arg -name")
                }
            } else {
                "".to_string()
            }
            None => "".to_string()
        };

        Ok(Config { path, name })
    }
}

fn list_dir(dir: path::PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut entries: Vec<_> = fs::read_dir(dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    Ok(entries)
}

fn print_dir_files(dir: path::PathBuf, name: &str) {
    let entries = list_dir(dir).unwrap();

    for entry in entries {
        // Print entry
        if name != "" {
            let file_name = file_name_to_string(entry.file_name());
            if file_name == name{
                println!("{}", entry.display());
            }
        } else {
            println!("{}", entry.display());
        }
        // if entry is dir then print entries in that dir
        // do this recursivly so can always go in to the dir in dir
        // and print that instead of just printing without checking new dirs
        if entry.is_dir() {
            print_dir_files(entry, name)
        }
    }
}

fn file_name_to_string(file_name: Option<&OsStr>) -> String {
    let name = match file_name {
        Some(i) => i,
        None => return "Error parsing the searched file name".to_string()
    };

    name.to_str().unwrap().to_string()
}

pub fn run(config: Config) -> io::Result<()> {
    print_dir_files(config.path.into(), &config.name);

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn no_name() {
//         let dir = "./src".into();
//
//         assert_eq!(vec!["src/lib.rs", "src/main.rs"], list_dir(dir).unwrap())
//     }
// }
