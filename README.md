# bcat (transparently ls and cat, lcat renamed bcat)

A command-line tool that allows transparent ls and cat operations regardless of whether they are files or directories.

Also for practicing Rust.

## install

### by cargo

```sh
cargo install bcat
```

### by release binary for linux x86

```sh
curl -LO https://github.com/bootjp/bcat/releases/latest/download/bcat
chmod +x bcat
./bcat .
sudo cp bcat /usr/local/bin/
```

## example

### for dir

![directory example](.doc/dir.png)

### for file

![file example](.doc/file.png)
