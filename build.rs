use clap::CommandFactory;

#[path = "src/cli.rs"]
mod cli;

/// Writes a manfile to the `target/man` directory. The name of the file
/// is dependent on the package name as set by cargo.
fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=src/cli.rs");
    println!("cargo:rerun-if-changed=build.rs");
    let out_dir = std::path::PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR").map_err(|_| std::io::ErrorKind::NotFound)?,
    );

    let pkg_name = std::env::var("CARGO_PKG_NAME").map_err(|_| std::io::ErrorKind::NotFound)?;
    let cmd = cli::Cli::command();

    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    let target_dir = out_dir.join("target").join("man");

    let _ = std::fs::create_dir(&target_dir);

    let manfile = format!("{}.1", pkg_name);

    std::fs::write(target_dir.join(manfile), &buffer)?;

    Ok(())
}
