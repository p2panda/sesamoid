<h1 align="center">p2panda</h1>

<div align="center">
  <strong>All the things a panda needs</strong>
</div>

<br />

<div align="center">
  <!-- CI status -->
  <a href="https://github.com/p2panda/p2panda/actions">
    <img src="https://github.com/p2panda/p2panda/workflows/Build%20and%20test/badge.svg" alt="CI Status" />
  </a>
  <!-- Crates version -->
  <a href="https://crates.io/crates/p2panda-rs">
    <img src="https://img.shields.io/crates/v/p2panda-rs.svg?style=flat-square" alt="Crates.io version" />
  </a>
  <!-- NPM version -->
  <a href="https://crates.io/crates/async-std">
    <img src="https://img.shields.io/npm/v/p2panda-js?style=flat-square" alt="NPM version" />
  </a>
</div>

<div align="center">
  <h3>
    <a href="https://github.com/p2panda/design-document/tree/master/spec">
      Specification
    </a>
    <span> | </span>
    <a href="https://github.com/p2panda/p2panda/releases">
      Releases
    </a>
    <span> | </span>
    <a href="https://github.com/p2panda/design-document#get-involved">
      Contributing
    </a>
  </h3>
</div>

<br/>

This library provides all tools required to write a client for the [`p2panda`] network. It is shipped both as an Rust crate [`p2panda-rs`] with WebAssembly bindings and a NPM package [`p2panda-js`] with TypeScript definitions running in NodeJS or any modern web browser.

[`p2panda`]: https://github.com/p2panda/design-document
[`p2panda-rs`]: https://github.com/p2panda/p2panda/tree/main/p2panda-rs
[`p2panda-js`]: https://github.com/p2panda/p2panda/tree/main/p2panda-js

## Features

- Generate Ed25519 author key pairs
- Create and encode [`bamboo`] entries
- Send messages to [`node`] servers via RPC API calls
- Query and filter data in the network

[`bamboo`]: https://github.com/AljoschaMeyer/bamboo
[`node`]: https://github.com/p2panda/node

## Examples

```js
import p2panda from 'p2panda-js';
const { KeyPair } = await p2panda;
const keyPair = new KeyPair();
console.log(keyPair.publicKey());
```

```rust
use p2panda_rs::KeyPair;
let key_pair = KeyPair::new();
println!("{}", key_pair.publicKey());
```

More examples can be found in the [`p2panda-rs`] and [`p2panda-js`] directories.

## Installation

If you are using `p2panda` in web browsers or NodeJS applications run:

```sh
$ npm i p2panda-js
```

For Rust environments and [cargo-edit](https://github.com/killercup/cargo-edit) installed run:

```sh
$ cargo add p2panda-rs
```

[cargo-add]: https://github.com/killercup/cargo-edit

## License

GNU Affero General Public License v3.0 `AGPL-3.0`
