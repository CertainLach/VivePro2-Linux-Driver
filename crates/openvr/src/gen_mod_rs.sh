#!/bin/sh

# Generate AST, fails, as it also tries to compile
clang -Xclang -ast-dump=json -stdlib=libc++ a.hpp > a.json || true
~/build/jrsonnet/target/release/jrsonnet a.jsonnet > openvr.json
~/build/jrsonnet/target/release/jrsonnet mod_rs.jsonnet -S > lib.rs

rustfmt lib.rs
