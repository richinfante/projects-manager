//#[macro_use]
// extern crate serde_derive;
// extern crate toml;
extern crate git2;
extern crate colored;

use colored::*;
use std::fs;
use git2::*;
use git2::Status;
use std::env;

// #[derive(Deserialize)]
// struct Config {
//     user_path: String,
//     install_path: String
// }


// #[derive(Serialize)]
// struct Types {
//     name: String,
//     commands: Option<String>,
// }


fn list_cmd() {
    let mut paths : Vec<Box<std::fs::DirEntry>> = fs::read_dir("./").unwrap().flat_map(|r| {
        match r {
            Ok(r) => Some(Box::new(r)),
            _ => None
        }
    }).collect();

    paths.sort_by_key(|dir| dir.path());

    for path in paths {
        let path = path.path();
        let display = path.strip_prefix("./").unwrap().display();
        
        // Get the repo (if it exists.)
        let repo = match Repository::open(path.clone()) {
            Ok(repo) => repo,
            Err(_) => {
                println!("{:>10} {:<32}", "folder".bright_black(), display);
                continue;
            }
        };

        // Get the statuses list
        let statuses = match repo.statuses(None) {
            Ok(statuses) => statuses,
            Err(e) => panic!(e)
        };

        //println!("    -> Found {} changes", statuses.len());
        let mut modified_count = 0;
        let mut ignored_count = 0;
        let mut deleted_count = 0;
        let mut new_count = 0;
        let mut renamed_count = 0;
        let mut type_change_count = 0;
        let mut conflict_count = 0;
        for status in statuses.iter() {
            let status = status.status();

            // Skip ignored items.
            if status.intersects(Status::IGNORED) {
                ignored_count += 1;
                continue;
            }

            if status.intersects(Status::WT_MODIFIED) {
                modified_count += 1;
                continue;
            }

            if status.intersects(Status::WT_NEW) {
                new_count += 1;
                continue;
            }

            if status.intersects(Status::WT_DELETED) {
                deleted_count += 1;
                continue;
            }

            if status.intersects(Status::WT_RENAMED) {
                renamed_count += 1;
                continue;
            }

            if status.intersects(Status::WT_TYPECHANGE) {
                type_change_count += 1;
                continue;
            }

            if status.intersects(Status::CONFLICTED) {
                conflict_count += 1;
                continue;
            }
            

            // let path = match status.path() {
            //     Some(value) => value,
            //     None => continue
            // };

        }

        if ignored_count == 0 {
            println!("{:>10} {:<32}", "git".bright_green(), display);
        } else {
            let mut changes = vec![
                // ("ignored", ignored_count),
                ("renamed", renamed_count),
                ("created", new_count),
                ("deleted", deleted_count),
                ("modified", modified_count),
                ("type change", type_change_count),
                ("conflicted", conflict_count)
            ];

            let change_desc = changes.iter().flat_map(|v| {
                if v.1 == 0 {
                    return None
                }

                return Some(format!("{} {}", v.1, v.0))
            }).collect::<Vec<String>>().join(", ");

            println!("{:>10} {:<32} {}", "git".bright_green(), display, change_desc.bright_black());
        }
    }
}

fn run_cmd(name: String) {
    match name.as_ref() {
        "list" => list_cmd(),
        _ => panic!("Command {} not found", name),
    };
}

fn main() {

    let cmd = match env::args().nth(1) {
        Some(value) => value,
        None => panic!("No Command Provided.")
    };

    run_cmd(cmd);
}

