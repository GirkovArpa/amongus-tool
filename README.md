# Among Us Tool

A helpful graphical overlay to help you keep track of potential impostors in Among Us.

Made with [Sciter](https://sciter.com).

![Animated Screencapture](screencapture.gif)

## Building

You need the Rust toolchain.

If you're not building for Windows, remove this line from `main.rs`:

```rust
#![windows_subsystem = "windows"]
```

It's purpose is to hide the console window that would otherwise pop up.  I'm not sure what the corresponding directive is for other operating systems.

```
$ cargo build
```

When running the executable, the appropriate Sciter library must be included in the same folder.

In this case, I've included the 64bit `sciter.dll`.

If this doesn't match your system, you can find the one that does in the [Sciter SDK repository](https://github.com/c-smile/sciter-sdk). Look in the folders that begin with `bin.`; e.g. `bin.win/x64/sciter.dll`.