#![allow(unknown_lints)]
#![warn(clippy::all)]

use std::env;
use std::iter::*;
use std::result::Result;
use std::{error::Error, fs, path::PathBuf};
use regex::Regex;

fn main() -> Result<(), Box<Error>> {
    fn getpath(args: &[String]) -> String {
        match args.len() {
            0 | 1 => ".".to_owned(),
            2 => args[1].to_owned(),
            _ => panic!("More than one argument is not supported"),
        }
    }
    fn printerr<T: std::fmt::Display> (e: T) -> T {eprintln!("{}", e); e};

    let start_dir = getpath(&env::args().collect::<Vec<String>>()[..]);
    let mut matchopts = glob::MatchOptions::new();
    matchopts.case_sensitive = false;
    let pb = PathBuf::from(&start_dir).join("src").join("*.md");
    println!("{}", pb.to_string_lossy());
    let fns = glob::glob_with(
        &pb.to_str().expect("Non-unicode chars in filename"),
        matchopts
    )?
    .filter_map(|p_| {
        let path = p_.map_err(printerr).ok()?;
        let contents = fs::read_to_string(&path).map_err(printerr).ok()?;
        Some((path, contents))
    });
    let re = Regex::new(r"(\[[\s\w]+\]\(../)(.+)(\.html\) instead\.)").unwrap();
    let _o = fns.filter_map(|(path, contents)| {
        let path: PathBuf = path;
        let rootdir = path.parent()?.parent()?.parent()?.join("src");
        let relpath = path.strip_prefix(&start_dir).ok()?.strip_prefix("src").ok()?;
        let found = re.captures(&contents)?.get(2)?.as_str();
        // println!("{:?}, {:?}: {}",rootdir, relpath, found);
        let newcnt = if rootdir.join(&relpath).exists() && &found.to_lowercase() == "index" {
            let noext = rootdir.join(&relpath).file_stem()?.to_str()?.to_owned();
            let ret = String::from(re.replace(&contents, |caps: &regex::Captures| {format!("{}{}{}", &caps[1], &noext, &caps[3] )}));
            // println!("Replacing '{}' to '{}':\n{}", found, noext, ret);
            ret
        } else {
            println!(">> Not replacing {:?}", path);
            contents.clone()
        };
        Some((path, newcnt))
    });
    for (path, newcnt) in _o {
        // println!("!!!{:?}", path);
        fs::write(path, newcnt).map_err(printerr).ok();
    }; // unlazy
    Ok(())
}
