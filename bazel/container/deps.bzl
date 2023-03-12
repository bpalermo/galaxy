load(":images.bzl", _custom_images = "custom_images")
load("@io_bazel_rules_docker//go:image.bzl", _go_image_repos = "repositories")
load("@io_bazel_rules_docker//java:image.bzl", _java_image_repos = "repositories")
load("@io_bazel_rules_docker//repositories:deps.bzl", _container_deps = "deps")
load("@io_bazel_rules_docker//repositories:repositories.bzl", _container_repositories = "repositories")
load("@io_bazel_rules_docker//rust:image.bzl", _rust_image_repos = "repositories")

def container_deps():
    _container_repositories()
    _container_deps()
    _go_image_repos()
    _java_image_repos()
    _rust_image_repos()
    _custom_images()
