use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/views");

    let output = Command::new("npx")
        .arg("@tailwindcss/cli")
        .arg("-i")
        .arg("./tailwind.css")
        .arg("-o")
        .arg("static/output.css")
        .arg("--minify")
        .output()
        .expect("Failed to execute tailwindcss");

    if !output.status.success() {
        panic!(
            "Failed to execute tailwindcss\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
