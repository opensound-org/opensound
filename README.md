# opensound

English | [简体中文](README-CN.md)

This is the OpenSound Project's Mono Repo.

[Website](https://opensound.run) | [crates.io](https://crates.io/crates/opensound) | [docs.rs](https://docs.rs/opensound/latest/opensound)

Original Author: [@czy-29](https://github.com/czy-29)

Latest version: [v0.0.6](https://github.com/opensound-org/opensound/releases/tag/v0.0.6)

## What
OpenSound is a (currently WIP and in early development stage) OpenSource One-Stop Multi-Level SoundSystem Abstraction written in [Rust](https://www.rust-lang.org/), suitable for being a solid foundation for [Pro-Audio](https://en.wikipedia.org/wiki/Professional_audio) Applications(e.g. a [DAW](https://en.wikipedia.org/wiki/Digital_audio_workstation)) or other sound related apps.

1.0 will be our [MVP](https://en.wikipedia.org/wiki/Minimum_viable_product) version, and it will consist of:
- The Core Rust API
- A Web API Server
- A bundled Web GUI Playground

The Web GUI Playground is mainly for Exploring & Testing & [Live Coding](https://en.wikipedia.org/wiki/Live_coding) purpose (its form will resemble the combination of [JUCE](https://juce.com/)'s DemoRunner and AudioPluginHost, as well as some of the unique features of this project), but of course, you can use it for music arrangement/production, it's just that the workflow can be quite cumbersome if you use just the playground frontend(e.g. this playground may not have a complete piano roll).

But for now, we are currently focusing on the 0.1 version, which is a [PoC](https://en.wikipedia.org/wiki/Proof_of_concept) version.
The main difference between the PoC version and the MVP version, is that in the PoC version, there will be no Web GUI Playground. Insdead, there will be a command-line "[REPL](https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop) Script Console" for testing & live coding demonstration.

In the future(after MVP), our roadmap will be:
- Wrapping the C API & create bindings for various languages(like C++/Python/C#...)
- Mobile support(1.0 will only support desktop platforms)
- WASM support
- OpenSound Native Playground(using Flutter & opensound's C API)
- Game Audio functionalities & Game Engines(like Unity/Unreal/Godot) integration

## Install
### Binary usage:
If you just want to try this project out, or want to use the pre-built Web API Server directly, you can:
```
cargo install opensound
```
Alternatively, if you do not have Rust installed or do not wish to use `cargo install`, you can download the pre-built binary directly from [Github Releases](https://github.com/opensound-org/opensound/releases) (macOS and Linux versions may require you to run `chmod +x` on the binary before execution).

#### Reproduce pre-built binary in [Github Release](https://github.com/opensound-org/opensound/releases/tag/v0.0.6):

First: `git checkout v0.0.6`

Then, the Windows version can be built directly by executing `cargo build --release` under the MSVC toolchain (which is the default toolchain under Windows machines).

For macOS and Linux versions, in order for the build to run across OS distributions, we used "[cargo-zigbuild](https://crates.io/crates/cargo-zigbuild)". So please refer to their guide first to correctly install cargo-zigbuild (including correctly installing zig and adding Rust targets).

Then the macOS version can be built by running `cargo zigbuild --target universal2-apple-darwin --release` (requires a machine with macOS 11.0 or higher).

The Linux version can be cross-compiled and built on any Windows 10+/macOS 10.12+/Linux (kernel 3.2+, glibc 2.17+) machine by running: `cargo zigbuild --target x86_64-unknown-linux-gnu --release`.

### Library usage:
You can:
```
cargo add opensound
```
if you prefer to use the underlying Rust API.

## Rust Version Policy
As an official, we always use the latest stable version of Rust (currently 1.77.1) to build and test this project. However, any version of Rust that is higher than the [MSRV](https://doc.rust-lang.org/stable/cargo/reference/manifest.html#the-rust-version-field) specified in [Cargo.toml](Cargo.toml) (currently 1.64.0, as it is required for the `cargo-zigbuild`'s `universal2-apple-darwin` target) should be built normally, but the official does not guarantee the correctness of the behavior.

If you encounter some problems after building this project using a version of Rust that is lower than the latest stable version, please try upgrading to the latest stable version of Rust first and then see if the problem still exists. If the problem persists, please initiate an issue at [GitLab](https://gitlab.com/opensound-org/opensound/-/issues) or [Github](https://github.com/opensound-org/opensound/issues).

## Why
1. As you can see, in the C++ ecosystem, there is a one-stop audio development framework like [JUCE](https://juce.com/), as well as a DAW audio engine like [tracktion_engine](https://github.com/Tracktion/tracktion_engine), but they all have various flaws (at least my own experience using them is very poor in many places), and they are C++(🤮). But in the Rust ecosystem, the distribution of audio crates are highly fragmented, lacking a unified solution, and many crates lack good maintenance, so I plan to write one myself. You can think this project as the [RIIR](https://github.com/ansuz/RIIR) version of JUCE + trackion_engine (but not quite, because the API of this project will be very different from theirs, it will be more elegant. At the same time, the API of this project will not include GUI modules, forcing you to practice a more modern architecture with front-end and back-end decoupling and strict isolation).
2. I am developing my own DAW (but DAW itself will be a commercial closed source project). I know that starting a new DAW from scratch in 2024 sounds like a joke, so I plan to fully open source the audio backend (which is this project) without reservation, introducing community power, and work together to open source and create. At the same time, the closed source of DAW front-end can retain commercial space, allowing this project to obtain funding for sustainable development. Therefore, overall speaking, the complete form of this project is actually an "[OpenCore](https://en.wikipedia.org/wiki/Open-core_model)" project. This project is the open source "Core", and the DAW (tentatively named OpenSound Studio) is the closed source part.
3. In order to maximize the adoption of this project, we will wrap a Web API Server at the earliest stage, allowing any developer who is not using Rust to call this project in their own language even before the C API is exposed -- as long as your language can send HTTP/WebSocket requests. This also forms a front-end/back-end "process isolation" architecture, so that anyone can develop their own front-end using any framework, making it easy to develop custom frontends.
4. I am also a semi professional musician myself. In the process of arranging my own music, I discovered many features that I hoped to have, but none of them are available in the current DAWs. This is also the reason why I ultimately decided to write my own DAW. And many features will be implemented on this open-source core, which means that this project will have a lot of innovative audio features. So stay tuned!

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

## Contribution

[GitLab](https://gitlab.com/opensound-org/opensound) is our [single source of truth](https://en.wikipedia.org/wiki/Single_source_of_truth), and the [Github](https://github.com/opensound-org/opensound) version is a read-only mirror, so please do not initiate any pull requests in the Github version.

Merge requests are welcome on our [GitLab](https://gitlab.com/opensound-org/opensound) version!

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in opensound by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
