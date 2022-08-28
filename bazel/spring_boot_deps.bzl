load("@rules_jvm_external//:defs.bzl", "maven_install")
load("@rules_jvm_external//:specs.bzl", "maven")

SPRING_VERSION = "5.3.22"
SPRING_BOOT_VERSION = "2.7.3"

SPRING_BOOT_API_DEPS = [
    "@maven//:org_jboss_xnio_xnio_nio",
    "@maven//:org_springframework_boot_spring_boot",
    "@maven//:org_springframework_boot_spring_boot_autoconfigure",
    "@maven//:org_springframework_boot_spring_boot_starter_undertow",
    "@maven//:org_springframework_boot_spring_boot_starter_web",
    "@maven//:org_springframework_spring_beans",
    "@maven//:org_springframework_spring_web",
]

def spring_boot_deps():
    maven_install(
        artifacts = [
            maven.artifact(
                group = "org.springframework.boot",
                artifact = "spring-boot-autoconfigure",
                version = SPRING_BOOT_VERSION,
            ),
            maven.artifact(
                group = "org.springframework.boot",
                artifact = "spring-boot-starter-web",
                version = SPRING_BOOT_VERSION,
                exclusions = [
                    maven.exclusion(
                        group = "org.springframework.boot",
                        artifact = "spring-boot-starter-tomcat",
                    ),
                ],
            ),
            maven.artifact(
                group = "org.springframework.boot",
                artifact = "spring-boot-starter-undertow",
                version = SPRING_BOOT_VERSION,
            ),
            maven.artifact(
                group = "org.springframework",
                artifact = "spring-web",
                version = SPRING_VERSION,
            ),
            maven.artifact(
                group = "org.springframework",
                artifact = "spring-beans",
                version = SPRING_VERSION,
            ),
            maven.artifact(
                group = "org.springframework.boot",
                artifact = "spring-boot",
                version = SPRING_BOOT_VERSION,
            ),
            maven.artifact(
                group = "org.jboss.xnio",
                artifact = "xnio-nio",
                version = "3.8.7.Final",
            ),
        ],
        repositories = [
            "https://repo1.maven.org/maven2",
        ],
        fetch_sources = True,
    )
