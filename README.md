# editor.md

```bash
$ cargo build
```

## Prerequisites

- On Linux, ensure you have a native file dialog backend installed (e.g., `zenity` or `kdialog`), since the `rfd` crate relies on these. On Windows and macOS, the native dialogs work out of the box.
  ```bash
  sudo apt install zenity    # or kdialog
  ```

## Running

```bash
cargo run --release
```