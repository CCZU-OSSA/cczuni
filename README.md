<div align=center>
  <img width=200 src="doc\logo.png"  alt="logo"/>
  <h1 align="center">CCZUNI</h1>
</div>

<div align=center>
    <img src="https://img.shields.io/badge/Rust-2024-brown" alt="Rust">
    <img src="https://img.shields.io/github/languages/code-size/CCZU-OSSA/cczuni?color=green" alt="size">
</div>


## Usage

```sh
cargo add --git https://github.com/CCZU-OSSA/cczuni.git
```
## Features

```rust
todo!()
```

## What's the Difference to `CCZU-Client-API`?

### Thread Safe

In `CCZU-Client-API`, we used this code to impl `Sync` and `Send`.

```rust
unsafe impl Send for #ident {}
unsafe impl Sync for #ident {}
```

There are some risks to use with it.

In `cczuni`, we use `Arc` and `tokio::sync` to ensure the thread safe.

### Faster Speed & Smaller Size

With less unnecessary clone, new...

`cczuni` is faster and smaller!

### Flexible Trait.

In `cczuni`, most traits are impl for generic `Client`.

We provide a default `crate::impls::client::DefaultClient`, If you want to custom a `Client`, just impl `Client`.

## Docs

```rust
todo!()
```