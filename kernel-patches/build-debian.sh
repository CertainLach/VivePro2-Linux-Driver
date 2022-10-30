#!/usr/bin/env sh

docker build . -f Dockerfile.debian-kernel -t proxy-driver-debian-kernel
docker run proxy-driver-debian-kernel cat linux.deb > linux.deb
docker run proxy-driver-debian-kernel cat headers.deb > headers.deb
