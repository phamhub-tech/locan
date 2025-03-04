<div id="app-logo" align="center">
    <br />
    <img src="./assets/logo.svg" height="100px" />
    <h1>Locan</h1>
		<h3>Your humble <b>L</b>ines <b>O</b>f <b>C</b>ode <b>An</b>alyser</h3>
</div>

<div id="badges" align="center">

[![current release](https://img.shields.io/github/release/phamhub-tech/locan.svg)](https://github.com/phamhub-tech/locan/releases)
[![license](https://img.shields.io/github/license/phamhub-tech/locan.svg)](https://github.com/phamhub-tech/locan/blob/main/LICENSE)

[![build status](https://img.shields.io/github/actions/workflow/status/phamhub-tech/locan/dev-release.yml?branch=dev&label=build)](https://github.com/phamhub-tech/locan/actions/workflows/dev-release.yml?query=branch%3Adev)
</div>


## 🚧 **Early Development** 🚧  
This project is still in early development and may have bugs and be unstable. Please report any issues you encounter.


## Features

The app currently supports:
- **Project Scanning**: Display total project lines of code  lines of code for each file type.
- **Exclude files/directories**: Customise scanning by excluding specific files and directories in the global settings.
- **Automatic Exclusion**: Automatically exclude files and directories listed in the project's `.gitignore` and `.ignore` files.
- **Scan History**: View a history of previous scans.


## Download / Install

You can download the latest release from [here](https://github.com/phamhub-tech/locan/releases).

## Build from source

Ensure you have the following dependencies installed:
- [rust](https://www.rust-lang.org/tools/install).
- [node](https://nodejs.org/en/download/package-manager).
- [pnpm](https://pnpm.io/installation).
	
	To install `pnpm` after installing `node` run:
	```sh
	npm i -g pnpm
	```

Install frontend dependencies:

```bash
pnpm install
```

Build the app:

```sh
pnpm tauri build
```

## Contributing

### Runing the app

Clone this repository:

```sh
# ssh (recommended)
git clone git@github.com:phamhub-tech/locan.git

# http
git clone https://github.com/phamhub-tech/locan.git
```

Instal the dependencies:

```sh
pnpm install
```

Start the app in develpment (runs on `http://localhost:5000`):

```sh
pnpm tauri dev
```

### Contributing to the project

We welcome contributions!.  Here's how you can help:

- Fork the project.
- Create a branch from the `main` branch:
	```sh
	git checkout -b feat/<feature-name>
	```
- Commit you changes.
- Push the branch.
- Open a Pull Request against the `main` branch.


## Pending features

### Core Features

- [ ] Implement a robust logging system.
- [ ] Detect and report chrashes.
- [ ] Templating project settings based on the type of project selected.

### UI/Ux Features
- [ ] Add custom themes.
- [ ] Implementing a command palette for quick actions.

## Feedback & Support

If you encounter any issues or have suggestions with the app, please [Open an Issue](https://github.com/phamhub-tech/locan/issues).  We appreciate your feedback