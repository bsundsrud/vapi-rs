.PHONY: build build-docker run release-build
DOCKER_UID := $(shell id -u)
DOCKER_GID := $(shell id -g)
DOCKER_RUN_ARGS := --rm -v "$(PWD):/code" -v "$(HOME)/.cargo/registry:/cargo/registry"
DOCKER_TAG := "vapi-rs-build:latest"
build-docker:
	podman build $(DOCKER_BUILD_ARGS) -f Dockerfile.build -t $(DOCKER_TAG) .
build: build-docker
	podman run $(DOCKER_RUN_ARGS) $(DOCKER_TAG) build
run: build-docker
	podman run --entrypoint /bin/bash -it $(DOCKER_RUN_ARGS) $(DOCKER_TAG) 

release-build: build-docker
	podman run $(DOCKER_RUN_ARGS) $(DOCKER_TAG) build --release

get-version:
	@cat vapi-logger/Cargo.toml | grep '^version' | sed 's/^version\s*=\s*"\(.\+\)"/\1/'
