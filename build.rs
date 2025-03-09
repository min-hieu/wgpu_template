use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let slang_files = find_slang_files(".");

    for path in slang_files {
        let shader_dir = path.parent().unwrap();
        // let wgsl_out_dir = shader_dir.join("wgsl_out");
        let wgsl_out_dir = shader_dir;
        fs::create_dir_all(&wgsl_out_dir).expect("Failed to create output directory");

        let shader_name = path.file_stem().unwrap().to_str().unwrap();

        compile_shader(&path, &wgsl_out_dir.join(format!("{}_vs.wgsl", shader_name)), "vs_main", "vertex");
        compile_shader(&path, &wgsl_out_dir.join(format!("{}_fs.wgsl", shader_name)), "fs_main", "fragment");
    }

    println!("cargo:rerun-if-changed=.");
}

fn find_slang_files(dir: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if path.is_dir() {
            files.extend(find_slang_files(path.to_str().unwrap()));
        } else if path.extension().map_or(false, |ext| ext == "slang") {
            files.push(path);
        }
    }
    files
}

fn compile_shader(input: &Path, output: &Path, entry: &str, stage: &str) {
    let status = Command::new("slangc")
        .arg(input.to_str().unwrap())
        .arg("-target")
        .arg("wgsl")
        .arg("-entry")
        .arg(entry)
        .arg("-o")
        .arg(output.to_str().unwrap())
        .arg("-stage")
        .arg(stage)
        .status()
        .expect("Failed to compile shader");

    if !status.success() {
        panic!("Shader compilation failed: {:?}", input);
    }
}
