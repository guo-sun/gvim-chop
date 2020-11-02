# TODO
- [ ] namespace before making an actual plugin (myplugin#func)

# Learnings

* Need to use 32-bit Rust to build the Rust library:

> rustup install stable-i686-pc-windows-gnu
> cargo +stable-i686-pc-windows-gnu build

* Rust library needs to be:
[lib]
crate-type = ["cdylib"]
