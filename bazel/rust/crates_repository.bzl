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
            "log": crate.spec(
                version = "0.4.17",
            ),
            "mockall": crate.spec(
                version = "0.11.3",
            ),
            "tokio": crate.spec(
                version = "1.23.0",
                features = [
                    "macros",
                    "rt-multi-thread",
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
            "testcontainers": crate.spec(
                version = "0.14.0",
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
