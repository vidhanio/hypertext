fn main() {
    println!("cargo:rerun-if-changed=src/views");

    // Process tailwind CSS
    std::process::Command::new("npx")
        .arg("@tailwindcss/cli")
        .arg("-i")
        .arg("./tailwind.css")
        .arg("-o")
        .arg("static/output.css")
        .arg("--minify")
        .output()
        .expect("Failed to execute tailwindcss");
}
