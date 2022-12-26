.PHONY: update-repos
update-repos:
	@bazel run //:gazelle-update-repos

.PHONY: gazelle
gazelle: update-repos
	@bazel run //:gazelle

.PHONY: cargo-repin
cargo-repin:
	@CARGO_BAZEL_REPIN=1 bazel sync --only=crate_index

.PHONY: test
test: gazelle
	@bazel test --test_output=all //...

.PHONY: coverage
coverage: gazelle
	@bazel coverage --combined_report=lcov //...

.PHONY: clean
clean:
	@bazel clean

build: gazelle
	@bazel build //...

.PHONY: build-images
build-images: build
	@bazel run //service/gateway/cmd/server:image -- --norun
	@bazel run //service/ledger/server:image -- --norun
	@bazel run //service/mailer/src/main:image -- --norun

.PHONY: build-ledger
build-ledger:
	@bazel run //service/ledger/server:image -- --norun

.PHONY: format
format:
	@bazel run @rules_rust//:rustfmt
