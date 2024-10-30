#!/bin/bash

set -e

cd "$(dirname "$0")"

source server.sh

CONTAINER_NAME=$(podman container ls -q -f name=nginx)
echo "Result is: $CONTAINER_NAME"

if [[ -z "$CONTAINER_NAME" ]]; then
    echo "Container name not found. Got $CONTAINER_NAME"
else
    echo "Trying remove."
    podman exec "$CONTAINER_NAME" mkdir -p /var/www/html/static_content/rotations
    podman exec "$CONTAINER_NAME" rm -fR /var/www/html/static_content/rotations/*
    echo "Copying static site."
    podman cp ../frontend/build/. "$CONTAINER_NAME":/var/www/html/static_content/rotations/
fi