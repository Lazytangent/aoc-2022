CC := gcc
BUILD_DIR := target

.PHONY: build
build: ${BUILD_DIR}
	${CC} main.c -o ${BUILD_DIR}/main

${BUILD_DIR}:
	@mkdir -p ${BUILD_DIR}

.PHONY: small
small: build
	@./target/main

.PHONY: large
large: build
	@/.target/main large

.PHONY: full
full: large
