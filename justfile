build *ARGS:
	cargo build {{ARGS}}

run *ARGS:
	cargo run {{ARGS}}

fmt:
  cargo +nightly fmt

test:
	@echo "coming soon"

serve-fixture FIXTURE:
	test -d node_modules || pnpm install
	test -f testing/fixtures/{{FIXTURE}} || echo "Does not exist" && exit 1
	cd testing/fixtures/{{FIXTURE}} && pnpm run serve

build-fixture FIXTURE *ARGS:
	test -d node_modules || pnpm install
	if [ ! -d "testing/fixtures/{{FIXTURE}}" ]; then \
		echo "Does not exist"; \
		exit 1; \
	fi; 
	cd testing/fixtures/{{FIXTURE}} && test -f ./src/index.jsx && cargo run --bin mach -- ./src/index.jsx {{ARGS}} || true
	cd testing/fixtures/{{FIXTURE}} && test -f ./src/index.js && cargo run --bin mach -- ./src/index.js {{ARGS}} || true
	cd testing/fixtures/{{FIXTURE}} && test -f ./src/index.tsx && cargo run --bin mach -- ./src/index.tsx {{ARGS}} || true
	cd testing/fixtures/{{FIXTURE}} && test -f ./src/index.ts && cargo run --bin mach -- ./src/index.ts {{ARGS}} || true
