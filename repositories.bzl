"""Load dependencies needed to compile galaxy."""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

BUILDBUDDY_TOOLCHAIN_RULE_SHA = "e899f235b36cb901b678bd6f55c1229df23fcbc7921ac7a3585d29bff2bf9cfd"
BUILDBUDDY_TOOLCHAIN_RULE_VERSION = "fd351ca8f152d66fc97f9d98009e0ae000854e8f"

PROTO_GRPC_RULE_SHA = "9ba7299c5eb6ec45b6b9a0ceb9916d0ab96789ac8218269322f0124c0c0d24e2"
PROTO_GRPC_RULE_VERSION = "4.5.0"

JAVA_RULES_VERSION = "5.4.0"
JAVA_RULES_SHA = "9b87757af5c77e9db5f7c000579309afae75cf6517da745de01ba0c6e4870951"

RULES_JVM_EXTERNAL_TAG = "4.5"
RULES_JVM_EXTERNAL_SHA = "b17d7388feb9bfa7f2fa09031b32707df529f26c91ab9e5d909eb1676badd9a6"

DOCKER_RULE_VERSION = "v0.25.0"
DOCKER_RULE_SHA = "b1e80761a8a8243d03ebca8845e9cc1ba6c82ce7c5179ce2b295cd36f7e394bf"

RULES_BUF_VERSION = "0.1.1"
RULES_BUF_SHA = "523a4e06f0746661e092d083757263a249fedca535bd6dd819a8c50de074731a"

PROTO_GEN_VALIDATE_VERSION = "1.0.2"
PROTO_GEN_VALIDATE_SHA = "0b1b1ea8c248dce8c7592dc1a93e4adebd116f0d68123f8eb34251e7ce410866"

def repositories():
    """Loads common dependencies needed to compile galaxy."""

    if not native.existing_rule("io_buildbuddy_buildbuddy_toolchain"):
        http_archive(
            name = "io_buildbuddy_buildbuddy_toolchain",
            sha256 = BUILDBUDDY_TOOLCHAIN_RULE_SHA,
            strip_prefix = "buildbuddy-toolchain-{}".format(BUILDBUDDY_TOOLCHAIN_RULE_VERSION),
            urls = ["https://github.com/buildbuddy-io/buildbuddy-toolchain/archive/{}.tar.gz".format(BUILDBUDDY_TOOLCHAIN_RULE_VERSION)],
        )

    if not native.existing_rule("rules_proto_grpc"):
        http_archive(
            name = "rules_proto_grpc",
            sha256 = PROTO_GRPC_RULE_SHA,
            strip_prefix = "rules_proto_grpc-%s" % PROTO_GRPC_RULE_VERSION,
            urls = ["https://github.com/rules-proto-grpc/rules_proto_grpc/releases/download/{}/rules_proto_grpc-{}.tar.gz".format(PROTO_GRPC_RULE_VERSION, PROTO_GRPC_RULE_VERSION)],
        )

    if not native.existing_rule("rules_java"):
        http_archive(
            name = "rules_java",
            url = "https://github.com/bazelbuild/rules_java/releases/download/" + JAVA_RULES_VERSION + "/rules_java-" + JAVA_RULES_VERSION + ".tar.gz",
            sha256 = JAVA_RULES_SHA,
        )

    if not native.existing_rule("rules_jvm_external"):
        http_archive(
            name = "rules_jvm_external",
            strip_prefix = "rules_jvm_external-%s" % RULES_JVM_EXTERNAL_TAG,
            sha256 = RULES_JVM_EXTERNAL_SHA,
            url = "https://github.com/bazelbuild/rules_jvm_external/archive/%s.zip" % RULES_JVM_EXTERNAL_TAG,
        )

    if not native.existing_rule("io_bazel_rules_docker"):
        http_archive(
            name = "io_bazel_rules_docker",
            sha256 = DOCKER_RULE_SHA,
            urls = ["https://github.com/bazelbuild/rules_docker/releases/download/{}/rules_docker-{}.tar.gz".format(DOCKER_RULE_VERSION, DOCKER_RULE_VERSION)],
        )

    if not native.existing_rule("rules_buf"):
        http_archive(
            name = "rules_buf",
            sha256 = RULES_BUF_SHA,
            strip_prefix = "rules_buf-%s" % RULES_BUF_VERSION,
            urls = ["https://github.com/bufbuild/rules_buf/archive/refs/tags/v{}.zip".format(RULES_BUF_VERSION)],
        )

    if not native.existing_rule("com_envoyproxy_protoc_gen_validate"):
        http_archive(
            name = "com_envoyproxy_protoc_gen_validate",
            sha256 = PROTO_GEN_VALIDATE_SHA,
            strip_prefix = "protoc-gen-validate-{}".format(PROTO_GEN_VALIDATE_VERSION),
            urls = ["https://github.com/bufbuild/protoc-gen-validate/archive/refs/tags/v{}.zip".format(PROTO_GEN_VALIDATE_VERSION)],
        )
