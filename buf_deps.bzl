load("@rules_buf//buf:defs.bzl", "buf_dependencies")

def buf_deps():
    buf_dependencies(
        name = "buf_deps_validate",
        modules = [
            "buf.build/envoyproxy/protoc-gen-validate:6607b10f00ed4a3d98f906807131c44a",
        ],
    )
