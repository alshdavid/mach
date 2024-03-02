build *ARGS:
	cargo build {{ARGS}}

run *ARGS:
	cargo run -- {{ARGS}}

fmt:
  cargo +nightly fmt

test:
	@echo "coming soon"

serve-fixture FIXTURE:
	test -d node_modules || pnpm install
	cd fixtures/{{FIXTURE}} && pnpm run serve

build-fixture FIXTURE *ARGS:
	test -d node_modules || pnpm install
	cd fixtures/{{FIXTURE}} && test -f ./src/index.jsx && cargo run --bin mach -- ./src/index.jsx {{ARGS}} || true
	cd fixtures/{{FIXTURE}} && test -f ./src/index.js && cargo run --bin mach -- ./src/index.js {{ARGS}} || true
