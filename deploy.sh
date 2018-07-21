#!/bin/bash
echo "$DOCKER_PASSWORD" | docker login -u "$DOCKER_USERNAME" --password-stdin
docker push cderwin/mandelbrot
docker push cderwin/mandelbrot:$(git rev-parse HEAD)
docker oush cderwin/mandelbrot:latest
