use anyhow::Result;
use filesize::PathExt;
use humansize::{file_size_opts as options, FileSize};
use prettytable::format;
use prettytable::{Cell, Row, Table};
use std::fs::{self};
use std::process;
use std::{os::linux::fs::MetadataExt, path::Path};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    path: String,
}

const SIZE_LESS: u64 = 1024 * 10;

fn main() -> Result<()> {
    let args = Cli::from_args();
    let path = Path::new(&args.path);
    if !path.exists() {
        //TODO libcのエラーメッセージを呼び出す方法を調べる
        eprintln!("No such file or directory.");
        process::exit(libc::ENOENT);
    }

    if path.is_file() {
        match read_file(path) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e)
            }
        }
    } else {
        match list_dir(path) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e)
            }
        }
    }
    /*
        match path {
            Path::is_file(&self) => {
                println!("{}", "hell")
            }
            Path::is_dir(&self) => {
                println!("{}", "aa")
            }
        }
    */
    //

    //

    //let content =
    //std::fs::read_to_string(&args.path).with_context(|| format!("could not read file"))?;

    //find_matches(&content, std::io::stdout())?;
    Ok(())
}

fn read_file(path: &Path) -> Result<()> {
    if SIZE_LESS < path.size_on_disk()? {
        //TODO impl less
        Ok(())
    } else {
        match std::fs::read_to_string(path) {
            Ok(v) => {
                println!("{}", v);
                Ok(())
            }
            Err(e) => Err(anyhow::Error::new(e)),
        }
    }
}

#[macro_use]
extern crate prettytable;
fn list_dir(path: &Path) -> Result<()> {
    let mut table = Table::new();
    table.set_titles(row!["permission", "name", "size"]);
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    for entry in fs::read_dir(path)? {
        let path = entry?.path();
        let meta = fs::metadata(&path).unwrap();
        let stat = meta.st_mode();
        table.add_row(Row::new(vec![
            Cell::new(&unix_mode::to_string(stat)),
            Cell::new(&path.display().to_string()),
            Cell::new(
                &path
                    .size_on_disk()?
                    .file_size(options::CONVENTIONAL)
                    .unwrap(),
            ),
        ]));
    }
    table.printstd();
    Ok(())
}
