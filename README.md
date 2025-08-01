[![crates.io](https://img.shields.io/crates/v/cargo-leptos)](https://crates.io/crates/cargo-leptos)
[![Discord](https://img.shields.io/discord/1031524867910148188?color=%237289DA&label=discord)](https://discord.gg/YdRAhS7eQB)

Build tool for [Leptos](https://crates.io/crates/leptos):

[<img src="https://raw.githubusercontent.com/gbj/leptos/main/docs/logos/Leptos_logo_RGB.png" alt="Leptos Logo" style="width: 30%; height: auto; display: block; margin: auto;">](http://https://crates.io/crates/leptos)

<br/>

- [Features](#features)
- [Getting started](#getting-started)
  - [Dependencies](#dependencies)
- [Single-package setup](#single-package-setup)
- [Workspace setup](#workspace-setup)
- [Build features](#build-features)
- [Parameters reference](#parameters-reference)
  - [Compilation parameters](#compilation-parameters)
  - [Site parameters](#site-parameters)
  - [Environment variables](#environment-variables)
  - [End-to-end testing](#end-to-end-testing)

<br/>

# Features

- Parallel build of server and client in watch mode for fast developer feedback.
- CSS hot-reload (no page-reload, only CSS updated).
- Build server and client for hydration (client-side rendering mode not supported).
- Support for both workspace and single-package setup.
- SCSS compilation using [dart-sass](https://sass-lang.com/dart-sass).
- CSS transformation and minification using [Lightning CSS](https://lightningcss.dev).
- Builds server and client (wasm) binaries using Cargo.
- Generates JS - Wasm bindings with [wasm-bindgen](https://crates.io/crates/wasm-bindgen)
  - Includes support for [JS Snippets](https://rustwasm.github.io/docs/wasm-bindgen/reference/js-snippets.html#js-snippets) for when you want to call some JS code from your WASM.
- Optimises the wasm with _wasm-opt_ from [Binaryen](https://github.com/WebAssembly/binaryen)
- `watch` command for automatic rebuilds with browser live-reload.
- `test` command for running tests of the lib and bin packages that makes up the Leptos project.
- `build` build the server and client.
- `end-to-end` command for building, running the server and calling a bash shell hook. The hook would typically launch Playwright or similar.
- `new` command for creating a new project based on templates, using [cargo-generate](https://cargo-generate.github.io/cargo-generate/index.html). Current templates include
  - [`https://github.com/leptos-rs/start`](https://github.com/leptos-rs/start): An Actix starter
  - [`https://github.com/leptos-rs/start-axum`](https://github.com/leptos-rs/start-axum): An Axum starter
  - [`https://github.com/leptos-rs/start-axum-workspace`](https://github.com/leptos-rs/start-axum-workspace): An Axum starter keeping client and server code in separate crates in a workspace
- 'no_downloads' feature to allow user management of optional dependencies
  <br/>

# Getting started

Install:

> `cargo install --locked cargo-leptos`

If you, for any reason, need the bleeding-edge super fresh version:

> `cargo install --git https://github.com/leptos-rs/cargo-leptos --locked cargo-leptos`

Help:

> `cargo leptos --help`

For setting up your project, have a look at the [examples](https://github.com/leptos-rs/cargo-leptos/tree/main/examples)

<br/>

## Dependencies

The dependency for [sass](https://sass-lang.com/install) is automatically installed in a cache directory when they are used if they are not already installed and found by [which](https://crates.io/crates/which).
Different versions of the dependencies might accumulate in this directory, so feel free to delete it.

| OS      | Example                                   |
| ------- | ----------------------------------------- |
| Linux   | /home/alice/.cache/cargo-leptos           |
| macOS   | /Users/Alice/Library/Caches/cargo-leptos  |
| Windows | C:\Users\Alice\AppData\Local\cargo-leptos |

If you wish to make it mandatory to install your dependencies, or are using Nix or NixOs, you can
install it with the `no_downloads` feature enabled to prevent cargo-leptos from trying to download and install them.

> `cargo install --features no_downloads --locked cargo-leptos`

<br/>

## Completions

For at least some shells (`bash` and `zsh` are confirmed to work) `cargo` is able to forward completions from custom
commands like `cargo leptos` back to the underlying `cargo-leptos` binary.

For that purpose `cargo-leptos` supports generating completions using `cargo leptos completions <SHELL>`.
Which you then can install/source for your shell. For bash, the command could like this 
```bash
cargo leptos completions bash > "${XDG_DATA_HOME:-"$HOME/.local/share}/bash-completion/completions/cargo-leptos"
```

# Single-package setup

The single-package setup is where the code for both the frontend and the server is defined in a single package.

Configuration parameters are defined in the package `Cargo.toml` section `[package.metadata.leptos]`. See the Parameters reference for
a full list of parameters that can be used. All paths are relative to the package root (i.e. to the `Cargo.toml` file)

<br/>

# Workspace setup

When using a workspace setup both single-package and multi-package projects are supported. The latter is when the frontend
and the server reside in different packages.

All workspace members whose `Cargo.toml` define the `[package.metadata.leptos]` section are automatically included as Leptos
single-package projects. The multi-package projects are defined on the workspace level in the `Cargo.toml`'s
section `[[workspace.metadata.leptos]]` which takes three mandatory parameters:

```toml
[[workspace.metadata.leptos]]
# project name
name = "leptos-project"
bin-package = "server"
lib-package = "front"

# more configuration parameters...
```

Note the double square brackets: several projects can be defined and one package can be used in several projects.

<br/>

# Build features

When building with cargo-leptos, the frontend, library package, is compiled into wasm using target
`wasm-unknown-unknown` and the features `--no-default-features --features=hydrate`
The server binary is compiled with the features `--no-default-features --features=ssr`

<br/>

# Parameters reference

These parameters are used either in the workspace section `[[workspace.metadata.leptos]]` or the package,
for single-package setups, section `[package.metadata.leptos]`.

Note that the Cargo Manifest uses the word _target_ with two different meanings.
As a package's configured `[[bin]]` targets and as the compiled output target triple.
Here, the latter is referred to as _target-triple_.

## Compilation parameters

```toml
# Sets the name of the binary target used.
#
# Optional, only necessary if the bin-package defines more than one target. Can also be set with the LEPTOS_BIN_TARGET=name env var
bin-target = "my-bin-name"

# Enables additional file hashes on outputted css, js, and wasm files
#
# Optional: Defaults to false. Can also be set with the LEPTOS_HASH_FILES=false env var (must be set at runtime too)
hash-files = false

# Sets the name for the file cargo-leptos uses to track the most recent hashes
#
# Optional: Defaults to "hash.txt". Can also be set with the LEPTOS_HASH_FILE_NAME="hash.txt" env var
hash-file-name = "hash.txt"

# The features to use when compiling all targets
#
# Optional. Can be extended with the command line parameter --features
features = []

# The features to use when compiling the bin target
#
# Optional. Can be over-ridden with the command line parameter --bin-features
bin-features = ["ssr"]

# If the --no-default-features flag should be used when compiling the bin target
#
# Optional. Defaults to false.
bin-default-features = false

# The profile to use for the bin target when compiling for release
#
# Optional. Defaults to "release".
bin-profile-release = "my-release-profile"

# The profile to use for the bin target when compiling for debug
#
# Optional. Defaults to "debug".
bin-profile-dev = "my-debug-profile"

# The target triple to use when compiling the bin target
#
# Optional. Env: LEPTOS_BIN_TARGET_TRIPLE
bin-target-triple = "x86_64-unknown-linux-gnu"

# The features to use when compiling the lib target
#
# Optional. Can be over-ridden with the command line parameter --lib-features
lib-features = ["hydrate"]

# If the --no-default-features flag should be used when compiling the lib target
#
# Optional. Defaults to false.
lib-default-features = false

# The profile to use for the lib target when compiling for release
#
# Optional. Defaults to "release".
lib-profile-release = "my-release-profile"

# The profile to use for the lib target when compiling for debug
#
# Optional. Defaults to "debug".
lib-profile-dev = "my-debug-profile"

# Fixes cargo bug that prevents incremental compilation (see #203)
#
# Optional. Defaults to false prior to 0.2.3, unconditionally enabled (with the setting becoming deprecated) since 0.2.3 and #216
separate-front-target-dir = true

# Pass additional parameters to the cargo process compiling to WASM
#
# Optional. No default
lib-cargo-args = ["--timings"]

# Pass additional parameters to the cargo process to build the server
#
# Optional. No default
bin-cargo-args = ["--timings"]

# The command to run instead of "cargo" when building the server
#
# Optional. No default. Env: LEPTOS_BIN_CARGO_COMMAND
bin-cargo-command = "cross"

# Whether to enable erased components mode for all cargo-leptos builds. This optimizes for compile speed,
# at the cost of a runtime/binary size overhead per component.
#
# Optional. By default, erased components are used for debug builds, and not for release builds.
always-erase-components = false

# Whether to disable erased components mode for all cargo-leptos builds.
#
# Optional. By default, erased components are used for debug builds, and not for release builds.
disable-erase-components = false

# Enabling wasm-opt features
#
# Optional. By default, "-Oz", "--enable-bulk-memory" and "--enable-nontrapping-float-to-int"  are used. For all features, consult `wasm-opt --help`.
# By providing features, you will override the default enabled features.
wasm-opt-features = ["-Oz","--enable-bulk-memory","--enable-nontrapping-float-to-int"]
```

## Site parameters

These parameters can be overridden by setting the corresponding environment variable. They can also be
set in a `.env` file as cargo-leptos reads the first it finds in the package or workspace directory and
any parent directory.

```toml
# Sets the name of the output js, wasm and css files.
#
# Optional, defaults to the lib package name or, in a workspace, the project name. Env: LEPTOS_OUTPUT_NAME.
output-name = "myproj"

# The site root folder is where cargo-leptos generate all output.
# NOTE: It is relative to the workspace root when running in a workspace.
# WARNING: all content of this folder will be erased on a rebuild!
#
# Optional, defaults to "/site" in the Cargo target directory. Env: LEPTOS_SITE_ROOT.
site-root = "target/site"

# The site-root relative folder where all compiled output (JS, WASM and CSS) is written.
#
# Optional, defaults to "pkg". Env: LEPTOS_SITE_PKG_DIR.
site-pkg-dir = "pkg"

# The source style file. If it ends with _.sass_ or _.scss_ then it will be compiled by `dart-sass`
# into CSS and processed by lightning css. When release is set, then it will also be minified.
#
# Optional. Env: LEPTOS_STYLE_FILE.
style-file = "style/main.scss"

# The tailwind input file.
#
# Optional, Activates the tailwind build
tailwind-input-file = "style/tailwind.css"

# The tailwind config file.
#
# Optional, defaults to "tailwind.config.js" which if is not present
# is generated for you
tailwind-config-file = "tailwind.config.js"

# The browserlist https://browsersl.ist query used for optimizing the CSS.
#
# Optional, defaults to "defaults". Env: LEPTOS_BROWSERQUERY.
browserquery = "defaults"

# Assets source dir. All files found here will be copied and synchronized to site-root.
# The assets-dir cannot have a sub directory with the same name/path as site-pkg-dir.
#
# Optional. Env: LEPTOS_ASSETS_DIR.
assets-dir = "assets"

# JS source dir. `wasm-bindgen` has the option to include JS snippets from JS files
# with `#[wasm_bindgen(module = "/js/foo.js")]`. A change in any JS file in this dir
# will trigger a rebuild.
#
# Optional. Defaults to "src"
js-dir = "src"

# Enables minification of the JS glue needed for Leptos to work.
# Uses SWC (Speed Web Compiler) to mangle and compress the JS, which it treats as a module.
#
# Applies to release builds only.
#
# Optional: Defaults to false. Can also be set with the LEPTOS_JS_MINIFY=false env var (must be set at runtime too)
js-minify = false

# Additional files your application could depends on.
# A change to any file in those directories will trigger a rebuild.
#
# Optional.
watch-additional-files = ["additional_files", "custom_config.json"]

# The IP and port where the server serves the content. Use it in your server setup.
#
# Optional, defaults to 127.0.0.1:3000. Env: LEPTOS_SITE_ADDR.
site-addr = "127.0.0.1:3000"

# The port number used by the reload server (only used in watch mode).
#
# Optional, defaults 3001. Env: LEPTOS_RELOAD_PORT
reload-port = 3001

# The command used for running end-to-end tests. See the section about End-to-end testing.
#
# Optional. Env: LEPTOS_END2END_CMD.
end2end-cmd = "npx playwright test"

# The directory from which the end-to-end tests are run.
#
# Optional. Env: LEPTOS_END2END_DIR
end2end-dir = "integration"

# The default prefix to use for server functions when generating API routes. Can be
# overridden for individual functions using `#[server(prefix = "...")]` as usual.
#
# This is useful to override the default prefix (`/api`) for all server functions without
# needing to manually specify via `#[server(prefix = "...")]` on every server function.
#
# Optional. Defaults to "/api". Env: SERVER_FN_PREFIX
server-fn-prefix = "/api"

# Whether to disable appending the server functions' hashes to the end of their API names.
#
# This is useful when an app's client side needs a stable server API. For example, shipping
# the CSR WASM binary in a Tauri app. Tauri app releases are dependent on each platform's
# distribution method (e.g., the Apple App Store or the Google Play Store), which typically
# are much slower than the frequency at which a website can be updated. In addition, it's
# common for users to not have the latest app version installed. In these cases, the CSR WASM
# app would need to be able to continue calling the backend server function API, so the API
# path needs to be consistent and not have a hash appended.
#
# Optional: Defaults to false. Env: DISABLE_SERVER_FN_HASH
disable-server-fn-hash = false

# Include the module path of the server function in the API route. This is an alternative
# strategy to prevent duplicate server function API routes (the default strategy is to add
# a hash to the end of the route). Each element of the module path will be separated by a `/`.
# For example, a server function with a fully qualified name of `parent::child::server_fn`
# would have an API route of `/api/parent/child/server_fn` (possibly with a
# different prefix and a hash suffix depending on the values of the other server fn configs).
#
# Optional, Defaults to false. Env: SERVER_FN_MOD_PATH
server-fn-mod-path = false
```

<br/>

## Environment variables

The following environment variables are set when compiling the lib (front) or bin (server) and when the server is run.

Echoed from the Leptos config:

- LEPTOS_OUTPUT_NAME
- LEPTOS_SITE_ROOT
- LEPTOS_SITE_PKG_DIR
- LEPTOS_SITE_ADDR
- LEPTOS_RELOAD_PORT

Directories used when building:

- LEPTOS_LIB_DIR: The path (relative to the working directory) to the library package
- LEPTOS_BIN_DIR: The path (relative to the working directory) to the binary package

Note when using directories:

- `cargo-leptos` changes the working directory to the project root or if in a workspace, the workspace root before building and running.
- the two are set to the same value when running in a single-package config.
- Avoid using them at run-time unless you can guarantee that the entire project struct is available at runtime as well.

Internally the versions of the external tools called by `cargo-leptos` are hardcoded. Use these environment variables to
override the versions `cargo-leptos` should use (e.g. `LEPTOS_SASS_VERSION=1.69.5`):

- LEPTOS_TAILWIND_VERSION
- LEPTOS_SASS_VERSION

## End-to-end testing

`cargo-leptos` provides end-to-end testing support for convenience. It is a simple
wrapper around a shell command `end2end-cmd` that is executed in a specific directory `end2end-dir`.

The `end2end-cmd` can be any shell command. For running [Playwright](https://playwright.dev) it
would be `npx playwright test`.

What it does is equivalent to running this manually:

- in a terminal, run `cargo leptos watch`
- in a separate terminal, change to the `end2end-dir` and run the `end2end-cmd`.

When testing the setup, please try the above first. If that works but `cargo leptos end-to-end`
doesn't then please create a GitHub ticket.
