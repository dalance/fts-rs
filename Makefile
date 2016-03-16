export RUST_TEST_THREADS=1

.PHONY: default run bench doc test clean watch_run watch_bench watch_doc watch_test

default: watch_test

run:
	cargo run

bench:
	cargo bench

doc:
	cargo doc --no-deps

test:
	cargo test -- --nocapture

clean:
	cargo clean

watch_run:
	while inotifywait -e modify ./src; do cargo run; done

watch_bench:
	while inotifywait -e modify ./src; do cargo bench; done

watch_doc:
	while inotifywait -e modify ./src; do cargo test; cargo doc --no-deps; done

watch_test:
	while inotifywait -e modify ./src; do cargo test -- --nocapture; done

