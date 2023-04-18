CC := gcc
BUILD_DIR := target

.PHONY: build
build: ${BUILD_DIR}
	${CC} main.c -o target/main

${BUILD_DIR}:
	@mkdir -p ${BUILD_DIR}

.PHONY: small
small: build
	@./target/main

.PHONY: full
full: build
	@./target/main full

.PHONY: large
large: full
