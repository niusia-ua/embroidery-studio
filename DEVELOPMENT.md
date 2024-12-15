# Development Process

This document describes the process of how we are working on Embroidery Studio.

You can follow it to start working on your ideas, improvements, fixes, etc.

## Project Structure

```
src/ # Everything related to the frontend.
├── api/ # Contains modules to interact with the backend through Tauri commands.
├── assets/ # Styles, fonts, icons, images, etc.
├── components/ # Vue.js components.
├── schemas/ # Schemas and types for parsing borsh-serialized data.
├── services/ # Modules that encapsulate complex logic.
├── stores/ # Pinia stores to share some application state through components.
├── types/ # Type definitions.
├── utils/ # A set of utility functions.
├── App.vue # The main application component.
└── main.ts # An entry point for the entire application.
src-tauri/ # Everything related to the backend.
├── capabilities/ # A set of permissions for the application.
├── icons/ # Desktop icons.
├── resources/ # Sample patterns, stitch fonts, colour palettes, etc.
├── src/ # Application source code.
│   ├── commands/ # A set of Tauri commands exposed to the frontend.
│   ├── core/ # The core functionality.
│   │   ├── actions/ # A set of actions for performing changes to patterns.
│   │   ├── parser/ # Cross-stitch pattern files parsers.
│   │   │   ├── oxs/ # OXS parser.
│   │   │   └── xsd.rs # XSD parser.
│   │   ├── pattern/ # Pattern structure definition that is used internally.
│   │   │   └── stitches/ # Definitions of the various stitch kinds and their methods.
│   │   └── history.rs # Defines a structure to save performed action objects.
│   ├── utils/ # A set of utility functions.
│   ├── error.rs # Defines custom error type for the command result.
│   ├── logger.rs # Configures the Tauri logger plugin.
│   ├── state.rs # Defines the application states.
│   ├── lib.rs # Composes all modules into the library used in `main.rs` and `tests/`.
│   └── main.rs # Bundles everything together and runs the application.
└── tests/ # End-to-end backend tests.
```

## Prerequisites

To get started working on Embroidery Studio, you will first need to install a few dependencies:

1. [System Dependencies](https://tauri.app/start/prerequisites/#system-dependencies).

2. [Rust](https://rust-lang.org/tools/install) and [Node.js](https://nodejs.org/en/download).

   We are using the latest stable Rust version and the latest LTS Node.js version.
   Also, we are using the nightly Rust edition for running its tooling with unstable features.

3. [Cargo Tauri CLI](https://tauri.app/reference/cli).

4. NPM and Cargo dependecnies.

   Run `npm install` in the porject root.
   Cargo dependencies will be installed during the first run of the project.

### VS Code Setup

We are using Visual Studio Code as our development environment.
Here is a recommended setup:

1. Install and enable extensions listed in `.vscode/extensions.json`.
2. Configure the VS Code using the following `.vscode/settings.json` file:

   ```json
   {
     // Enable auto-formatting on file save.
     "editor.formatOnSave": true,
     "editor.codeActionsOnSave": {
       "source.fixAll": "explicit"
     },
     // Use Prettier as the default formatter.
     "editor.defaultFormatter": "esbenp.prettier-vscode",

     // Enable and configure file nesting to hide related files.
     "explorer.fileNesting.enabled": true,
     "explorer.fileNesting.expand": false,
     "explorer.fileNesting.patterns": {
       // Hide all documentation files and a license under the `README.md`.
       "README.md": "*.md, LICENSE",
       // Hide all unit test files under their source files.
       "*.vue": "${capture}.test.ts",
       "*.ts": "${capture}.test.ts",
       "*.rs": "${capture}.test.rs",
       // Hide all TypeScript configs under the main one.
       "tsconfig.json": "tsconfig.*.json, *.d.ts, *.tsbuildinfo",
       // Hide lock files under the main manifests.
       "package.json": "package-lock.json",
       "Cargo.toml": "Cargo.lock, rustfmt.toml",
       // Hide ESLint and Prettier configurations under `.editorconfig`.
       ".editorconfig": "eslint.config.js, .prettierrc.json",
       // Hide PostCSS and Tailwind configs under Vite config.
       "vite.config.ts": "postcss.config.js, tailwind.config.ts"
     },

     // Optionally, use enhanced syntax highlighter for TOML files.
     // Make sure you have installed a corresponding extension.
     // "[toml]": {
     //   "editor.defaultFormatter": "tamasfe.even-better-toml"
     // },

     // Optionally, use enhanced syntax highlighter for XML files to view OXS patterns.
     // Make sure you have installed a corresponding extension.
     // "[xml]": {
     //   "editor.defaultFormatter": "redhat.vscode-xml"
     // },

     // Use RustAnalyzer as the default formatter for Rust source files.
     "[rust]": {
       "editor.defaultFormatter": "rust-lang.rust-analyzer"
     },
     // Exclude build artefacts and frontend sources from being tracked by RustAnalyzer.
     "rust-analyzer.files.excludeDirs": ["node_modules", "src", "dist"],
     // Force Rustfmt to use the nightly Rust version.
     "rust-analyzer.rustfmt.extraArgs": ["+nightly"],

     // Exclude build artefacts from being tracked by VS Code.
     "files.watcherExclude": {
       "node_modules/**": true,
       "dist/**": true,
       "src-tauri/target/**": true
     }
   }
   ```

## Running and Building Application

We prefer using Cargo Tauri CLI to work with the project.

> By default, Tauri installs a `@tauri-apps/cli` NPM dev dependency in a new project.
> However, this caused some trouble with management lock files when working with different OSs, so we removed it and used Cargo Tauri CLI instead.

Refer to the [reference](https://tauri.app/reference/cli) to see available commands.

## A Few Words About Testing

All unit tests are extracted into separate files titled `[filename].test.{ts,vue}`.
All end-to-end tests are located under the `tests/` folder (in the root for frontend tests and in the `src-tauri/` for backend tests).

> It may be inconvenient to navigate through the projects.
> Check out the recommended VS Code setup described above to fix that.

## Organization Notes

We are following [conventional commits](https://conventionalcommits.org/en/v1.0.0), [semantic branch names](https://gist.github.com/seunggabi/87f8c722d35cd07deb3f649d45a31082) and [semantic versioning](https://semver.org).
However, in PRs, you can title commits as you want; in any case, they will be squashed.

## Before Submitting a PR

Please make sure your code is well-formatted, linted and properly tested:

- Check the frontend code:

  ```sh
  npm run fmt
  npm run lint
  npm run test
  ```

- Check the backend code:

  ```sh
  cd src-tauri/
  cargo fmt --check
  cargo clippy -- -D warnings
  cargo test
  ```

You can configure local Git hooks so these checks run on every commit/push.
