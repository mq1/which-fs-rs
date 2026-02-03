// SPDX-FileCopyrightText: 2026 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cfg(feature = "cli")]
struct Args {
    path: String,
}

#[cfg(feature = "cli")]
fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut path = None;

    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Value(val) => {
                path = Some(val.string()?);
            }
            Short('h') | Long("help") => {
                println!("Usage: which-fs PATH");
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    Ok(Args {
        path: path.ok_or("Please specify a path")?,
    })
}

#[cfg(feature = "cli")]
fn main() -> Result<(), lexopt::Error> {
    use std::path::Path;

    let args = parse_args()?;
    let path = Path::new(&args.path);

    assert!(path.is_dir(), "The path must be a directory");

    let fs_kind = which_fs::FsKind::try_from_path(path).unwrap();
    println!("Filesystem: {fs_kind}");
    Ok(())
}
