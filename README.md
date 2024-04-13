# cloud-storage-utilizer

`cloud-storage-utilizer` is a capstone project for the [*Ukrainian Rust Programming
Bootcamp*](https://t.me/rustlang_ua).

This project presents a TUI application backed with [`ratatui`] that allows users to perform basic operations with cloud
storages.

## Installation

You can simply install the application with:

```bash
cargo install --git https://github.com/BigglesworthCat/cloud-storage-utilizer
```

Or you can clone the GitHub repository and build it from sources:

```bash
git clone https://github.com/BigglesworthCat/cloud-storage-utilizer
cd cloud-storage-utilizer
cargo install --path .
```

## Preparation

At this moment, only [Dropbox] cloud storage is supported.

### Dropbox

To use [Dropbox] as cloud storage, you need to register your application and get an
access token following this [instruction](https://www.dropbox.com/developers/reference/getting-started#overview).

This access token must be set into the `DROPBOX_ACCESS_TOKEN` environment variable.

Don't forget to give necessary scope access (in *Permissions* tab) for application refresh your token time-to-time (in
general *Settings* tab).

## Usage

After all preparations, you can run the application by typing in the terminal:

```bash
csu
```

Once started, you will be able to perform operations by providing necessary arguments to be executed.

Supported operations:

* `download`:
    * `from_path` - path to the local file
    * `to_path` - path to the destination file on the cloud storage
* `upload`:
    * `from_path` - path to the file on the local machine
    * `to_path`- path to the destination file on the cloud storage
* `delete`
    * `path` - path to the file on the cloud storage
* `list` - refreshes the list of local and cloud files in the working directories
* `clear` - clears log messages

## Problems:

Tangible is entering correct paths to files. For example, for cloud file it may be necessary `/` at the beginning (for
example, for file `photo.jpg` in root directory you need to write `/photo.jpg` as command argument).

## To implement:

* Make error messages more informative
* Ability to change local and cloud working directories
* Other file operations
* Implementations for other cloud storages (like [Google Drive])

[`ratatui`]: https://crates.io/crates/ratatui

[Dropbox]: https://www.dropbox.com

[Google Drive]: https://www.google.com/drive