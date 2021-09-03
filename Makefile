.PHONY: build build-docker run release-build
DOCKER_BUILD_ARGS := --build-arg UID=$(shell id -u) --build-arg GID=$(shell id -g) --build-arg UNAME=$(shell whoami)
DOCKER_RUN_ARGS := -v "$(PWD):/home/$(shell whoami)/code"
DOCKER_TAG := "vapi-rs:build"
build-docker:
	docker build $(DOCKER_BUILD_ARGS) -f Dockerfile.build -t $(DOCKER_TAG) .
build: build-docker
	docker run $(DOCKER_RUN_ARGS) $(DOCKER_TAG)
run: build-docker
	docker run -it $(DOCKER_RUN_ARGS) $(DOCKER_TAG) /bin/bash

release-build: build-docker
	docker run $(DOCKER_RUN_ARGS) $(DOCKER_TAG) build --release

get-version:
	@cat vapi-logger/Cargo.toml | grep '^version' | sed 's/^version\s*=\s*"\(.\+\)"/\1/'
