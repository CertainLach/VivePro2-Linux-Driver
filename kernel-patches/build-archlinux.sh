#!/usr/bin/env sh

docker build . -f Dockerfile.archlinux-kernel -t proxy-driver-archlinux-kernel
docker run proxy-driver-archlinux-kernel cat kernel/linux.pkg.tar.zst > linux.pkg.tar.zst
docker run proxy-driver-archlinux-kernel cat kernel/headers.pkg.tar.zst > headers.pkg.tar.zst
