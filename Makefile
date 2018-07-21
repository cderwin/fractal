project := mandelbrot
service := server

target := x86_64-unknown-linux-musl
executable = target/$(target)/release/$(project)
src := $(wildcard src/*.rs)

image_name := $(project)
build_image_name := $(image_name)-build
build_container_name := $(build_image_name)

registry := registry.camderwin.us
tag := $(shell echo "latest")
image := $(registry)/$(project):$(tag)

.ts.buildcontainer: BuildDockerfile
	docker build -t $(build_image_name) -f BuildDockerfile . && \
		touch $@

$(executable): $(src) Cargo.toml Cargo.lock .ts.buildcontainer
	if [ ! $$(docker ps -q -f "name=$(buld_container_name)") ]; then \
		docker run --rm -ti -d \
			--name $(build_container_name) \
			-v $$(pwd):/mandelbrot \
			-w /mandelbrot \
			$(build_image_name); \
	fi && \
	docker exec -it \
		-w /mandelbrot \
		$(build_container_name) \
		/root/.cargo/bin/cargo build --release --target=$(target)

.ts.container: $(executable)
	docker build -t $(image_name) . && \
		touch $@

.PHONY: run sh build deploy end

run: .ts.container
	docker run --rm \
		-v $$(pwd)/static:/static \
		-p 8000:8000 \
		$(image_name)

sh: $(executable)
	docker run --rm -it \
		-v $$(pwd)/static:/static \
		-v $$(pwd):/code \
		-p 8000:8000 \
		$(image_name) \
		bash

sh_build:
	docker exec -it \
		-w /mandelbrot \
		$(build_container_name) \
		bash

end:
	docker stop $(build_container_name)

build: .ts.container

deploy: $(executable)
	docker build . -t $(image)  && \
	docker push $(image)
