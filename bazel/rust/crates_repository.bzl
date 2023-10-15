load("@rules_proto_grpc//rust:crate_deps.bzl", "crate_repositories")
load("@rules_rust//crate_universe:defs.bzl", "crate", _crates_repository = "crates_repository")

def crates_repository():
    crate_repositories()
    _crates_repository(
        name = "crate_index",
        cargo_lockfile = "//bazel/rust:Cargo.lock",
        lockfile = "//bazel/rust:Cargo.Bazel.lock",
        packages = {
            "env_logger": crate.spec(
                version = "0.10.0",
            ),
            "chrono": crate.spec(
                version = "0.4.31",
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
                    "with-chrono",
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
