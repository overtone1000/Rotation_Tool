#!/bin/bash

set -e

docker context use autoscheda_production_manager

CONTAINER_NAME=$(docker container ls -q -f name=autoscheda_nginx)
echo "Result is: $CONTAINER_NAME"

if [[ -z "$CONTAINER_NAME" ]]; then
    echo "Container name not found. Got $CONTAINER_NAME"
else
    echo "Copying static site."
    echo "Trying remove."
    echo $(docker exec $CONTAINER_NAME rm -R /var/www/html/static_content/rotations)
    docker cp ./build/ $CONTAINER_NAME:/var/www/html/static_content/rotations/
fi

docker context use default