use std::path::PathBuf;

pub fn thrift() -> PathBuf {
    PathBuf::from(env!("INSTALL_DIR"))
        .join("bin")
        .join("thrift")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_exists() {
        assert!(dbg!(thrift()).exists())
    }
}
