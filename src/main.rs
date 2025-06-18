use chrono::{format, DateTime, Local, Utc};
use clap::{builder::styling::Color, Parser};
use owo_colors::OwoColorize;
use serde::Serialize;
use std::{
    collections::btree_map::Entry,
    fs,
    path::{Path, PathBuf},
};
use strum::Display;
use tabled::settings::Color as TabledColor;
use tabled::{
    settings::{
        object::{Columns, Rows},
        Style,
    },
    Table, Tabled,
};

#[derive(Debug, Display, Serialize)]
enum EntryType {
    File,
    Dir,
}

#[derive(Debug, Tabled, Serialize)]
struct FileEntry {
    #[tabled{rename="Name"}]
    name: String,
    #[tabled{rename="Type"}]
    e_type: EntryType,
    #[tabled{rename="Size"}]
    len_bytes: u64,
    #[tabled{rename="Modified"}]
    modified: String,
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = "The better ls command")]
struct Cli {
    path: Option<PathBuf>,
    #[arg(short, long)]
    json: bool,
}

fn get_meta(file: fs::DirEntry, data: &mut Vec<FileEntry>) {
    if let Ok(meta) = fs::metadata(&file.path()) {
        data.push(FileEntry {
            name: file
                .file_name()
                .into_string()
                .unwrap_or("unknown name".into()),
            e_type: if meta.is_dir() {
                EntryType::Dir
            } else {
                EntryType::File
            },
            len_bytes: meta.len(),
            modified: if let Ok(modi) = meta.modified() {
                let date: DateTime<Utc> = modi.into();
                format!("{}", date.format("%a %b %e %Y"))
            } else {
                String::default()
            },
        });
    }
}

fn get_files(path: &Path) -> Vec<FileEntry> {
    let mut data = Vec::default();
    if let Ok(read_dir) = fs::read_dir(path) {
        for entry in read_dir {
            if let Ok(file) = entry {
                get_meta(file, &mut data);
            }
        }
    }
    data
}

fn display_table(path: PathBuf) {
    let entries = get_files(&path);
    let mut table = Table::new(entries);
    table.with(Style::rounded());
    table.modify(Columns::first(), TabledColor::FG_BRIGHT_CYAN);
    table.modify(Columns::one(2), TabledColor::FG_BRIGHT_MAGENTA);
    table.modify(Columns::one(3), TabledColor::FG_BRIGHT_YELLOW);
    table.modify(Rows::first(), TabledColor::FG_BRIGHT_GREEN);
    println!("{}", table);
}

fn main() {
    let cli = Cli::parse();
    let path = cli.path.unwrap_or(PathBuf::from("."));

    if let Ok(does_exist) = fs::exists(&path) {
        if does_exist {
            if cli.json {
                let entries = get_files(&path);
                println!(
                    "{}",
                    serde_json::to_string(&entries).unwrap_or("Cannot parse JSON".to_string())
                );
            } else {
                display_table(path);
            }
        } else {
            println!("{}", "Path does not exist.".red());
        }
    } else {
        println!("{}", "Error reading directory.".red())
    }
}
