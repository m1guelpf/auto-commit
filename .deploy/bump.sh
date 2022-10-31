#!/bin/bash

# Bump the version number in the package.json file
version=$(git describe --tags | sed 's/^v//;s/\([^-]*-g\)/r\1/')

# Update the version in the PKGBUILD file.
sed -i "s/pkgver=.*/pkgver=$version/" PKGBUILD

# Replace the version in the Cargo.toml file with the $version variable
sed -i "0,/version = \".*\"/s//version = \"$version\"/" Cargo.toml
