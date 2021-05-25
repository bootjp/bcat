use anyhow::Result;
use chrono::{Local, TimeZone};
use filesize::PathExt;
use humansize::{file_size_opts as options, FileSize};
use prettytable::format;
use prettytable::Table;
use std::process;
use std::{fs, os::unix::prelude::CommandExt};
use std::{io::Read, os::linux::fs::MetadataExt};
use structopt::StructOpt;
use users::{get_group_by_gid, get_user_by_uid};

#[derive(StructOpt)]
struct Cli {
    path: String,
}

const SIZE_LESS: u64 = 1024 * 10;

fn main() -> Result<()> {
    let args = Cli::from_args();

    let mut file = fs::File::open(&args.path)?;
    let metadata = file.metadata()?;

    return if metadata.is_file() {
        if SIZE_LESS < metadata.len() {
            //TODO impl less
            process::Command::new("less").arg(&args.path).exec();
            return Ok(());
        }

        read_file(&mut file)
    } else {
        list_dir(&args.path)
    };
}

fn read_file(file: &mut fs::File) -> Result<()> {
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    println!("{}", buf);
    Ok(())
}

#[macro_use]
extern crate prettytable;
fn list_dir(path: &str) -> Result<()> {
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
        let user = get_user_by_uid(uid)
            .map(|u| u.name().to_str().unwrap_or_default().to_owned())
            .unwrap_or_default();
        let gid = meta.st_gid();
        let group = get_group_by_gid(gid)
            .map(|g| g.name().to_str().unwrap_or_default().to_owned())
            .unwrap_or_default();
        let stat = meta.st_mode();
        let size = path
            .size_on_disk()?
            .file_size(options::CONVENTIONAL)
            .unwrap_or_default();
        let lmtime = Local.timestamp(meta.st_mtime(), 0);

        table.add_row(row![
            &unix_mode::to_string(stat),
            &user,
            &group,
            &path.display().to_string(),
            &lmtime.to_string(),
            &size,
        ]);
    }
    table.printstd();
    Ok(())
}
