use std::fs;
use std::path::PathBuf;
use std::process::Command;
use whython_7::root::main_args;
use whython_7::root::Args;

#[test]
fn tests() {
    fs::create_dir_all("tests/temp").unwrap();

    for entry in get_files(PathBuf::from("tests/inputs")) {
        let in_path = PathBuf::from("tests/inputs").join(entry.clone());

        let out_path = PathBuf::from("tests/outputs").join(entry);

        compare(
            in_path.into_os_string().into_string().unwrap(),
            out_path.into_os_string().into_string().unwrap(),
        );
    }
}

fn get_files(base_dir: PathBuf) -> Vec<PathBuf> {
    let mut out = Vec::new();
    walk_dir(&base_dir, PathBuf::new(), &mut out);
    out
}

fn walk_dir(base_dir: &PathBuf, prefix: PathBuf, out: &mut Vec<PathBuf>) {
    for entry in base_dir.join(prefix.clone()).read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let emit = prefix.clone().join(path.file_name().unwrap());
        if path.is_dir() {
            walk_dir(base_dir, emit, out);
            continue;
        }

        out.push(emit);
    }
}

fn compare(in_path: String, out_path: String) {
    println!("Testing `{}`", in_path);

    assert!(main_args(Args {
        input: in_path,
        output: String::from("tests/temp/out"),
        build: true
    })
    .is_ok());

    let result = Command::new("tests/temp/out.exe").output().unwrap();

    assert!(result.status.success());

    let result = String::from_utf8(result.stdout)
        .unwrap()
        .replace('\0', "")
        .replace("\r\n", "\n");

    assert_eq!(
        result.trim(),
        fs::read_to_string(out_path)
            .unwrap()
            .replace("\r\n", "\n")
            .trim()
    );
}
