#!/bin/sh

# Generate AST, fails, as it also tries to compile
clang -Xclang -ast-dump=json -stdlib=libc++ a.hpp > a.json || true
jrsonnet a.jsonnet > openvr.json
jrsonnet mod_rs.jsonnet -S > mod.rs

rustfmt mod.rs
