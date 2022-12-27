load("@rules_rust//crate_universe:defs.bzl", "crate", _crates_repository = "crates_repository")

def crates_repository():
    _crates_repository(
        name = "crate_index",
        cargo_lockfile = "//bazel/rust:Cargo.lock",
        generator_sha256s = {
            "x86_64-unknown-linux-gnu": "48dc0990d0742dce8902319ac9762983280f8ec66a7f0da669ead38e84e3578d",
        },
        generator_urls = {
            "aarch64-apple-darwin": "https://github.com/bazelbuild/rules_rust/releases/download/0.14.0/cargo-bazel-aarch64-apple-darwin",
            "x86_64-aarch64-unknown-linux-gnu": "https://github.com/bazelbuild/rules_rust/releases/download/0.14.0/cargo-bazel-aarch64-unknown-linux-gnu",
            "x86_64-apple-darwin": "https://github.com/bazelbuild/rules_rust/releases/download/0.14.0/cargo-bazel-x86_64-apple-darwin",
            "x86_64-unknown-linux-gnu": "https://github.com/bazelbuild/rules_rust/releases/download/0.14.0/cargo-bazel-x86_64-unknown-linux-gnu",
        },
        lockfile = "//bazel/rust:Cargo.Bazel.lock",
        packages = {
            "env_logger": crate.spec(
                version = "0.10.0",
            ),
            "figment": crate.spec(
                version = "0.10.8",
                features = [
                    "env",
                    "toml",
                ],
            ),
            "glob": crate.spec(
                version = "0.3.0",
            ),
            "log": crate.spec(
                version = "0.4.17",
            ),
            "mockall": crate.spec(
                version = "0.11.3",
            ),
            "num-traits": crate.spec(
                version = "0.2.15",
            ),
            "once_cell": crate.spec(
                version = "1.16.0",
            ),
            "prost": crate.spec(
                version = "0.11.5",
            ),
            "prost-build": crate.spec(
                version = "0.11.5",
            ),
            "prost-derive": crate.spec(
                version = "0.11.5",
            ),
            "prost-types": crate.spec(
                version = "0.11.5",
            ),
            "rust_decimal": crate.spec(
                version = "1.27.0",
                features = [
                    "maths",
                ],
            ),
            "sea-orm": crate.spec(
                version = "0.10.5",
                features = [
                    "macros",
                    "mock",
                    "runtime-tokio-rustls",
                    "sqlx-mysql",
                ],
            ),
            "sea-orm-migration": crate.spec(
                version = "0.10.5",
            ),
            "serde": crate.spec(
                version = "1.0.151",
                features = [
                    "derive",
                ],
            ),
            "serde_derive": crate.spec(
                version = "1.0.151",
            ),
            "serde_json": crate.spec(
                version = "1.0.91",
            ),
            "time": crate.spec(
                version = "0.3.17",
            ),
            "tokio": crate.spec(
                version = "1.23.0",
                features = [
                    "macros",
                    "rt-multi-thread",
                ],
            ),
            "testcontainers": crate.spec(
                version = "0.14.0",
            ),
            "tonic": crate.spec(
                version = "0.8.3",
            ),
            "tonic-build": crate.spec(
                version = "0.8.4",
            ),
            "tonic-health": crate.spec(
                version = "0.8.0",
            ),
            "uuid": crate.spec(
                version = "1.2.2",
                features = [
                    "v4",
                    "fast-rng",
                    "macro-diagnostics",
                ],
            ),
        },
    )
