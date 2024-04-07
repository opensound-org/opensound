# Dev Notes

English | [简体中文](../zh/README.md)

This section is a daily written record of the project development process, mainly recording many "What/Why/How" that cannot be recorded in the code. You can understand the articles under this section as [RFC](https://en.wikipedia.org/wiki/Request_for_Comments)s one by one, but their role is not exactly the same as RFCs because they also record a lot of background knowledge, thoughts and ideas related to it. Therefore, they also carry functions similar to "diaries", "blogs" or "guides" (and revision history is version controlled). Because we believe that, "open knowledge" is equally important as "open source code", and that this information has important value for the overall understanding of software and even for the historical tracing of design decisions, and their value even exceeds the code implementation itself.

If you are familiar with the Rust language community, you can analogize the functionality of this section to a collection of features from the Rust ecosystem:
- [RFC Book](https://rust-lang.github.io/rfcs/) (serves as "What" and "Why")
- [Unstable Book](https://doc.rust-lang.org/nightly/unstable-book/index.html) (serves as "What")
- [Nomicon](https://doc.rust-lang.org/nomicon/index.html) (serves as "Why" and "How")
- [rustc-dev-guide](https://rustc-dev-guide.rust-lang.org/) (serves as a "How" and "Guide")
- Personal blogs of various Rust team members (such as [Niko Matsakis](https://smallcultfollowing.com/babysteps/)’s blog, which functions as a “diary” or “blog”)

The first level subdirectories under this section are different topic categories (specific categories need to be updated, currently there is only one [design](design/) category), and within each directory, there are articles on specific topics sorted by the time when the discussion appears, and the numerical prefix of the file name is the order in which the topics appear (you can also understand it as an ID). Each topic will discuss a specific problem and will continue to be updated. Unless the topic is expanding further and further, and new articles must be opened, the same topic will only be updated in the same file, allowing you to have a comprehensive and in-depth understanding of the same problem.

Since this part is a by-product of the development process, updates will not have any definite frequency and will only depend on when and what we think about during the development process.

## Prior Knowledge
Understanding the overall prior knowledge of this section of the article requires and only requires you to:
- Have read "[the Book](https://doc.rust-lang.org/stable/book/)" and can write compilable Rust code
- Have read "[Tokio Tutorial](https://tokio.rs/tokio/tutorial)" and understand async/await syntax, as well as the role and usage of asynchronous runtime

The remaining prior knowledge will be pointed out in specific topic articles.
