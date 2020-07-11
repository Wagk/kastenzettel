use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Zettel-rs")]
struct Args {
    #[structopt(long, short)]
    debug: bool,
    #[structopt(long, parse(from_os_str))]
    directory: PathBuf,
}

// TODO: implement
/// Collects all files (including those files in directories) for
/// zettel analysis
fn collect_files_nested(paths: &Path) -> Vec<PathBuf> {
    if paths.is_file() {
        return vec![paths.to_path_buf()];
    }

    let mut vec = Vec::<PathBuf>::new();

    for path in paths.read_dir().unwrap() {
        match path.unwrap().path() {
            p if p.is_dir() => vec.append(&mut collect_files_nested(&p)),
            p if p.is_file() => vec.push(p),
            _ => panic!(),
        }
    }

    vec
}

fn main() {
    let opt = Args::from_args();
}

#[cfg(test)]
mod tests {
    use super::*;

    const ZETTEL_DIR: &'static str = "tests/zettel/";

    fn get_args() -> Args {
        let args = vec!["test.exe", "--directory", ZETTEL_DIR];
        Args::from_iter(args.into_iter())
    }

    /// Checks that the test directory containing all the zettels
    /// exist
    #[test]
    fn check_test_zettel_directory() {
        let opt = get_args();

        assert!(opt.directory.exists());
    }

    #[test]
    fn test_read_dir() {
        let opt = get_args();
        let files: Vec<_> = opt
            .directory
            .read_dir()
            .expect("Can't access ZETTEL_DIR")
            .map(|x| x.unwrap())
            .collect();

        dbg!(files);
    }
}
