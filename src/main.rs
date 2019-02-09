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

/// List command
fn list_cmd() {
    // Turn the paths into a vector of dir entries
    let mut paths : Vec<Box<std::fs::DirEntry>> = fs::read_dir("./").unwrap().flat_map(|r| {
        match r {
            Ok(r) => Some(Box::new(r)),
            _ => None
        }
    }).collect();

    // Sort in alphabetical order by path.
    paths.sort_by_key(|dir| dir.path());

    for path in paths {
        let path = path.path();
        let display = path.strip_prefix("./").unwrap().display();
        
        // Get the repo (if it exists.)
        let repo = match Repository::open(path.clone()) {
            Ok(repo) => repo,
            Err(_) => {
                if path.is_dir() {
                    println!("{:>10} {:<32}", "folder".bright_black(), display);
                } else {
                    println!("{:>10} {:<32}", "file".blue(), display);
                }
                
                continue;
            }
        };

        // Get the statuses list
        let statuses = match repo.statuses(None) {
            Ok(statuses) => statuses,
            Err(e) => panic!(e)
        };

        // Store counters
        let mut modified_count = 0;
        let mut ignored_count = 0;
        let mut deleted_count = 0;
        let mut new_count = 0;
        let mut renamed_count = 0;
        let mut type_change_count = 0;
        let mut conflict_count = 0;

        // For each repo status, count them.
        for status in statuses.iter() {
            let status = status.status();

            // Count ignored statuses
            if status.intersects(Status::IGNORED) {
                ignored_count += 1;
                continue;
            }

            // Count modified statuses
            if status.intersects(Status::WT_MODIFIED) {
                modified_count += 1;
                continue;
            }

            // Count new files
            if status.intersects(Status::WT_NEW) {
                new_count += 1;
                continue;
            }

            // Count deleted files
            if status.intersects(Status::WT_DELETED) {
                deleted_count += 1;
                continue;
            }

            // Count renamed files
            if status.intersects(Status::WT_RENAMED) {
                renamed_count += 1;
                continue;
            }

            // Count typechange statuses
            if status.intersects(Status::WT_TYPECHANGE) {
                type_change_count += 1;
                continue;
            }

            if status.intersects(Status::CONFLICTED) {
                conflict_count += 1;
                continue;
            }
        }

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

/// Fallback command.
fn unknown_cmd(subcommand: &str) {
    println!("error: unknown subcommand: {}. valid options: list", subcommand)
}

/// Parse subcommands
fn run_cmd(name: String) {
    match name.as_ref() {
        "list" => list_cmd(),
        _ => unknown_cmd(&name)
    };
}

/// Main function
fn main() {
    match env::args().nth(1) {
        Some(value) => run_cmd(value),
        None => println!("no subcommand provided. valid options: list")
    }
}

