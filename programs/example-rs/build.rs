use std::{env, path::Path, process::Command};

fn main() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir_path = Path::new(&out_dir);

    let bin_dir = out_dir_path
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    if !Command::new("ln")
        .args([
            "-sf",
            &format!("{}", bin_dir.join("example-rs").display()),
            &format!("{}", manifest_dir.join("a.out").display()),
        ])
        .status()
        .unwrap()
        .success()
    {
        panic!("could not generate symlink a.out to bin");
    }
}
