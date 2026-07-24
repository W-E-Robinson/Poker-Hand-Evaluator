test:
	cargo nextest run --no-fail-fast

test-lib:
	cargo nextest run --lib --no-fail-fast

test-server:
	cargo nextest run --bin server --no-fail-fast

audit:
	cargo audit

outdate:
	cargo outdated
