"""Load dependencies needed to compile galaxy."""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

PROTO_GRPC_RULE_SHA = "bbe4db93499f5c9414926e46f9e35016999a4e9f6e3522482d3760dc61011070"
PROTO_GRPC_RULE_VERSION = "4.2.0"

JAVA_RULES_VERSION = "5.1.0"
JAVA_RULES_SHA = "d974a2d6e1a534856d1b60ad6a15e57f3970d8596fbb0bb17b9ee26ca209332a"

RULES_JVM_EXTERNAL_TAG = "4.2"
RULES_JVM_EXTERNAL_SHA = "cd1a77b7b02e8e008439ca76fd34f5b07aecb8c752961f9640dea15e9e5ba1ca"

DOCKER_RULE_VERSION = "v0.25.0"
DOCKER_RULE_SHA = "b1e80761a8a8243d03ebca8845e9cc1ba6c82ce7c5179ce2b295cd36f7e394bf"

def galaxy_deps():
    """Loads common dependencies needed to compile galaxy."""

    if not native.existing_rule("rules_proto_grpc"):
        http_archive(
            name = "rules_proto_grpc",
            sha256 = PROTO_GRPC_RULE_SHA,
            strip_prefix = "rules_proto_grpc-%s" % PROTO_GRPC_RULE_VERSION,
            urls = ["https://github.com/rules-proto-grpc/rules_proto_grpc/archive/%s.tar.gz" % PROTO_GRPC_RULE_VERSION],
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
