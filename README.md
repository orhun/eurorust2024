# EuroRust 2024 Presentation 🦀

**Session**: [Renaissance of Terminal User Interfaces with Rust](https://eurorust.eu/talks/renaissance-of-terminal-user-interfaces-with-rust)

## Presenting

To start the presentation, first install [presenterm](https://github.com/mfontanini/presenterm):

```bash
cargo install --git https://github.com/mfontanini/presenterm
```

Update submodules:

```bash
git submodule update --init --recursive
```

Then simply run:

```bash
presenterm presentation.md -X -c config.yml -p
```

> [!TIP]  
> Or you can use the [`present.sh`](./present.sh) script.

> [!IMPORTANT]  
> It is recommended to use [wezterm](https://github.com/wez/wezterm) for good image rendering.
