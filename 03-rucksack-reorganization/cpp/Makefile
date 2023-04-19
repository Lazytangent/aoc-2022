CC := g++
BUILD_DIR := target

${BUILD_DIR}/solution: ${BUILD_DIR} solution.cc
	${CC} -Wall -g -o ${BUILD_DIR}/solution solution.cc

.PHONY: build
build: ${BUILD_DIR}/solution

${BUILD_DIR}:
	@mkdir -p ${BUILD_DIR}

.PHONY: small
small: build
	@./target/solution

.PHONY: large
large: build
	@./target/solution full

.PHONY: full
full: large

.PHONY: clean
clean:
	@rm -rf target
