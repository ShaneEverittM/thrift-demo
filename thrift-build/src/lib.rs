use std::{
    env,
    error::Error,
    path::Path,
    process::{exit, Command},
};

use thrift_vendor::thrift;

pub fn generate_rust<P>(thrift_file: P) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let out_dir = env::var("OUT_DIR")?;

    let status = Command::new(thrift())
        .arg("--gen")
        .arg("rs")
        .arg("-out")
        .arg(&out_dir)
        .arg(thrift_file.as_ref())
        .spawn()?
        .wait()?;

    if !status.success() {
        println!("Failed to run thrift compiler");
        exit(1);
    }

    println!("cargo:rustc-env=THRIFT_OUT_DIR={}", out_dir);

    Ok(())
}
