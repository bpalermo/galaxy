"""Load dependencies needed to compile galaxy."""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

BUILDBUDDY_TOOLCHAIN_RULE_SHA = "a2a5cccec251211e2221b1587af2ce43c36d32a42f5d881737db3b546a536510"
BUILDBUDDY_TOOLCHAIN_RULE_VERSION = "829c8a574f706de5c96c54ca310f139f4acda7dd"

PROTO_GRPC_RULE_SHA = "fb7fc7a3c19a92b2f15ed7c4ffb2983e956625c1436f57a3430b897ba9864059"
PROTO_GRPC_RULE_VERSION = "4.3.0"

KOTLIN_RULES_VERSION = "1.6.0"
KOTLIN_RULES_SHA = "a57591404423a52bd6b18ebba7979e8cd2243534736c5c94d35c89718ea38f94"

JAVA_RULES_VERSION = "5.1.0"
JAVA_RULES_SHA = "d974a2d6e1a534856d1b60ad6a15e57f3970d8596fbb0bb17b9ee26ca209332a"

RULES_JVM_EXTERNAL_TAG = "4.2"
RULES_JVM_EXTERNAL_SHA = "cd1a77b7b02e8e008439ca76fd34f5b07aecb8c752961f9640dea15e9e5ba1ca"

DOCKER_RULE_VERSION = "v0.25.0"
DOCKER_RULE_SHA = "b1e80761a8a8243d03ebca8845e9cc1ba6c82ce7c5179ce2b295cd36f7e394bf"

RULES_BUF_VERSION = "0.1.1"
RULES_BUF_SHA = "523a4e06f0746661e092d083757263a249fedca535bd6dd819a8c50de074731a"

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
            urls = ["https://github.com/rules-proto-grpc/rules_proto_grpc/archive/%s.tar.gz" % PROTO_GRPC_RULE_VERSION],
        )

    if not native.existing_rule("io_bazel_rules_kotlin"):
        http_archive(
            name = "io_bazel_rules_kotlin",
            sha256 = KOTLIN_RULES_SHA,
            urls = ["https://github.com/bazelbuild/rules_kotlin/releases/download/v%s/rules_kotlin_release.tgz" % KOTLIN_RULES_VERSION],
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
