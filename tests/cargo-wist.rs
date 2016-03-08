#[macro_use]
extern crate assert_cli;

#[test]
fn listing() {
    assert_cli!("target/debug/cargo-wist",
                &["wist", "--manifest-path=tests/fixtures/wist/Cargo.toml.sample"] =>
                Success, r#"cargo-edit      path: "../../../"
clippy          git: "https://github.com/Manishearth/rust-clippy.git" (optional)
docopt          0.6
pad             0.1
rustc-serialize 0.3
semver          0.1
toml            0.1"#)
        .unwrap();
}

#[test]
fn listing_dev() {
    assert_cli!("target/debug/cargo-wist",
                &["wist", "--dev", "--manifest-path=tests/fixtures/wist/Cargo.toml.sample"] =>
                Success, r#"term 0.2.12"#)
        .unwrap();
}

#[test]
fn listing_build() {
    assert_cli!("target/debug/cargo-wist",
                &["wist", "--build", "--manifest-path=tests/fixtures/wist/Cargo.toml.sample"] =>
                Success, r#"gcc 0.3.19"#)
        .unwrap();
}

#[test]
fn treat_missing_section_as_empty() {
    // empty dependencies
    assert_cli!("target/debug/cargo-wist",
                &["wist", "--manifest-path=tests/fixtures/wist-empty/Cargo.toml.sample"] =>
                Success, "\n").unwrap();

    // empty dev-dependencies
    assert_cli!("target/debug/cargo-wist",
                &["wist", "--dev", "--manifest-path=tests/fixtures/wist-empty/Cargo.toml.sample"] =>
                Success, "\n").unwrap();

    // empty build-dependencies
    assert_cli!("target/debug/cargo-wist",
                &["wist", "--build", "--manifest-path=tests/fixtures/wist-empty/Cargo.toml.sample"] =>
                Success, "\n").unwrap();
}

#[test]
fn tree() {
    assert_cli!("target/debug/cargo-wist",
                &["wist", "--tree", "--manifest-path=tests/fixtures/tree/Cargo.lock"] =>
                Success, r#"├── clippy (0.0.5)
├── docopt (0.6.67)
│   ├── regex (0.1.38)
│   │   ├── aho-corasick (0.2.1)
│   │   │   └── memchr (0.1.3)
│   │   │       └── libc (0.1.8)
│   │   ├── memchr (0.1.3)
│   │   │   └── libc (0.1.8)
│   │   └── regex-syntax (0.1.2)
│   ├── rustc-serialize (0.3.15)
│   └── strsim (0.3.0)
├── pad (0.1.4)
│   └── unicode-width (0.1.1)
├── rustc-serialize (0.3.15)
├── semver (0.1.19)
└── toml (0.1.20)
    └── rustc-serialize (0.3.15)"#)
        .unwrap();
}

#[test]
fn unknown_flags() {
    assert_cli!("target/debug/cargo-wist", &["wist", "foo", "--flag"] => Error 1,
                r#"Unknown flag: '--flag'

Usage:
    cargo wist [--dev|--build] [options]
    cargo wist --tree
    cargo wist (-h|--help)
    cargo wist --version"#)
        .unwrap();
}
