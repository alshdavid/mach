build *ARGS:
	cargo build {{ARGS}}

run *ARGS:
	cargo run {{ARGS}}

test:
	@echo "coming soon"

serve-fixture FIXTURE:
	test -d node_modules || pnpm install
	cd fixtures/{{FIXTURE}} && pnpm run serve

build-fixture FIXTURE:
	test -d node_modules || pnpm install
	cd fixtures/{{FIXTURE}} && test -f ./src/index.jsx && cargo run ./src/index.jsx || true
	cd fixtures/{{FIXTURE}} && test -f ./src/index.js && cargo run ./src/index.js || true
