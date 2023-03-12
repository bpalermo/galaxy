load("@io_bazel_rules_docker//container:container.bzl", _container_pull = "container_pull")

def custom_images():
    _container_pull(
        name = "envoy_amd64",
        digest = "sha256:c16ecaa6f4de8575b9cb22f6836290530cd65b2f89c06701d81de4d94b342eb2",
        registry = "index.docker.io",
        repository = "envoyproxy/envoy",
    )
    _container_pull(
        name = "envoy_arm64",
        digest = "sha256:b0fb1c9e5884a82c78e32db131395388ddd2ad616530a12033a16b4faecb8d15",
        registry = "index.docker.io",
        repository = "envoyproxy/envoy",
    )
