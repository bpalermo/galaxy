# gazelle:repo bazel_gazelle
workspace(name = "dev_palermo_galaxy")

load("//:repositories.bzl", "repositories")

repositories()

load("@io_buildbuddy_buildbuddy_toolchain//:deps.bzl", "buildbuddy_deps")

buildbuddy_deps()

load("@io_buildbuddy_buildbuddy_toolchain//:rules.bzl", "buildbuddy")

buildbuddy(name = "buildbuddy_toolchain")

load("@rules_buf//buf:repositories.bzl", "rules_buf_dependencies", "rules_buf_toolchains")

rules_buf_dependencies()

rules_buf_toolchains(version = "v1.5.0")

load("@rules_proto_grpc//:repositories.bzl", "rules_proto_grpc_repos", "rules_proto_grpc_toolchains")

rules_proto_grpc_toolchains()

rules_proto_grpc_repos()

load("@rules_proto//proto:repositories.bzl", "rules_proto_dependencies", "rules_proto_toolchains")

rules_proto_dependencies()

rules_proto_toolchains()

# Go
load("@rules_proto_grpc//:repositories.bzl", "bazel_gazelle", "io_bazel_rules_go")  # buildifier: disable=same-origin-load

io_bazel_rules_go()

bazel_gazelle()

load("@rules_buf//gazelle/buf:repositories.bzl", "gazelle_buf_dependencies")

gazelle_buf_dependencies()

load("@rules_proto_grpc//go:repositories.bzl", rules_proto_grpc_go_repos = "go_repos")

rules_proto_grpc_go_repos()

# Load the dependencies of PGV. This is required for the next step to work.
load("@com_envoyproxy_protoc_gen_validate//bazel:repositories.bzl", "pgv_dependencies")

pgv_dependencies()

# Perform any necessary actions to initialize dependencies.
load("@com_envoyproxy_protoc_gen_validate//bazel:dependency_imports.bzl", "pgv_dependency_imports")

pgv_dependency_imports()

load("@com_envoyproxy_protoc_gen_validate//:dependencies.bzl", "go_third_party")

go_third_party()

# Gazelle
load("@bazel_gazelle//:deps.bzl", "gazelle_dependencies", "go_repository")
load("//:go_deps.bzl", "go_dependencies")

# gazelle:repository_macro go_deps.bzl%go_dependencies
go_dependencies()

gazelle_dependencies()

# Rust
load("@rules_proto_grpc//rust:repositories.bzl", rules_proto_grpc_rust_repos = "rust_repos")

rules_proto_grpc_rust_repos()

load("@com_github_grpc_grpc//bazel:grpc_deps.bzl", "grpc_deps")

grpc_deps()

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(edition = "2021")

load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

crate_universe_dependencies(bootstrap = True)

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
