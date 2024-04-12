# darling-api

**Note: darling-api is still pre-1.0, and breaking changes will occur. Consider very carefully if you actually want to implement an API that will certainly have breaking changes in the near future and require updates.**

`darling-api` is the library crate that allows custom implementations of [darling](https://github.com/darling-package-manager/darling). Darling is designed to specifically be extendible without changing darling itself, so that anyone can implement it for their package manager without having to contribute to darling itself. This API provides the necessary capabilities to implement darling.

## Usage

This API can be implemented either by modifying a pre-existing template, or by creating a new crate from scratch. The template is recommended due to some quirks about naming, but you're free to start from scratch if you understand how to implement it correctly.

### Implementing From Template

[darling-template](https://github.com/darling-package-manager/darling-template) provides a starting point for implementing darling. To use it, first set the template up onto your local machine:

#### With version contol

Open the repository and click "Use this template > Create a new repository". This will set up a git repo that you can clone onto your local machine and begin development with git immediately.

#### Without version control

If you don't want to make a git repository for your project right now, you can just clone the template to get it onto your local machine.

#### Next steps

Next, rename the struct and implement the missing methods. Read the documentation for each method carefully to understand what it must do and not do. **Do not rename the global `PACKAGE_MANAGER` variable**.

Edit your crate name in `Cargo.toml`, and ensure it starts with `darling-`, such as `darling-npm` or `darling-vscode`. Publish your crate when it's done!

That's it! once your crate is published, it can be used by anyone with darling, no updates required.

### Manual Implementation

Alternatively, you can start from scratch and create a module without using a template. The process is as follows:

- Create a rust (library) project. **It must start with `darling-`** (technically `darling_` is also allowed, but not the convention). For example, `cargo new darling-example --lib`. Ensure that your name isn't taken on [crates.io](https://crates.io).
- Add `darling-api` to your dependencies with `cargo add darling-api`.
- Create an empty struct that implements `darling::PackageManager`.
	- Ensure that the `get_name()` function returns a consistent value on all calls, and that **it does not return "module"**. `module` is a built-in reserved name used by darling to manage itself. Also, it should be unique to other darling modules, or else they will be incompatible. It is convention to make it the name of your crate, without the `darling-` prefix. For example, the `darling
- **Declare a `pub static` variable of your struct with the name `PACKAGE_MANAGER` that is accessible from your crate root.**
- Publish your crate on `crates.io` with `cargo publish`
