use anyhow::Result;
use chrono::{Local, TimeZone};
use filesize::PathExt;
use humansize::{file_size_opts as options, FileSize};
use prettytable::format;
use prettytable::{Cell, Row, Table};
use std::process;
use std::{fs, os::unix::prelude::CommandExt};
use std::{os::linux::fs::MetadataExt, path::Path};
use structopt::StructOpt;
use users::{get_group_by_gid, get_user_by_uid};

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

    return if path.is_file() {
        if SIZE_LESS < path.size_on_disk()? {
            //TODO impl less
            std::process::Command::new("less").arg(path).exec();
            return Ok(());
        }

        read_file(path)
    } else {
        list_dir(path)
    };
}

fn read_file(path: &Path) -> Result<()> {
    match std::fs::read_to_string(path) {
        Ok(v) => {
            println!("{}", v);
            Ok(())
        }
        Err(e) => Err(anyhow::Error::new(e)),
    }
}

#[macro_use]
extern crate prettytable;
fn list_dir(path: &Path) -> Result<()> {
    let mut table = Table::new();
    table.set_titles(row![
        "permission",
        "user",
        "group",
        "name",
        "last-modify",
        "size"
    ]);
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    for entry in fs::read_dir(path)? {
        let path = entry?.path();
        let meta = fs::metadata(&path)?;
        let uid = meta.st_uid();
        let gid = meta.st_gid();
        let stat = meta.st_mode();
        let size = &path.size_on_disk()?.file_size(options::CONVENTIONAL);
        let lmtime = Local.timestamp(meta.st_mtime(), 0);

        let size = match size {
            Ok(s) => s,
            Err(_) => continue,
        };

        table.add_row(Row::new(vec![
            Cell::new(&unix_mode::to_string(stat)),
            Cell::new(get_user_by_uid(uid).unwrap().name().to_str().unwrap()),
            Cell::new(get_group_by_gid(gid).unwrap().name().to_str().unwrap()),
            Cell::new(&path.display().to_string()),
            Cell::new(&lmtime.to_string()),
            Cell::new(size),
        ]));
    }
    table.printstd();
    Ok(())
}
