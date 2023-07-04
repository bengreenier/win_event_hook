# win_event_hook

[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/win_event_hook.svg
[crates-url]: https://crates.io/crates/win_event_hook
[docs-badge]: https://docs.rs/win_event_hook/badge.svg
[docs-url]: https://docs.rs/win_event_hook
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE
[actions-badge]: https://github.com/bengreenier/win_event_hook/workflows/CI/badge.svg
[actions-url]: https://github.com/bengreenier/win_event_hook/actions?query=workflow%3ACI

A safe Rust API for using [`SetWinEventHook`](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwineventhook), powered by the [`windows`](https://crates.io/crates/windows) crate.

## Usage

To use `win_event_hook`, add the following to your `Cargo.toml`:

```toml
[dependencies]
win_event_hook = "0.1"
```

Then create a configuration and install a hook, for example:

```rust
// create our hook config
let config = win_event_hook::Config::builder()
    .skip_own_process()
    .with_dedicated_thread()
    .with_events(vec![
        // to see these, try right clicking
        Event::Named(NamedEvent::ObjectShow),
        Event::Named(NamedEvent::ObjectHide),
        // to see this, try moving around the cursor
        Event::Named(NamedEvent::ObjectLocationChange),
    ])
    .finish();

// and our handler
let handler = |ev, _, _, _, _, _| {
    println!("got event: {:?}", ev);
};

// install the hook
let hook = win_event_hook::WinEventHook::install(config, handler)?;
```

When `hook` is [dropped](https://doc.rust-lang.org/std/ops/trait.Drop.html), an uninstall is attempted automatically. Uninstallation may fail - to handle failures, instead call [`uninstall`](https://docs.rs/win_event_hook/latest/win_event_hook/struct.WinEventHook.html#method.uninstall) yourself, for example:

```rust
// building on the above example

// uninstall the hook
hook.uninstall()?;
```

For more information, see [the generated documentation](https://docs.rs/win_event_hook).

## LICENSE

This project is licensed under the [MIT license](LICENSE).
