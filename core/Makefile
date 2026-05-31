test:
	@RUST_BACKTRACE=full cargo test -- --color=always --test-threads=1
	@rm -R /tmp/rust_framework

test_debug:
	@RUST_BACKTRACE=full cargo test -- --color=always --test-threads=1 --nocapture
	@rm -R /tmp/rust_framework

lint:
	@cargo clippy --all-targets

clean:
	@cargo clean
	@rm -Rf target

update:
	@if [[ -n $$(git status --porcelain) ]]; then \
		echo "Cannot update dependencies version on dirty workspace"; \
	else \
		cargo update; \
	fi

build:
	@cargo build