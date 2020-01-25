RUST_BACKTRACE=1 RUST_LOG=debug cargo run run ls:ll
RUST_BACKTRACE=1 RUST_LOG=debug cargo run run pwd
RUST_BACKTRACE=1 RUST_LOG=debug cargo run run pwd:fail

RUST_BACKTRACE=1 RUST_LOG=debug cargo run run ls:f first second


RUST_BACKTRACE=1 RUST_LOG=debug cargo run run ls:f shiftcare Movies