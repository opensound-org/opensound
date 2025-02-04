# 开发手记

[English](../en/README.md) | 简体中文

本部分是项目开发过程的日常文字记录，主要会记录很多代码中无法记载的“What/Why/How”。您可以把本部分下的文章理解为一个一个的[RFC](https://en.wikipedia.org/wiki/Request_for_Comments)，但它跟RFC的作用又不完全相同，因为它也会记录很多关于背景知识以及背后的思考与思想相关的内容，因此它也承载了类似“日记”或“博客”或“指南”一样的功能（而且修订历史会被版本控制）。因为我们相信，“开放知识”与“开放源代码”同等重要，而且这些信息对于软件整体的理解，甚至关于设计决策的历史追溯，都有重要价值，它们的价值甚至会高过代码实现本身。

如果您熟悉Rust语言社区，您可以把本部分的功能类比为以下Rust生态内容的功能合集：
- [RFC Book](https://rust-lang.github.io/rfcs/)（起到“What”和“Why”的功能）
- [Unstable Book](https://doc.rust-lang.org/nightly/unstable-book/index.html)（起到“What”的功能）
- [Nomicon](https://doc.rust-lang.org/nomicon/index.html)（起到“Why”和“How”的功能）
- [rustc-dev-guide](https://rustc-dev-guide.rust-lang.org/)（起到“How”以及“指南”的功能）
- 各个Rust团队成员的个人博客（比如[Niko Matsakis](https://smallcultfollowing.com/babysteps/)的博客，起到“日记”或者“博客”的功能）

本部分下属一级子目录是不同的话题类别（具体类目有待更新，目前只有一个[design](design/)类别），每个目录内，则是平铺的，按讨论出现时间排序的具体话题的文章，文件名的数字前缀就是话题出现的顺序（你也可以把它理解为一种ID），每一个话题会讨论一个具体的问题，而且会持续更新，除非这个话题的展开越来越远，以至于必须要开新文章了，否则同一话题只会在同一文件中更新，让您可以对同一问题有一个整体的，深入的理解。

由于本部分是属于开发过程中的副产品，因此更新不会有任何的确定频率，只会取决于开发过程中，我们什么时候思考了什么。

## 先验知识
理解本部分文章的整体先验知识有且仅有需要您：
- 阅读过《[the Book](https://item.jd.com/12878638.html)》，并能写出可编译通过的Rust代码
- 阅读过《[Tokio Tutorial](https://tokio.rs/tokio/tutorial)》，并理解async/await语法以及异步运行时的作用以及使用方法

剩余的先验知识会在具体的话题文章内指出。
