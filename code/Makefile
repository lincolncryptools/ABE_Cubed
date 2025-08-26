test:
	cargo test opt0
	cargo test opt1
	cargo test opt2
	cargo test opt3
	cargo test opt4 --features "decrypt_mode_a"
	cargo test opt4 --features "decrypt_mode_b"
	cargo test opt5 --features "decrypt_mode_a"
	cargo test opt5 --features "decrypt_mode_b"
	cargo test opt6 --features "decrypt_mode_aa"
	cargo test opt6 --features "decrypt_mode_ab"
	cargo test opt6 --features "decrypt_mode_ba"
	cargo test opt6 --features "decrypt_mode_bb"

bench_01a:
	cargo bench --bench strat_01a -- --noplot --output-format bencher | tee strat_01a_out.txt

bench_01b:
	cargo bench --bench strat_01b -- --noplot --output-format bencher | tee strat_01b_out.txt

bench_02:
	cargo bench --bench strat_02 -- --noplot --output-format bencher | tee strat_02_out.txt

bench: bench_01a bench_01b bench_02

run:
	cargo run abe_cubed

clean:
	cargo clean

format:
	cargo fmt

.PHONY: test bench_01a bench_01b bench_02 run clean format
