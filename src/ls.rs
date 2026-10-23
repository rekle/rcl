use std::fs;
use std::io;
use std::path::{Path, PathBuf};


pub struct Ls
{
    pub long_output: bool,
}

impl Ls {
    pub fn process(&self) -> i32 {
        let paths = Ls::list_paths(".", false).unwrap();

        if self.long_output {
            for path in paths {
                let metadata = path.metadata().unwrap();
                Ls::print_file_type(&metadata);
                print!(" {}", metadata.len());
                println!(" {}", path.file_name().unwrap_or_default().to_string_lossy());
            }
        }
        else {
            for path in paths {
                println!(" {}", path.file_name().unwrap_or_default().to_string_lossy());
            }
        }

        0
    }

    fn list_paths<P: AsRef<Path>>(path: P, recursive: bool) -> io::Result<Vec<PathBuf>> {
        let mut paths = Vec::new();
        Ls::collect_paths(path.as_ref(), recursive, &mut paths)?;
        Ok(paths)
    }

    fn collect_paths(path: &Path, recursive: bool, paths: &mut Vec<PathBuf>) -> io::Result<()> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            let is_directory = entry.file_type()?.is_dir();

            paths.push(entry_path.clone());

            if recursive && is_directory {
                Ls::collect_paths(&entry_path, true, paths)?;
            }
        }

        Ok(())
    }

    fn print_file_type(metadata: &fs::Metadata) {
        if metadata.is_dir() {
            print!("d");
        } else if metadata.is_file() {
            print!("-");
        } else if metadata.is_symlink() {
            print!("s");
        } else {
            print!("?");
        }
    }
}
