load("@rules_jvm_external//:defs.bzl", "maven_install")
load("@rules_jvm_external//:specs.bzl", "maven")

SPRING_VERSION = "5.3.22"
SPRING_BOOT_VERSION = "2.7.4"

SPRING_BOOT_API_DEPS = [
    "@maven//:org_jboss_xnio_xnio_nio",
    "@maven//:org_springframework_boot_spring_boot",
    "@maven//:org_springframework_boot_spring_boot_autoconfigure",
    "@maven//:org_springframework_boot_spring_boot_starter_undertow",
    "@maven//:org_springframework_boot_spring_boot_starter_web",
    "@maven//:org_springframework_boot_spring_boot_starter_actuator",
    "@maven//:org_springframework_spring_beans",
    "@maven//:org_springframework_spring_web",
]

SPRING_BOOT_API_TEST_DEPS = SPRING_BOOT_API_DEPS + [
    "@maven//:junit_junit",
    "@maven//:org_junit_vintage_junit_vintage_engine",
    "@maven//:org_springframework_spring_test",
    "@maven//:org_springframework_boot_spring_boot_test",
    "@maven//:io_rest_assured_rest_assured",
    "@maven//:org_hamcrest_hamcrest",
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
                group = "org.springframework.boot",
                artifact = "spring-boot-starter-actuator",
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
            # Test
            maven.artifact(
                group = "junit",
                artifact = "junit",
                version = "4.13.2",
                testonly = True,
            ),
            maven.artifact(
                group = "org.springframework",
                artifact = "spring-test",
                version = SPRING_VERSION,
                testonly = True,
            ),
            maven.artifact(
                group = "org.springframework.boot",
                artifact = "spring-boot-starter-test",
                version = SPRING_BOOT_VERSION,
                testonly = True,
            ),
            maven.artifact(
                group = "org.springframework.boot",
                artifact = "spring-boot-test",
                version = SPRING_BOOT_VERSION,
                testonly = True,
            ),
            maven.artifact(
                group = "org.junit.vintage",
                artifact = "junit-vintage-engine",
                version = "5.9.0",
                testonly = True,
                exclusions = [
                    maven.exclusion(
                        group = "org.hamcrest",
                        artifact = "hamcrest-core",
                    ),
                ],
            ),
            maven.artifact(
                group = "io.rest-assured",
                artifact = "rest-assured",
                version = "5.2.0",
                testonly = True,
            ),
            maven.artifact(
                group = "org.hamcrest",
                artifact = "hamcrest",
                version = "2.2",
                testonly = True,
            ),
        ],
        repositories = [
            "https://repo1.maven.org/maven2",
        ],
        fetch_sources = True,
    )
