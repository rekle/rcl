use std::fs;
use std::io;
use std::path::{Path, PathBuf};

struct Ls
{
    long_output: bool,
}

pub fn list_paths<P: AsRef<Path>>(path: P, recursive: bool) -> io::Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    collect_paths(path.as_ref(), recursive, &mut paths)?;
    Ok(paths)
}

fn collect_paths(path: &Path, recursive: bool, paths: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        let is_directory = entry.file_type()?.is_dir();

        paths.push(entry_path.clone());

        if recursive && is_directory {
            collect_paths(&entry_path, true, paths)?;
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


pub fn process(long_output: &bool) {
    let paths = list_paths(".", false).unwrap();

    if *long_output {
        for path in paths {
            let metadata = path.metadata().unwrap();
            print_file_type(&metadata);
            print!(" {}", metadata.len());
            println!(" {}", path.file_name().unwrap_or_default().to_string_lossy());
        }
    }
    else {
        for path in paths {
            println!(" {}", path.file_name().unwrap_or_default().to_string_lossy());
        }
    }
}


#[cfg(test)]
mod tests {
    use super::list_paths;
    use std::fs::{self, File};

    #[test]
    fn lists_immediate_entries_without_recursing() {
        let root = tempfile_directory("shallow");
        let nested = root.join("nested");
        fs::create_dir(&nested).unwrap();
        File::create(root.join("file.txt")).unwrap();
        File::create(nested.join("nested.txt")).unwrap();

        let paths = list_paths(&root, false).unwrap();

        assert_eq!(paths.len(), 2);
        assert!(paths.contains(&nested));
        assert!(paths.contains(&root.join("file.txt")));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn lists_nested_entries_when_recursing() {
        let root = tempfile_directory("recursive");
        let nested = root.join("nested");
        fs::create_dir(&nested).unwrap();
        let nested_file = nested.join("nested.txt");
        File::create(&nested_file).unwrap();

        let paths = list_paths(&root, true).unwrap();

        assert_eq!(paths.len(), 2);
        assert!(paths.contains(&nested));
        assert!(paths.contains(&nested_file));
        fs::remove_dir_all(root).unwrap();
    }

    fn tempfile_directory(name: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!("rcl-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir(&path).unwrap();
        path
    }
}