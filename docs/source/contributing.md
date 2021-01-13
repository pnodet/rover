---
title: "Contributing to Rover"
sidebar_title: "Contributing"
description: "Contributing to Rover's Development"
---

> Rover is not currently ready for external feature contributors, though some
> [documentation contributions](#documentation) may be accepted. Documentation 
> for contributions below is only here for internal contributors and partners.

## Prerequisites 

Rover is written in [Rust]. In order to contribute, you'll need to have
Rust installed. To install Rust, visit [https://www.rust-lang.org/tools/install].

Rust has a build tool and package manager called [`cargo`] that you'll use to 
interact with Rover's code.

To build the CLI:
```
cargo build
```

To run the CLI:
```
cargo run -- <args>
# e.g. cargo run -- help will run the rover help command
```

[Rust]: https://www.rust-lang.org/
[`cargo`]: https://doc.rust-lang.org/cargo/index.html
[https://www.rust-lang.org/tools/install]: https://www.rust-lang.org/tools/install

## Project Structure

- `src`: the `rover` CLI
    - `src/bin/rover.rs`: the entry point for the CLI executable
    - `src/command`: logic for the CLI commands
      - `src/command/output.rs`: Enum containing all possible `stdout` options for Rover commands
    - `src/utils`: shared utility functions
    - `src/cli.rs`: Module containing definition for all top-level commands
    - `src/client.rs`: Module with functions that setup and return an implemented `rover-client`.
    - `src/env.rs`: Logic for working with all env variables in Rover.
    - `src/lib.rs`: all the logic used by the CLI
    - `src/telemetry.rs`: implements `sputnik::Report` for capturing and reporting anonymous usage data

- `crates`
    - `crates/houston`: logic related to configuring rover
    - `crates/robot-panic`: Fork of robot-panic to create helpful panic handlers
    - `crates/rover-client`: logic for querying apollo services
    - `crates/sputnik`: logic for capturing anonymous usage data
    - `crates/timber`: output formatting and logging logic


## Documentation

Documentation for using and contributing to rover is built using Gatsby and [Apollo's Docs Theme for Gatsby](https://github.com/apollographql/gatsby-theme-apollo/tree/master/packages/gatsby-theme-apollo-docs).

To contribute to these docs, you can add or edit the markdown & MDX files in the `docs/source` directory.

To build and run the documentation site locally, you'll have to install the relevant packages by doing the following from the root of the `rover` repository:

```sh
cd docs
npm i
npm start
```

This will start up a development server with live reload enabled. You can see the docs by opening [localhost:8000](http://localhost:8000) in your browser.

To see how the sidebar is built and how pages are grouped and named, see [this section](https://github.com/apollographql/gatsby-theme-apollo/tree/master/packages/gatsby-theme-apollo-docs#sidebarcategories) of the gatsby-theme-apollo-docs docs. There is also a [creating pages section](https://github.com/apollographql/gatsby-theme-apollo/tree/master/packages/gatsby-theme-apollo-docs#creating-pages) if you're interesed in adding new pages.