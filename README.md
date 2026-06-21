# jscore.ewwii

Ewwii plugin providing JavaScript language support in a unique way.

## Features

- **npm support**: Wide range of packages from js ecosystem
- **v8 engine**: Blazingly fast evaluation
- **self-sufficient**: No need to rely on external scripts

## Installation

You can use the following command to install jscore using [eiipm](https://github.com/ewwii-sh/eiipm).

```sh
# if you prefer prebuilt
eiipm add "Ewwii-sh/jscore.ewwii" --prebuilt --ref <version>

# if you prefer building yourself
eiipm add "Ewwii-sh/jscore.ewwii"                  # Up to date with "main" branch
eiipm add "Ewwii-sh/jscore.ewwii" --ref <version>  # Locked to a specific version
```

Or you can download it manually from [Github Releases](https://github.com/ewwii-sh/jscore.ewwii/releases).

If you are downloading from the GitHub Releases, make sure to create a `plugins/` directory in your 
ewwii configuration and put the `libjscore.so` in there.

## Usage

See the documentation for usage guide:

https://ewwii-sh.github.io/jscore.ewwii

## Versioning Notice

This project uses a custom versioning system instead of the standard Semantic Versioning to avoid confusion.

It uses a **EWWII_VERSION.RELEASE_NUMBER** system where:

- **EWWII_VERSION**: The version of ewwii this release was made for.
- **RELEASE_NUMBER**: The patch number of this plugin.
