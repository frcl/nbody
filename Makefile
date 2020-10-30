simulation:
	cargo build --release
	./target/release/nbody > data.csv
	python test_mpl_ani.py
