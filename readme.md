# VowpalWabbit-rs

[![build](https://github.com/jackgerrits/vowpalwabbit-rs/workflows/build/badge.svg?branch=master)](https://github.com/jackgerrits/vowpalwabbit-rs/actions?query=workflow%3Abuild)

This repo mainly provides rust bindings to [VowpalWabbit (VW)](https://github.com/VowpalWabbit/vowpal_wabbit). It replaces the [sys crate](https://github.com/jackgerrits/vowpalwabbit-sys-rs) which I originally created. The sys crate tried wrapping the existing C bindings of VW, however they have issues such as not providing an error reporting mechanism. Because of the lack of a stable and functional C API, creating and maintaining a sys crate did not seem worth the effort. This crate bundles the source of VW and its dependencies for a standalone crate build (CMake and a C++11 compiler must be available) so the user need not worry about the native code. It includes a C API wrapper around VW which exposes the functionality necessary for this crate. The C API is wrapped in Rust code that is hopefully more or less idiomatic.

Currently it is only tested on Ubuntu 20.04 with Clang 10 and GCC 9.

Because of the tightly coupled C API implementation and in package source bundling each version of this crate will target a specific VW version described in the table below:

| Crate version | VW version | VW commit |
|---|---|---|
|...|...|...|
