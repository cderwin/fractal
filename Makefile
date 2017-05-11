project := mandelbrot
service := server

target := x86_64-unknown-linux-musl

registry := registry.camderwin.us
tag := $(shell echo "latest")
image := $(registry)/$(project):$(tag)

run:
	docker-compose run --rm --service-ports $(service)

sh:
	docker-compose run --rm $(service) sh

build:
	cargo build --release --target=$(target) && \
	docker-compose build

deploy:
	docker build . -t $(image)  && \
	docker push $(image)
