use std::{env, error::Error, fs::create_dir_all, fs::File, path::PathBuf};
use std::io::Write;

use reqwest::blocking::get;

static THRIFT_URL: &str = "https://dlcdn.apache.org/thrift/0.17.0/thrift-0.17.0.exe";

fn main() -> Result<(), Box<dyn Error>> {
    // TODO: Build from source on Linux
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let install_dir = out_dir.join("install");
    let bin_dir = install_dir.join("bin");
    create_dir_all(&bin_dir)?;

    File::create(bin_dir.join("thrift"))?.write_all(&get(THRIFT_URL)?.bytes()?)?;

    println!("cargo:rustc-env=INSTALL_DIR={}", install_dir.display());

    Ok(())
}
