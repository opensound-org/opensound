<div align="center">

# opensound

English | [简体中文](README-CN.md)

This is the OpenSound Project's [Monorepo](https://en.wikipedia.org/wiki/Monorepo).

[Website](https://opensound.run) | [crates.io](https://crates.io/crates/opensound) | [docs.rs](https://docs.rs/opensound/latest/opensound)

Original Author: [@czy-29](https://github.com/czy-29)

Latest version: [v0.0.6](https://github.com/opensound-org/opensound/releases/tag/v0.0.6)

![Crates.io Total Downloads](https://img.shields.io/crates/d/opensound)
![Crates.io Dependents](https://img.shields.io/crates/dependents/opensound?link=https%3A%2F%2Fcrates.io%2Fcrates%2Fopensound%2Freverse_dependencies)
![GitHub Repo stars](https://img.shields.io/github/stars/opensound-org/opensound)

![MSRV (version)](https://img.shields.io/crates/msrv/opensound/0.0.6?label=v0.0.6-msrv)
[![dependency status (version)](https://deps.rs/crate/opensound/0.0.6/status.svg?subject=v0.0.6-deps)](https://deps.rs/crate/opensound/0.0.6)

![MSRV (git)](https://img.shields.io/badge/git--msrv-1.76-blue)
[![dependency status (git)](https://deps.rs/repo/github/opensound-org/opensound/status.svg?subject=git-deps)](https://deps.rs/repo/github/opensound-org/opensound)

![Static Badge](https://img.shields.io/badge/build_with-Rust_1.81.0-dca282?link=https%3A%2F%2Fblog.rust-lang.org%2F2024%2F09%2F05%2FRust-1.81.0.html)

</div>

## What
OpenSound is a (currently WIP and in early development stage) OpenSource One-Stop Multi-Level SoundSystem Abstraction (or say sound/audio engine) written in [Rust](https://www.rust-lang.org/). Suitable for being a solid foundation for [Pro-Audio](https://en.wikipedia.org/wiki/Professional_audio) Applications(e.g. a [DAW](https://en.wikipedia.org/wiki/Digital_audio_workstation)) or other sound related apps.

"SoundSystem" can basically refer to all software systems related to sound, from simple audio players to complex DAWs, all of which can be easily implemented using this project. "Multi-Level" means that from high-level DAW workflows to low-level abstraction of operating system audio APIs, this project will provide encapsulation and modeling. "One-Stop" means that all the above capabilities are available "out-of-the-box" and do not require you to combine any external dependencies to implement them yourself.

1.0 will be our [MVP](https://en.wikipedia.org/wiki/Minimum_viable_product) version, and it will consist of:
- The Modular Core Rust API
- A plugin based desktop application development framework
- A customizable Web API Server (built using both of the above)
- A bundled Web GUI Playground

The Core Rust API is more like modular "building blocks", while the higher-level application framework allows you to easily and gracefully "assemble" the building blocks into a complete application with a plugin-like architecture. And the Web API Server itself can not only be directly used for the development of sound applications, but also serve as a good example of the usage of the above two parts.

And the Web GUI Playground is mainly for Exploring & Testing & [Live Coding](https://en.wikipedia.org/wiki/Live_coding) purpose (its form will resemble the combination of [JUCE](https://juce.com/)'s DemoRunner and AudioPluginHost, as well as some of the unique features of this project), but of course, you can use it for music arrangement/production, it's just that the workflow can be quite cumbersome if you use just the playground frontend(e.g. this playground may not have a complete piano roll). At the same time, although the built-in Web GUI Playground front-end is not a complete DAW experience, the underlying Web API Server should be available as a complete backend for a DAW.

But for now, we are currently focusing on the 0.1 version, which is a [PoC](https://en.wikipedia.org/wiki/Proof_of_concept) version.
The main difference between the PoC version and the MVP version, is that in the PoC version, there will be no Web GUI Playground. Instead, there will be a command-line "[REPL](https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop) Script Console" for testing & live coding demonstration.

There will be several v0.0.x versions before v0.1, and every time a small feature is implemented, a new version will be bumped.

In the future(after MVP), our roadmap will be:
- Wrapping the C API & create bindings for various languages(like C++/Python/C#...)
- Mobile support(1.0 will only support desktop platforms)
- WASM support
- OpenSound Native Playground(using Flutter & opensound's C API)
- Audio plugin development framework (like VST/VST3/AU/CLAP)
- Game Audio functionalities & Audio engine (like Wwise/FMod) plugin development & Game Engines(like Unity/Unreal/Godot) integration
- (Maybe) Embedded device support

## Etymology
The "Open" of OpenSound is the "Open" of "[Open Source](https://en.wikipedia.org/wiki/Open_source)" (similar to the naming of "[OpenCV](https://opencv.org/)"), and it is also the "Open" of "[Open Standard](https://en.wikipedia.org/wiki/Open_standard)" (similar to the naming of "[OpenAPI](https://www.openapis.org/)"). At the same time, our open source is fundamentalist open source, which is [open source](https://opensource.org/osd) that meets the [OSI](https://opensource.org/) definition.

In addition, the abbreviation of "OpenSound" is "OS", which is the same as the abbreviation of "operating system". This is intentional design, and many concepts of the project will also be borrowed from the operating system, and this also reflects the ultimate goal of the project, which is to become a "sound" operating system!

## Install
### Binary usage:
If you just want to try this project out, or want to use the pre-built Web API Server directly, you can:
```
cargo install opensound
```
Alternatively, if you do not have Rust installed or do not wish to use `cargo install`, you can download the pre-built binary directly from [Github Releases](https://github.com/opensound-org/opensound/releases) (macOS and Linux versions may require you to run `chmod +x` on the binary before execution).

#### Reproduce pre-built binary in [Github Release](https://github.com/opensound-org/opensound/releases/tag/v0.0.6):

At present, the entire release process is purely manual, but in the future, we plan to use Github Actions to automate the entire process.

The following steps describe the method of manually building binaries in Github Release:
- First: `git checkout v0.0.6`
- Then, the Windows version can be built directly by executing `cargo build --release` under the MSVC toolchain (which is the default toolchain under Windows machines).
- For macOS and Linux versions, in order for the build to run across OS distributions, we used "[cargo-zigbuild](https://crates.io/crates/cargo-zigbuild)". So please refer to their guide first to correctly install cargo-zigbuild (including correctly installing zig and adding Rust targets).
- Then the macOS version can be built by running `cargo zigbuild --target universal2-apple-darwin --release` (requires a machine with macOS 11.0 or higher).
- The Linux version can be cross-compiled and built on any Windows 10+/macOS 10.12+/Linux (kernel 3.2+, glibc 2.17+) machine by running: `cargo zigbuild --target x86_64-unknown-linux-gnu --release`.

### Library usage:
You can:
```
cargo add opensound
```
if you prefer to use the underlying Rust API.

## Branch Model
This project adopts a "[centralized workflow](https://git-scm.com/book/en/v2/Distributed-Git-Distributed-Workflows)", which means there is only one single "main" branch, and all work is carried out on this branch. When each version is released, a tag for the current version will be created, and you can checkout a specific tag to obtain the complete code for the corresponding version. If you find a bug in a historical version, you need to first upgrade to the latest version to check if the bug still exists. If it still exists, please initiate an issue at [GitLab](https://gitlab.com/opensound-org/opensound/-/issues) or [Github](https://github.com/opensound-org/opensound/issues) and wait for the next version to fix it. We do not currently provide [hotfix](https://en.wikipedia.org/wiki/Hotfix) support for historical versions (due to limited human bandwidth).

The version release of this project does not have a fixed cycle, but the version number will follow [SemVer](https://semver.org/): after v0.1, each revised version will only contain bug fixes, each minor version will contain minor feature updates, and each major version will contain major feature updates (but before reaching the goal of v0.1 PoC, all small feature updates will only increase the last digit of the version number, which is the revision).

## Rust Version Policy
As an official, we always use the latest stable version of Rust (currently 1.78.0) to build and test this project. However, any version of Rust that is higher than the [MSRV](https://doc.rust-lang.org/stable/cargo/reference/manifest.html#the-rust-version-field) specified in [Cargo.toml](Cargo.toml) (currently 1.76.0, as the MSRV of "[salvo](https://crates.io/crates/salvo)" with the highest MSRV among dependencies is 1.76.0) should be built normally, but the official does not guarantee the correctness of the behavior.

If you encounter some problems after building this project using a version of Rust that is lower than the latest stable version, please try upgrading to the latest stable version of Rust first and then see if the problem still exists. If the problem persists, please initiate an issue at [GitLab](https://gitlab.com/opensound-org/opensound/-/issues) or [Github](https://github.com/opensound-org/opensound/issues).

## Platform Support Policy
We officially maintain the build of three targets:
- x86_64-pc-windows-msvc
- [universal2-apple-darwin](https://crates.io/crates/cargo-zigbuild) (a combination of `x86_64-apple-darwin` and `aarch64-apple-darwin`)
- x86_64-unknown-linux-gnu

Other targets may also work, but we do not guarantee it. And we guarantee that all three targets can be built successfully for all versions of this project, but there may be differences in the level of support for their testing (also due to limited resources and human bandwidth).

At the same time, in the v0.0.x stage, not all features will behave consistently on all platforms. We will have development priorities for platform-specific features, and some versions will only focus on the development of specific features for specific platforms. For specific version information, please refer to the Release Note or CHANGELOG of each version. But after v0.1, we will try our best to ensure the consistency of behavior across all platforms.

Next we'll explain support for testing and platform-specific bug fixes:

Firstly, we only have two Windows 10 machines (one Surface Laptop Studio and one DELL workstation) and one macOS Sonoma machine (2023 MacMini, M2 chip), so testing on the Linux platform can only be conducted on virtual machines temporarily. At the same time, testing on Windows 11 cannot be covered (we no longer support Windows 8 and below systems), and testing on macOS on x86 chips cannot be covered, as well as more lower versions of macOS systems.

Because the author [@czy-29](https://github.com/czy-29) personally uses Surface Laptop Studio for daily development and DELL workstations for music production, and moreover, he has the richest development experience on the Windows platform and the deepest understanding of Windows system programming (the [Win32 API](https://learn.microsoft.com/en-us/windows/win32/)), so the Windows 10 platform will have the highest level of support and quality assurance. macOS will be tested, but due to its short usage time, relatively lack of development experience, and lack of dedicated testing personnel, the quality may not meet the Windows 10 level guarantee, and unexpected edge case bugs may occur. Meanwhile, because Linux only tests on virtual machines, there may be a large number of physical machine use case scenarios that cannot be covered.

Of course, in addition to the operating system, there are also numerous issues with sound card drivers and audio plugins, which obviously cannot be fully covered by our testing. And in order to avoid legacy technical debt, for some buggy sound card drivers/audio plug-ins, unless unavoidable, we will only provide as few workarounds as possible in the low-level modules, and in most cases, we will only provide mitigation solutions in the high-level modules, making the low-level modules of the project as "transparent" as possible without letting the whole project's behavior become "dependent" on these third-party bugs (this is different from what JUCE does. JUCE has done a lot of workarounds for third parties in the low-level modules, which we think it is not a good practice).

For all the issues mentioned above, if you find any bugs, although we welcome you to submit an issue, we can only do our best to fix them. We will definitely fix any bugs that we have the conditions to reproduce. But for environments that we really cannot reproduce, we can only lower the priority of fixing it. We hope for your understanding.

There will be more short stories about testing in various environments in "[dev-notes](dev-notes/en/)", welcome to watch.

## Understanding Internals
The "[dev-notes](dev-notes/en/)" directory is a good starting point from which you can understand the overall design and implementation of the project.

## Why
1. As you can see, in the C++ ecosystem, there is a one-stop audio development framework like [JUCE](https://juce.com/), as well as a DAW audio engine like [tracktion_engine](https://github.com/Tracktion/tracktion_engine), but they all have various flaws (at least my own experience using them is very poor in many places), and they are C++(🤮). But in the Rust ecosystem, the distribution of audio crates are highly fragmented, lacking a unified solution, and many crates lack good maintenance, so I plan to write one myself. You can think this project as the [RIIR](https://github.com/ansuz/RIIR) version of JUCE + trackion_engine (but not quite, because the API of this project will be very different from theirs, it will be more elegant. At the same time, the API of this project will not include GUI modules, forcing you to practice a more modern architecture with front-end and back-end decoupling and strict isolation).
2. I am developing my own DAW (but DAW itself will be a commercial closed source project). I know that starting a new DAW from scratch in 2024 sounds like a joke, so I plan to fully open source the audio backend (which is this project) without reservation, introducing community power, and work together to open source and create. At the same time, the closed source of DAW front-end can retain commercial space, allowing this project to obtain funding for sustainable development. Therefore, overall speaking, the complete form of this project is actually an "[OpenCore](https://en.wikipedia.org/wiki/Open-core_model)" project. This project is the open source "Core", and the DAW (tentatively named OpenSound Studio) is the closed source part.
3. In order to maximize the adoption of this project, we will wrap a Web API Server at the earliest stage, allowing any developer who is not using Rust to call this project in their own language even before the C API is exposed -- as long as your language can send HTTP/WebSocket requests. This also forms a front-end/back-end "process isolation" architecture, so that anyone can develop their own front-end using any framework, making it easy to develop custom frontends.
4. I am also a semi-professional musician myself. In the process of arranging my own music, I discovered many features that I hoped to have, but none of them are available in the current DAWs. This is also the reason why I ultimately decided to write my own DAW. And many features will be implemented on this open-source core, which means that this project will have a lot of innovative audio features. So stay tuned!

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
