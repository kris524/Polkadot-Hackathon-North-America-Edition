# Polkadot Hackathon 2022

We are doing another Polkadot hackathon.
It has been a while since I last hacked on Substrate.
A lot has changed,
with many Substrate networks having live networks.
Ink has had another year to mature.
I am excited to see what the experience is like now.

This time it's not just me and [Aimee].
[@kris524] and [@domroselli],
from the [Rust in Blockchain][rib] group are joining us.
I am nervous about working on a team again,
but hope to remember to just have fun and not stress out.

[Aimee]: https://github.com/Aimeedeer
[@kris524]: https://github.com/kris524
[@domroselli]: https://github.com/domroselli
[rib]: https://rustinblockchain.org/

For the sake of learning I have suggested a (conceptually) simple project:
translate [Uniswap v2] to [Ink] and run it on [Astar Network][ast],
which supports contracts compiled to WASM.

[Uniswap v2]: https://github.com/Uniswap/v2-core/tree/master/contracts/interfaces
[Ink]: https://github.com/paritytech/ink
[ast]: https://docs.astar.network/


## Things we learned

todo

### Solidity concepts mapped to Ink

- `address` -> [`AccountId`]
- `bytes` -> `Vec<u8>`
- `bytes32` -> ?
- `external` -> `pub`
- `view` -> `&self` recievers
- `pure` -> no direct mapping to Ink traits at least. Probably should be associated consts.
- `mapping` -> [`ink_storage::Mapping`]
- `private` ->
- `internal`
- `uint` -> ?
- `uint112` -> ?

More:

- Large integers types can be had in the [`ethereum_types`] crate.
  It is not tied to Ethereum specifically.
- Solidity safe math libraries aren't need in Rust &mdash;
  Rust has checked arithmetic functios on all its types,
  as does the `ethereum_types` crate's.
- Rust doesn't panic on overflow for primitive types by default in release mode,
  but this can and probably should be turned on for smart contracts by setting
  `overflow-checks = true` in the `[profile.dev]` section of the manifest.

[`ethereum_types`]: https://docs.rs/ethereum_types


## Being a manager again (2022/05/30)

It's day 1 of the hackathon.

I fear managing people,
and wasn't exactly looking to do that for this project,
but it is a natural role on this project,
as I take it everybody else is much less experienced (at least with Rust),
so I am trying to gently create a path forward for the other
people on the team,
without taking on too much responsibility or pushing people around.

[I filed an issue][tasks] outlining plausible tasks for the first two weeks,
that 4 people might be able to divide between themselves.

[tasks]: https://github.com/kris524/Polkadot-Hackathon-North-America-Edition/issues/1

The big thing we need to accomplish is to unblock everybody,
so each individual can hack on something.
To that end I asked Kris to create a `swap_traits` crate,
where we will start defining the uniswap interfaces as Ink traits.
Once we have those traits in place people can work on implementing them,
deploying them,
calling them, etc.
There should be things to hack on then.


## `dylint-link` doesn't understand `mold` (2022/05/30)

Before I can get started I have to install [`cargo-contract`].
running `cargo install cargo-contract` though fails with an error
indicating I need to install [`dylint-link`], which appears
to be a custom linting tool of some kind.

`dylint-link` is also installed via `cargo` with
`cargo install dylint-link cargo-dylint`,
and that worked fine.
But I still can't build `cargo-contract`.

[`cargo-contract`]: https://github.com/paritytech/cargo-contract
[`dylint-link`]: https://github.com/trailofbits/dylint

I see this error:

```
     Compiling memchr v2.4.1
  error: linking with `dylint-link` failed: exit status: 1
  ...
    = note: cc: error: unrecognized command line option '-fuse-ld=/usr/local/bin/mold'
```

I am using [`mold`] as my linker,
configured by putting the following in `~/.cargo/config.toml`:

```toml
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/mold"]
```

It appears that `dylint-link` is a drop-in `cc` wrapper,
but does not understand the `-fuse-ld` flag.

Can I fix `dylint-link` to take this flag?

Let's find out.

After some investigation,
the problem isn't that `dylint-link` doesn't understand `-fuse-ld`,
it's that my cargo config says I want to link with `clang`,
which understands `-fuse-ld`,
but `dylint-link` passes linking duties to `cc`,
ignoring my cargo config.

After poking around for a bit,
it's not immediately obvious how to change `dylint-link` to handle this,
so I just [file an issue][dllissue].

[dllissue]: https://github.com/trailofbits/dylint/issues/337

In the meantime I temporarily disable my global cargo configuration to finish
the `cargo-contract` install.


## Getting the skeleton of our workspace in place (2022/05/31)

Today we're just focused on getting our cargo workplace in place,
and a `swap_traits` crate set up with its ink dependencies,
so we can start writing the uniswap interfaces in ink.

So far I'm personally just doing code review,
and interacting with the `dylint-link` maintainers about the bug I ran into,
but once we've got `swap_traits` in place I'll start doing my part hacking too.


## Figuring out how to build Ink contracts (2022/06/02)

I discover that the recommended way to install binaryen on my system
(`apt get install binaryen`) yields a binaryen that the toolchain rejects:

```
ERROR: Your wasm-opt version is 91, but we require a version >= 99.

If you tried installing from your system package manager the best
way forward is to download a recent binary release directly:

https://github.com/WebAssembly/binaryen/releases

Make sure that the `wasm-opt` file from that release is in your `PATH`.
```

I ended up downloading it directly from the Binaryen [releases][br],
untarring it, and putting its `bin` directory in my `PATH` by modifying my `.bashrc`.
Not the best onboarding experience.
A lot of newbies would not figure this out.

[br]: https://github.com/WebAssembly/binaryen/releases

`cargo-contract` [doesn't understand cargo workspaces][ws],
and this appears to mean that every ink crate needs to be built
with its own command, so I documented that too.

[ws]: https://github.com/paritytech/cargo-contract/issues/182

`cargo-contract` requires a nightly Rust,
and one that has the non-default `rust-src` package installed,
so I set up a `rust-toolchain.toml` file in the root of our project:

```toml
[toolchain]
channel = "nightly-2022-06-02"
components = [ "rust-src" ]
```

After about an hour and a half of setting up dependencies
I have a working build of our `swap_traits` crate,
which as of now is just the Ink example `flipper` contract.

Ink has quite a few dependencies that need to be installed individually:
`cargo-contract`, `cargo-dylib`, `dylint-link`, Binaryen;
so I added instructions to our readme file.

Kris is still trying to figure out the equivalent of an Ethereum "address" in Ink
so he can write the `IUniswapV2Callee` trait.
We think it is probably the [`AccountId`] type.

I worry that by starting with implementing Ink traits,
that we've bypassed a bunch of steps for learning Ink.


## Setting up a Svelte webapp (2022/06/03)

Today I am determined to get a web frontend in place,
just the basic npm compilation,
mabe a sketch of the components.
I am going to use Svelte,
as I have been learning it lately.


## Slow progress (2022/06/04)

Trail of Bits [published a new version of `dylint-link`][dllp]
to fix my earlier problem.
I verified their fix works for me.
Feels good.

[dllp]: https://github.com/trailofbits/dylint/issues/337#issuecomment-1145482930

Kris is still getting up to speed on both Rust and ink trait definitions.
I continue to review his PRs and nudge him in the right direction.
Hopefully we'll have the trait definitions in a reasonable state in a few days.


## Finally progress (2022/06/08)

I accidentally deleted several days of journal here.
That is frustrating.

In the meantime though,
Kris finished outlining most of the Uniswap traits,
and I took a pass over them to clean them up.

Aimee has written an implementation of `UniswapV2ERC20`.

At the moment we are proceeding as if ERC20 is the token standard we need to use,
but I suspect that is not true &mdash; this isn't the EVM.
We probably need to use [PSP-22]

[PSP-22]: https://github.com/w3f/PSPs/blob/master/PSPs/psp-22.md

We are so far confused about how to map various Solidity types to Ink,
including:

- `bytes32` - this is _up to_ 32 bytes in Solidity, so kind of like a `Vec<u8>`,
  but with a bound on its length.
- `uint` - this is just u 256-bit integer, but we haven't found the Ink equivalent yet.
- `uint112` - this is a 112-bit integer! Unusual.

Right now we are using a lot of `u64`s where we should be using larger types.

Kris has moved on to stubbing out `UniswapV2Factory`.
This seems to me the "main" uniswap contract.
My near-term goal is to figure out how to deploy this contract to a devnet
so that we can write JS and Rust client code that drives it,
get some end-to-end connectivity working.

The details about what the contracts actually do doesn't matter yet.


## A difficult compile-time problem to resolve (2022/06/08)

During testing Aimee ran into a compilation problem that would be
very difficult for many Rust programmers to solve:

Crates that use ink have fairly complicated manifests.
They look like this:

```toml
[package]
name = "uniswap_v2_erc20"
version = "0.1.0"
authors = ["[your_name] <[your_email]>"]
edition = "2021"

[dependencies]
ink_primitives = { version = "3", default-features = false }
ink_metadata = { version = "3", default-features = false, features = ["derive"], optional = true }
ink_env = { version = "3", default-features = false }
ink_storage = { version = "3", default-features = false }
ink_lang = { version = "3", default-features = false }
ink_prelude = { version = "3", default-features = false}
scale = { package = "parity-scale-codec", version = "3", default-features = false, features = ["derive"] }
scale-info = { version = "2", default-features = false, features = ["derive"], optional = true }

[lib]
name = "uniswap_v2_erc20"
path = "lib.rs"
crate-type = [
    "cdylib",
]

[features]
default = ["std"]
std = [
    "ink_metadata/std",
    "ink_env/std",
    "ink_storage/std",
    "ink_primitives/std",
    "scale/std",
    "scale-info/std",
]
ink-as-dependency = []
```

They are expected to follow this feature pattern:
default "std", "std" enables "std" recursively,
and there's this mystery "ink-as-dependency" feature that I don't understand yet.
When `cargo-contract` builds your crate for wasm it will seemingly disable the default features.
The "std" feature is for unit testing in userspace.

We have a crate full of ink traits, `swap_traits`, and our contracts link to it,
so our contracts need to keep these "std" features linked up with the `swap_traits` crate.
Aimee did not do this and got many errors like

```
error[E0433]: failed to resolve: use of undeclared crate or module `ink_engine`
  --> /Users/aimeez/.cargo/registry/src/github.com-1ecc6299db9ec823/ink_env-3.2.0/src/engine/off_chain/impls.rs:43:5
   |
43 | use ink_engine::{
   |     ^^^^^^^^^^ use of undeclared crate or module `ink_engine`
```

This is super mysterious &mdash;
the error is occurring in a crates.io dependency.
_Usually_ this means there's a bug in that dependency,
and I would be inclined to look to version pinning to fix the problem.
But in this case I saw that `off_chain` path that was triggering the error
and had the intuition that it was related to the two different modes
that the ink crate stack can be compiled in,
guessed that the cargo features weren't configured correctly.

This would be tough to figure out without a magic combination of knowledge and
intuition.


## Solidity math helpers aren't needed in Rust (2022/06/21)

The uniswap codebase includes a safe math library that
includes basic arithmetic functions that error on overflow.
Rust doesn't need these since its integers have checked arithmetic methods.

We discovered that we can get large integer types from the [`ethereum_types`] crate.
This crate is not Ethereum specific.
The types in this crate panic on overflow by default,
and also include checked arithmetic methods.

I have realized that probably all Rust smart contract projects,
and maybe just all Rust projects,
should put this in their manifest:

```toml
[profile.release]
overflow-checks = true
```

In retrospect I think it was a mistake for Rust to not do overflow checks by default &mdash;
it's a best practice now to only use Rust arithmetic operators when overflow is obviously not possible,
and instead use arithmetic methods appropriate for each specific operation.


## Svelte and Tailwind are super fun (2022/06/22)

My job on this project is implementing the frontend,
and I am taking the opportunity to learn both Svelte and Tailwind,
and liking both a lot.

Getting them both working together with `rollup` took some frustrating effort:
`rollup`'s "watch" mode kept rebuilding the project infinitely.
I found some other people running into this problem but no solutions.
Eventually I figured out that my `tailwind.css.js` configuration
was seemingly causing postcss to touch files in my `public/build`
directory, causing `rollup` to rebuild, etc.

My original config, as suggested on the internet, was:

```JavaScript
module.exports = {
  content: [
    "./public/**/*.html",
    "./src/**/*.svelte",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
```

I changed it to

```JavaScript
module.exports = {
  content: [
    "./src/**/*.svelte",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
```

Svelte is fun,
but getting everything properly reactive feels a bit fiddly.
There's a lot to learn.

Tailwind is a pure productivity booster.
It's as fun as people say.

I am having fun doing frontend development.
