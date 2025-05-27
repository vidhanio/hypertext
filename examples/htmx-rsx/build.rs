use std::{error::Error, process::Command};

const TAILWIND_CSS: &str = "tailwind.css";

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed={TAILWIND_CSS}");
    println!("cargo:rerun-if-changed=src/views/");

    let output = Command::new("tailwindcss")
        .args(["-i", TAILWIND_CSS, "-o", "static/styles.css", "--minify"])
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "failed to execute `tailwindcss`:\n{}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    Ok(())
}
