# {{ cookiecutter.plugin_name }}

## Building

After installing [Rust](https://rustup.rs/), you can compile {{ cookiecutter.plugin_name }} as follows:

```shell
cargo xtask bundle {{ cookiecutter.project_name }} --release
```
