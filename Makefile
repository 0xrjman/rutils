.PHONY: run
run:
	python py/main.py

.PHONY: dev
build:
	maturin develop