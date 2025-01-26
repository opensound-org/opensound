<div align="center">

# opensound

[English](README.md) | 简体中文

这里是OpenSound项目的[单体仓库](https://en.wikipedia.org/wiki/Monorepo)。

[官网](https://opensound.run) | [crates.io](https://crates.io/crates/opensound) | [docs.rs](https://docs.rs/opensound/latest/opensound)

原始作者：[@czy-29](https://github.com/czy-29)

最新版本：[v0.0.6](https://github.com/opensound-org/opensound/releases/tag/v0.0.6)

![Crates.io Total Downloads](https://img.shields.io/crates/d/opensound)
[![Crates.io Dependents](https://img.shields.io/crates/dependents/opensound)](https://crates.io/crates/opensound/reverse_dependencies)
![GitHub Repo stars](https://img.shields.io/github/stars/opensound-org/opensound)

![MSRV (version)](https://img.shields.io/crates/msrv/opensound/0.0.6?label=v0.0.6-msrv)
[![dependency status (version)](https://deps.rs/crate/opensound/0.0.6/status.svg?subject=v0.0.6-deps)](https://deps.rs/crate/opensound/0.0.6)

![MSRV (git)](https://img.shields.io/badge/git--msrv-1.76.0-blue)
[![dependency status (git)](https://deps.rs/repo/github/opensound-org/opensound/status.svg?subject=git-deps)](https://deps.rs/repo/github/opensound-org/opensound)

[![Static Badge](https://img.shields.io/badge/build_with-Rust_1.84.0-dca282)](https://blog.rust-lang.org/2024/09/05/Rust-1.84.0.html)

</div>

## 是什么
OpenSound是使用[Rust](https://www.rust-lang.org/)开发的，一站式多层级的开源声音系统抽象层，或者说声音/音频引擎（当前正在开发中，且在早期开发阶段）。可适用于作为[专业音频](https://en.wikipedia.org/wiki/Professional_audio)应用（如[DAW](https://en.wikipedia.org/wiki/Digital_audio_workstation)）或其它声音相关应用的稳固后端内核。

“声音系统”基本可代指与声音有关的所有软件系统，从简单的音频播放器，到复杂的DAW，都应可以使用本项目方便地实现。“多层级”意味着从高层次的DAW工作流，到底层的操作系统音频API抽象，本项目都将提供封装与建模。“一站式”意味着上述所有能力都是“开箱即用”的，不需要您自己组合任何外部依赖去实现。

1.0将会是我们的[MVP](https://en.wikipedia.org/wiki/Minimum_viable_product)版本，它会包含：
- 模块化的核心Rust API
- 一个插件化的桌面应用开发框架
- 一个（使用以上两者构建的）可定制的Web API Server
- 一个内置的Web GUI Playground

核心Rust API更像是一些模块化的“积木”，而高一级的应用框架则是可以让你以一种插件化的架构，轻松而优雅地将积木“拼装”成一个完整的应用。而Web API Server本身不但可以直接用于声音应用的开发，而且也是可以作为以上两个部分用法的一个很好的示例。

而内置的Web GUI Playground将主要用于探索&测试&[Live Coding](https://en.wikipedia.org/wiki/Live_coding)用途（它的形态会比较像[JUCE](https://juce.com/)的DemoRunner和AudioPluginHost，以及本项目的一些特色功能的结合体），但是当然，您也可以用它来进行编曲/音乐制作，只是如果您仅使用Playground的交互前端的话，您的工作流将会比较繁杂（比如Playground可能并不会有一个完整的钢琴卷帘）。与此同时，尽管自带的Web GUI Playground前端不是一个完整的DAW体验，但是背后的Web API Server应当可用于作为一个DAW的完整后端。

但是现在，我们目前正在专注于0.1版本，它将是一个[PoC](https://en.wikipedia.org/wiki/Proof_of_concept)版本。
PoC版本和MVP版本的主要区别在于，PoC版本不会有Web GUI Playground，取而代之的是一个用于测试和Live Coding演示的命令行“[REPL](https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop)脚本控制台”。

v0.1之前还会有若干个v0.0.x版本，每实现一个小功能都会bump一个新版本。

在未来（MVP之后），我们计划的路线图是：
- 包装C API，并创建不同语言的绑定（如C++/Python/C#……）
- 移动端支持（1.0将仅会支持桌面平台）
- WASM支持
- OpenSound原生Playground（使用Flutter & opensound的C API）
- 音频插件开发框架（如VST/VST3/AU/CLAP）
- 游戏音频功能，音频引擎（如Wwise/FMod）插件开发，以及游戏引擎（如Unity/Unreal/Godot）集成
- （或许）嵌入式设备支持

## 词源学
OpenSound的“Open”，就是“[Open Source](https://en.wikipedia.org/wiki/Open_source)”的“Open”（类似“[OpenCV](https://opencv.org/)”的命名），同时也是“[Open Standard](https://en.wikipedia.org/wiki/Open_standard)”的“Open”（类似“[OpenAPI](https://www.openapis.org/)”的命名）。同时我们的开源是原教旨主义开源，也就是符合[OSI](https://opensource.org/)定义下的[开源](https://opensource.org/osd)。

另外，“OpenSound”的缩写是“OS”，与“操作系统”的缩写相同，这是有意设计的，项目的很多概念也会借用自操作系统中的概念，而且这也反映了项目的终极目标，就是变成一个“声音”操作系统！

## 安装
### 二进制使用：
如果您只想尝鲜本项目，或者您想直接使用预编译的Web API Server，那么您可以：
```
cargo install opensound
```
或者如果您没有安装Rust或者您不想使用`cargo install`，您也可以直接从[Github Releases](https://github.com/opensound-org/opensound/releases)下载预构建的二进制（macOS和Linux版本在执行前可能需要您先给二进制运行一下`chmod +x`）。

#### 复现[Github Release](https://github.com/opensound-org/opensound/releases/tag/v0.0.6)中的预构建二进制：

目前，整个发布过程是纯手动完成的，但未来计划使用Github Actions将整个过程自动化。

以下步骤描述了手动构建Github Release中的二进制的方法：
- 首先：`git checkout v0.0.6`
- 然后Windows版本直接在msvc工具链（也就是Windows机器下的默认工具链）下执行 `cargo build --release` 即可构建。
- 对于macOS和Linux版本，为了使构建制品可以跨OS发行版运行，我们使用了“[cargo-zigbuild](https://crates.io/crates/cargo-zigbuild)”。所以请先参阅他们的指南以正确安装cargo-zigbuild（包含正确安装zig，以及添加Rust targets）。
- 然后macOS版本可以通过运行 `cargo zigbuild --target universal2-apple-darwin --release` 来构建（需要macOS 11.0以上的机子）。
- Linux版本可以在任何Windows 10+/macOS 10.12+/Linux（内核3.2+，glibc 2.17+）机子上，通过运行 `cargo zigbuild --target x86_64-unknown-linux-gnu --release` 来交叉编译构建。

### 库使用：
您可以：
```
cargo add opensound
```
如果您更想使用底层的Rust API。

## 分支模型
本项目采用“[集中式工作流](https://git-scm.com/book/zh/v2/%E5%88%86%E5%B8%83%E5%BC%8F-Git-%E5%88%86%E5%B8%83%E5%BC%8F%E5%B7%A5%E4%BD%9C%E6%B5%81%E7%A8%8B)”，意味着只有一个单一的“main”分支，所有的工作都在本分支上进行。每个版本发布时，会创建一个当前版本的tag，您checkout某一个tag，即可得到相应版本的完整代码。如果您发现某个历史版本存在bug，您需要先升级到最新版本检查bug是否依然存在。如果依然存在，请到[GitLab](https://gitlab.com/opensound-org/opensound/-/issues)或[Github](https://github.com/opensound-org/opensound/issues)发起issue，并等待下个版本的修复。我们暂不对历史版本提供[hotfix](https://en.wikipedia.org/wiki/Hotfix)支持（因为人力带宽有限）。

本项目的版本发布没有固定的周期，但是版本号会遵循[SemVer](https://semver.org/lang/zh-CN/)：在v0.1之后，每个修订版本会仅包含bug修复，而每个小版本会包含小的功能更新，而每个大版本则会包含重大功能更新（但是在没有达到v0.1 PoC的目标之前，所有的小功能更新都只会增加版本号的最后一位，也就是修订版本）。

## Rust版本策略
作为官方，我们始终使用最新稳定版的Rust（当前是1.84.0）来构建和测试本项目。但是任何高于[Cargo.toml](Cargo.toml)中指定的[MSRV](https://doc.rust-lang.org/stable/cargo/reference/manifest.html#the-rust-version-field)（当前是1.76.0，因为依赖项中MSRV最高的“[salvo](https://crates.io/crates/salvo)”的MSRV是1.76.0）的Rust版本，均应正常构建，但是行为是否正确，官方不予保证。

如果您使用低于最新稳定版的Rust构建本项目后，遇到一些问题，请先尝试升级到最新稳定版的Rust之后，再来看看问题是否依然存在。如果问题依然存在，请到[GitLab](https://gitlab.com/opensound-org/opensound/-/issues)或[Github](https://github.com/opensound-org/opensound/issues)发起issue。

## 平台支持策略
我们官方维护：
- x86_64-pc-windows-msvc
- [universal2-apple-darwin](https://crates.io/crates/cargo-zigbuild)（`x86_64-apple-darwin` 和 `aarch64-apple-darwin` 的组合）
- x86_64-unknown-linux-gnu

三个target的build。其它target可能也可以工作，但是不做保证。我们保证本项目的所有版本，这三个target都可以构建通过，但是对于它们测试的支持程度，这三个target会有差异（同样是由于资源和人力带宽有限）。

同时在v0.0.x阶段，不是所有功能在所有平台上的行为都会是一致的，我们会有平台特定功能的开发优先级，某些版本只会专注特定平台的特定功能的开发，具体版本情况参见每个版本的Release Note或CHANGELOG。但是在v0.1之后，我们就会尽可能保证所有平台行为的一致性。

接下来我们会解释一下对测试和平台特定bug修复支持的情况：

首先，我们只有两台Windows 10机器（一台Surface Laptop Studio和一台DELL工作站），和一台macOS Sonoma机器（2023款MacMini，M2芯片），因此Linux平台的测试暂时只能在虚拟机中进行，同时Windows 11的测试无法被覆盖到（Windows 8及以下的系统我们不再支持），另外x86芯片的macOS的测试也无法被覆盖到，以及更多低版本的macOS系统。

因为 [@czy-29](https://github.com/czy-29) 作者个人日常开发会使用Surface Laptop Studio，同时制作音乐时会使用DELL工作站，而且他对Windows平台的开发经验最丰富，对Windows系统编程（[Win32 API](https://learn.microsoft.com/en-us/windows/win32/)）的理解也最深刻，因此Windows 10平台会有最高等级的支持和质量保证。macOS会进行测试但是因为使用时间不长，开发经验相对缺少，且没有专职的测试人员，因此质量可能无法达到Windows 10级别的保证，可能会出现意想不到的edge case bug。同时因为Linux只会在虚拟机中进行测试，因此可能有大量的物理机用例场景，是无法被覆盖到的。

当然，除了操作系统以外，还有大量的声卡驱动，以及音频插件的问题，这些显然我们的测试是不可能100%覆盖全的。而且为了避免遗留技术债务，对于部分有bug的声卡驱动/音频插件，除非不可避免，否则我们只会在底层模块提供尽可能少的workaround，而大部分情况下，我们都只会在上层模块提供缓解方案，让项目的底层模块尽可能地“透明”，而不会让整个项目的行为变成了“依赖”这些第三方的bug（这就不同于JUCE的做法了，JUCE在底层模块做了大量的对第三方的workaround，我们认为这不是一种好的实践）。

对于以上提到的所有这些问题，如果您发现了一些bug，我们虽然欢迎您提交issue，但是对于修复，我们只能是尽力而为。对于我们有条件复现的bug，我们一定会修复。但是对于我们实在没有办法复现的环境，我们只能将修复它的优先级降低，也望理解。

“[dev-notes](dev-notes/zh/)”中还会有更多关于各种环境下测试的小故事，欢迎围观。

## 理解内幕
“[dev-notes](dev-notes/zh/)”目录是您可以理解项目整体设计与实现的很好的起点。

## 为什么
1. 正如您所见，在C++生态中，有像[JUCE](https://juce.com/)这样的一站式音频开发框架，也有像[tracktion_engine](https://github.com/Tracktion/tracktion_engine)这样的DAW音频引擎，但它们都存在各种各样的缺陷（至少我自己的使用体验在很多地方都很差），而且它们是 C++（🤮）。然而在 Rust 生态中，音频crate的分布又高度碎片化，缺少一个“大一统”的解决方案，而且很多crate缺少良好的维护，所以我打算自己写一个。你可以把这个项目看成是JUCE + trackion_engine的[RIIR](https://github.com/ansuz/RIIR)版本（但不完全是，因为这个项目的API会和它们的有很大不同，会更优雅。同时本项目的API不会包含GUI模块，强制您实践一种更现代的，前后端解耦合并且严格隔离的架构）。
2. 我正在开发一个自己的DAW（但DAW本身将是一个商业闭源项目）。我知道在2024年从头开始写一个新的DAW听起来像是一个笑话，所以我打算毫无保留地全面开源音频后端（也就是这个项目），引入社区力量，大家开源共创。同时，DAW前端的闭源也可以保留商业化空间，让这个项目可以获得资金来持续发展。因此，总的来说，这个项目的完整形态实际上是一个“[OpenCore](https://en.wikipedia.org/wiki/Open-core_model)”项目。本项目是这个开源的“核心”，而DAW（暂定名为OpenSound Studio）则是其闭源的部分。
3. 为了最大化本项目的采用率，我们将在最早期就封装一个Web API Server，允许哪怕不使用Rust的任何开发者，甚至在C API暴露之前就可以使用自己的语言调用本项目——只需要您的语言可以发送HTTP/WebSocket请求。这也形成了一种前后端“进程隔离”的架构，允许任何人使用任何框架来开发自己的前端，使得开发自定义前端变得更容易。
4. 我自己本身也是一名半职业音乐人。在给自己的音乐编曲的过程中，我发现了很多我希望拥有，但当前市面上的DAW都没有的功能，这也是我最终决定开发一个自己的DAW的其中一个原因。而且很多这些功能我都会实现在这个开源核心上，这就意味着本项目将会有很多创新的音频功能。所以敬请期待！

## Star历史

[![Star History Chart](https://api.star-history.com/svg?repos=opensound-org/opensound&type=Date)](https://star-history.com/#opensound-org/opensound&Date)

# 许可证

本项目使用以下两种许可之一

 * Apache协议，2.0版本，([LICENSE-APACHE](LICENSE-APACHE) 或
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT协议 ([LICENSE-MIT](LICENSE-MIT) 或
   http://opensource.org/licenses/MIT)

由您选择。

## 贡献

[Github](https://github.com/opensound-org/opensound)是我们的[单一信源](https://en.wikipedia.org/wiki/Single_source_of_truth)，这里我们欢迎所有的issue和pull request。

我们另有两个[自动推送](.github/workflows/mirror.yml)的下游只读镜像：
- [Gitea](https://gitea.29bot.com/opensound-org/opensound)
- [Gitee](https://gitee.com/opensound-org/opensound)

由于它们是只读镜像，因此请不要在这两个平台上发起任何合并请求或pull request。

除非您另有明确说明，否则您有意提交的
包含在 `opensound` 中的任何贡献（如 Apache-2.0 许可证中所定义）均应
获得上述双重许可，无需任何附加条款或条件。
