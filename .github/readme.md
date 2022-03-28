# lua-sys

Unsafe Rust Lua bindings, to be consumed by the Worldserver. The bindings were created using bindgen.

The currently targeted Lua version is 5.4.4 only.

## Creating the bindings

The bindings are generated using the following command.

```sh
bindgen lua.hpp -o bindings.rs --no-layout-tests --size_t-is-usize --default-macro-constant-type signed
```
