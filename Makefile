.PHONY: update-repos
update-repos:
	@bazel run //:gazelle-update-repos

.PHONY: gazelle
gazelle: update-repos
	@bazel run //:gazelle

.PHONY: test
test: gazelle
	@bazel test //...

.PHONY: coverage
coverage: gazelle
	@bazel coverage --combined_report=lcov //...

.PHONY: clean
clean:
	@bazel clean

build: gazelle
	@bazel build //...
