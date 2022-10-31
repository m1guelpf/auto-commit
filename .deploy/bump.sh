#!/bin/bash

# This script is used to bump the version in the PKGBUILD file.

version=$(git describe --long --tags | sed 's/^v//;s/\([^-]*-g\)/r\1/;s/-/./g')

sed -i "s/pkgver=/pkgver=$version/" PKGBUILD
