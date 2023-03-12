# gazelle:repo bazel_gazelle
workspace(name = "dev_palermo_galaxy")

load("//:repositories.bzl", "repositories")

repositories()

# Buildbuddy
load("@io_buildbuddy_buildbuddy_toolchain//:deps.bzl", "buildbuddy_deps")

buildbuddy_deps()

load("@io_buildbuddy_buildbuddy_toolchain//:rules.bzl", "buildbuddy")

buildbuddy(name = "buildbuddy_toolchain")

# gRPC
load("@rules_proto_grpc//:repositories.bzl", "rules_proto_grpc_repos", "rules_proto_grpc_toolchains")

rules_proto_grpc_toolchains()

rules_proto_grpc_repos()

load("@rules_proto//proto:repositories.bzl", "rules_proto_dependencies", "rules_proto_toolchains")

rules_proto_dependencies()

rules_proto_toolchains()

# Buf
load("@rules_proto_grpc//buf:repositories.bzl", rules_proto_grpc_buf_repos = "buf_repos")

rules_proto_grpc_buf_repos()

# Go
load("@rules_proto_grpc//:repositories.bzl", "bazel_gazelle", "io_bazel_rules_go")  # buildifier: disable=same-origin-load

io_bazel_rules_go()

bazel_gazelle()

load("@rules_proto_grpc//go:repositories.bzl", rules_proto_grpc_go_repos = "go_repos")

rules_proto_grpc_go_repos()

load("@io_bazel_rules_go//go:deps.bzl", "go_register_toolchains", "go_rules_dependencies")

go_rules_dependencies()

go_register_toolchains(
    version = "1.19",
)

load("@bazel_gazelle//:deps.bzl", "gazelle_dependencies")
load("//:go_deps.bzl", "go_dependencies")

# gazelle:repository_macro go_deps.bzl%go_dependencies
go_dependencies()

gazelle_dependencies()

load("//:buf_deps.bzl", "buf_deps")

# gazelle:repository_macro buf_deps.bzl%buf_deps
buf_deps()

# pkg
load("@rules_pkg//:deps.bzl", "rules_pkg_dependencies")

rules_pkg_dependencies()

# Rust
load("@rules_proto_grpc//rust:repositories.bzl", rules_proto_grpc_rust_repos = "rust_repos")

rules_proto_grpc_rust_repos()

load("@com_github_grpc_grpc//bazel:grpc_deps.bzl", "grpc_deps")

grpc_deps()

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(edition = "2021")

load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

crate_universe_dependencies()

load("//bazel/rust:crates_repository.bzl", "crates_repository")

crates_repository()

load("@crate_index//:defs.bzl", "crate_repositories")

crate_repositories()

# Java
load("@rules_proto_grpc//java:repositories.bzl", rules_proto_grpc_java_repos = "java_repos")

rules_proto_grpc_java_repos()

# Spring Boot
load("//bazel/spring:deps.bzl", "spring_boot_deps")

spring_boot_deps()

# Container
load("//bazel/container:deps.bzl", "container_deps")

container_deps()
