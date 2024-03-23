# opensound

[English](README.md) | 简体中文

这里是OpenSound项目的单体仓库（monorepo）

[官网](https://opensound.run) | [crates.io](https://crates.io/crates/opensound) | [docs.rs](https://docs.rs/opensound/latest/opensound)

最新版本：[v0.0.3](https://gitlab.com/opensound-org/opensound/-/tree/0.0.3?ref_type=tags)

## 是什么
OpenSound是使用Rust开发的，一站式多层级的开源声音系统抽象层（当前正在开发中，且在早期开发阶段），可适用于作为[专业音频](https://en.wikipedia.org/wiki/Professional_audio)应用（如[DAW](https://en.wikipedia.org/wiki/Digital_audio_workstation)）或其它声音相关应用的稳固后端内核。

1.0将会是我们的[MVP](https://en.wikipedia.org/wiki/Minimum_viable_product)版本，它会包含：
- 核心Rust API
- 一个Web API Server
- 一个内置的Web GUI Playground

内置的Web GUI Playground将主要用于探索&测试&[Live Coding](https://en.wikipedia.org/wiki/Live_coding)用途，但是当然，您也可以用它来进行编曲/音乐制作，只是如果您仅使用Playground的交互前端的话，您的工作流将会比较繁杂（比如Playground可能并不会有一个完整的钢琴卷帘）。

但是现在，我们目前正在专注于0.1版本，它将是一个[PoC](https://en.wikipedia.org/wiki/Proof_of_concept)版本。
PoC版本和MVP版本的主要区别在于，PoC版本不会有Web GUI Playground，取而代之的是一个用于测试和Live Coding演示的命令行“[REPL](https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop)脚本控制台”。

在未来（MVP之后），我们计划的路线图是：
- 包装C API，并创建不同语言的绑定（如C++/Python/C#……）
- 移动端支持（1.0将仅会支持桌面平台）
- WASM支持
- OpenSound原生Playground（使用Flutter & opensound的C API）
- 游戏音频功能，以及游戏引擎（如Unity/Unreal/Godot）集成

## 安装
您可以：
```
cargo install opensound
```
如果您只想尝鲜本项目，或者您想直接使用预编译的Web API Server。

或者您也可以：
```
cargo add opensound
```
如果您更想使用底层的Rust API。

## 为什么
1. 正如您所见，在C++生态中，有像[JUCE](https://juce.com/)这样的一站式音频开发框架，也有像[tracktion_engine](https://github.com/Tracktion/tracktion_engine)这样的DAW音频引擎，但它们都存在各种各样的缺陷（至少我自己的使用体验在很多地方都很差），而且它们是 C++（🤮）。然而在 Rust 生态中，音频crate的分布又高度碎片化，缺少一个“大一统”的解决方案，而且很多crate缺少良好的维护，所以我打算自己写一个。你可以把这个项目看成是JUCE + trackion_engine的[RIIR](https://github.com/ansuz/RIIR)版本（但不完全是，因为这个项目的API会和它们的有很大不同，会更优雅）。
2. 我正在开发一个自己的DAW（但DAW本身将是一个商业闭源项目）。我知道在2024年从头开始写一个新的DAW听起来像是一个笑话，所以我打算毫无保留地全面开源音频后端（也就是这个项目），引入社区力量，大家开源共创。同时，DAW前端的闭源也可以保留商业化空间，让这个项目可以获得资金来持续发展。因此，总的来说，这个项目的完整形态实际上是一个“[OpenCore](https://en.wikipedia.org/wiki/Open-core_model)”项目。本项目是这个开源的“核心”，而DAW（暂定名为OpenSound Studio）则是其闭源的部分。
3. 为了最大化本项目的采用率，我们将在最早期就封装一个Web API Server，允许哪怕不使用Rust的任何开发者，甚至在C API暴露之前就可以使用自己的语言调用本项目——只需要您的语言可以发送HTTP/WebSocket请求。这也形成了一种前后端“进程隔离”的架构，允许任何人使用任何框架来开发自己的前端，使得开发自定义前端变得更容易。
4. 我自己本身也是一名半职业音乐人。在给自己的音乐编曲的过程中，我发现了很多我希望拥有，但当前市面上的DAW都没有的功能，这也是我最终决定开发一个自己的DAW的其中一个原因。而且很多这些功能我都会实现在这个开源核心上，这就意味着本项目将会有很多创新的音频功能。所以敬请期待！

# 许可证

本项目使用以下两种许可之一

 * Apache协议，2.0版本，([LICENSE-APACHE](LICENSE-APACHE) 或
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT协议 ([LICENSE-MIT](LICENSE-MIT) 或
   http://opensource.org/licenses/MIT)

由您选择。

## 贡献

[GitLab](https://gitlab.com/opensound-org/opensound)是我们的[单一信源](https://en.wikipedia.org/wiki/Single_source_of_truth)，[Github](https://github.com/opensound-org/opensound)版本只是一个只读镜像，因此请不要在Github版本上面发起任何pull requests。

[GitLab](https://gitlab.com/opensound-org/opensound)版本上面的合并请求是欢迎的！

除非您另有明确说明，否则您有意提交的
包含在 opensound 中的任何贡献（如 Apache-2.0 许可证中所定义）均应
获得上述双重许可，无需任何附加条款或条件。
