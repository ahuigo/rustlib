run:
	cargo run

test_foo:
	#src/unittest/foo_test.rs
	RUST_BACKTRACE=1 cargo test --package myrustlib --bin myrustlib -- spec::func::error::err_backtrace --exact --nocapture
