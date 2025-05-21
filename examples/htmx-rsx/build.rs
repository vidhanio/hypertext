use std::process::Command;

const TAILWIND_CSS: &str = "tailwind.css";

fn main() {
    println!("cargo:rerun-if-changed=src/views/");

    let output = Command::new("tailwindcss")
        .args(["-i", TAILWIND_CSS, "-o", "static/styles.css", "--minify"])
        .output()
        .expect("failed to execute `tailwindcss`");

    if !output.status.success() {
        panic!(
            "failed to execute `tailwindcss`:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
