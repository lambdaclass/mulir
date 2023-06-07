use std::{
    env::var,
    ffi::OsStr,
    io::{BufRead, Cursor},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

fn find_llvm_config() -> PathBuf {
    PathBuf::from(var("MLIR_SYS_160_PREFIX").expect("MLIR_SYS_160_PREFIX is not set"))
        .join("bin/llvm-config")
}

fn find_clang() -> PathBuf {
    PathBuf::from(var("MLIR_SYS_160_PREFIX").expect("MLIR_SYS_160_PREFIX is not set"))
        .join("bin/clang++")
}

pub fn extract_clang_include_paths(path: &Path) -> Vec<String> {
    let process = Command::new(find_clang())
        .arg("-c")
        .arg("-v")
        .arg(path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let output = process.wait_with_output().unwrap();
    let mut stderr = Cursor::new(output.stderr);

    let mut buffer = String::new();
    while stderr.read_line(&mut buffer).unwrap() != 0
        && buffer != "#include \"...\" search starts here:\n"
    {
        buffer.clear();
    }

    let mut include_paths = Vec::new();
    while stderr.read_line(&mut buffer).unwrap() != 0 && buffer != "End of search list.\n" {
        if buffer.starts_with("#include") && buffer.ends_with("search starts here:\n") {
            buffer.clear();
            continue;
        }

        let include_path = buffer.trim();
        include_paths.push(include_path.to_string());

        buffer.clear();
    }
    buffer.clear();

    // Append paths from `llvm-config`.
    let process = Command::new(find_llvm_config())
        .arg("--includedir")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();

    let output = process.wait_with_output().unwrap();
    let mut stdout = Cursor::new(output.stdout);
    while stdout.read_line(&mut buffer).unwrap() != 0 {
        let include_path = buffer.trim();
        include_paths.push(include_path.to_string());
        buffer.clear();
    }

    include_paths
}

pub fn build_auxiliary_library(target_path: &Path, source_path: &Path) {
    assert_eq!(source_path.extension().and_then(OsStr::to_str), Some("cpp"));
    assert_eq!(target_path.extension().and_then(OsStr::to_str), Some("a"));

    todo!("build and package auxiliary library")
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn test_find_clang() {
        assert!(find_clang().exists());
    }

    #[test]
    fn test_extract_clang_include_paths() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_cpp = temp_dir.path().join("source.cpp");
        fs::write(&temp_cpp, b"").unwrap();

        let include_paths = extract_clang_include_paths(&temp_cpp);
        assert!(!include_paths.is_empty());
    }
}
