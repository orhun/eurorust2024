---
title: Renaissance of Terminal User Interfaces with Rust
sub_title: EuroRust 2024
author: Orhun Parmaksız
theme:
  override:
    footer:
      style: template
      left: "gh/@orhun"
      right: "{current_slide}"
    slide_title:
      alignment: center
      padding_top: 1
---

# Who dat?

<!-- column_layout: [5, 5, 1] -->

<!-- column: 0 -->

![](assets/orhun.png)

<!-- column: 1 -->

<!-- new_lines: 1 -->

## _Orhun Parmaksız_

<!-- pause -->

✨ Open Source Connoisseur (`github.com/orhun`)

🦀 Creator of **git-cliff**, kmon, gpg-tui, systeroid…

🐭 Project Leader @ **Ratatui**

📦 Package Maintainer @ Alpine & **Arch Linux** (btw)

`https://orhun.dev`

<!-- column: 2 -->

<!-- new_lines: 1 -->

![image:width:40%](assets/arch.png) ![image:width:40%](assets/ratatui.png) ![image:width:40%](assets/git-cliff.png)

<!-- end_slide -->

# VT100

![image:width:50%](assets/vt100.png)

<!-- pause -->

<!-- column_layout: [1, 2, 1] -->

<!-- column: 1 -->

> The VT100 terminal follows two programming standards - American National Standards Institute (ANSI) and VT52. In ANSI mode, the VT100 generates and responds to coded sequences per ANSI standards X3.41-1974 and X3.64-1977. In VT52 mode, the VT100 terminal is compatible with previous DIGITAL software using the VT52 video terminal.

<!-- end_slide -->

## VT100 Art

```bash +exec
mpv --quiet --loop-file /home/orhun/gh/eurorust2024/assets/twilight.mp4
```

<!-- pause -->

These are VT100 Animation files, which are meant to be thrown up raw at a vt100-compatible terminal.

[http://artscene.textfiles.com/vt100/](http://artscene.textfiles.com/vt100/)

| Animation                        | File Size | Description                                             |
| -------------------------------- | --------- | ------------------------------------------------------- |
| [bambi.vt](bambi.vt)             | 12818     | VT100 ANIMATION: Bambi vs. Godzilla                     |
| [dirty.vt](dirty.vt)             | 25081     | VT100 ANIMATION: Someone Having an Awful Amount of Fun  |
| [globe.vt](globe.vt)             | 29696     | VT100 ANIMATION: ABSOLUTELY EXCELLENT Spinning Globe    |
| [monkey.vt](monkey.vt)           | 51041     | VT100 ANIMATION: The Monkey Gives You The Finger        |
| [movglobe.vt](movglobe.vt)       | 250452    | VT100 ANIMATION: Incredible Spinning, Moving Globe      |
| [outerlimits.vt](outerlimits.vt) | 63832     | VT100 ANIMATION: The Outer Limits                       |
| [pac3d.vt](pac3d.vt)             | 17248     | VT100 ANIMATION: Pac Man in 3-D Chomping a Ghost        |
| [peace.vt](peace.vt)             | 79456     | VT100 ANIMATION: Imagine World Peace by John G. Poupore |

<!-- pause -->

<!-- new_lines: 2 -->

```sh
\033[4;9H\033[1mE\033[0m\033[12;69Hq\033[12;80Hq
\033[4;8H\033[1mR\033[0m\033[12;56Hq\033[12;53Hq
\033[4;5H\033[1mT\033[0m\033[12;75Hq\033[12;30Hq
\033[4;12H\033[1mS\033[0m\033[12;42Hq\033[12;27Hq
\033[4;11H\033[1mI\033[0m\033[12;28Hq\033[12;58Hq
```

<!-- column_layout: [1, 1, 1] -->

<!-- column: 1 -->

_Example ANSI escape sequences used above_

<!-- end_slide -->

## Turbo Vision

![image:width:50%](assets/turbo-vision.png)

<!-- end_slide -->

## DOS Navigator

![image:width:50%](assets/dos-navigator.png)

<!-- end_slide -->

# TUIs

<!-- column_layout: [1, 1] -->

<!-- column: 1 -->

![image:width:40%](assets/ratcopter.gif)

<!-- column: 0 -->

<!-- pause -->

- 🔡 Made using text characters.

<!-- pause -->

```plaintext
┌─┬┐  ╔═╦╗  ╓─╥╖  ╒═╤╕
│ ││  ║ ║║  ║ ║║  │ ││
├─┼┤  ╠═╬╣  ╟─╫╢  ╞═╪╡
└─┴┘  ╚═╩╝  ╙─╨╜  ╘═╧╛
┌───────────────────┐
│  ╔═══╗ Some Text  │▒
│  ╚═╦═╝ in the box │▒
╞═╤══╩══╤═══════════╡▒
│ ├──┬──┤           │▒
│ └──┴──┘           │▒
└───────────────────┘▒
 ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
```

<!-- pause -->

````rust
pub const ONE_EIGHTH_TOP_EIGHT: &str = "▔";
pub const ONE_EIGHTH_BOTTOM_EIGHT: &str = "▁";
pub const ONE_EIGHTH_LEFT_EIGHT: &str = "▏";
pub const ONE_EIGHTH_RIGHT_EIGHT: &str = "▕";

/// Wide border set based on McGugan box technique
///
/// ```text
/// ▁▁▁▁▁▁▁
/// ▏xxxxx▕
/// ▏xxxxx▕
/// ▔▔▔▔▔▔▔
````

<!-- end_slide -->

# TUIs

<!-- column_layout: [1, 1] -->

<!-- column: 1 -->

![image:width:40%](assets/ratcopter.gif)

<!-- column: 0 -->

- 💻 Designed to run in a terminal or console.

<!-- pause -->

like this presentation! 🤡

<https://github.com/mfontanini/presenterm>

<!-- pause -->

```bash +exec
alacritty -qq -e btm
```

<!-- pause -->

- ⚡ Efficient and lightweight.

<!-- pause -->

**blazingly fast???** 🦀

<!-- end_slide -->

# GUIs

<!-- pause -->

> _bloat_

<!-- pause -->

<!-- column_layout: [1, 1, 1] -->

<!-- column: 0 -->

![image:width:100%](assets/bloat1.png)

<!-- pause -->

<!-- column: 1 -->

![image:width:80%](assets/bloat2.png)

<!-- pause -->

<!-- column: 2 -->

![](assets/bloat3.png)

<!-- end_slide -->

# GUIs

> _bloat_

<!-- column_layout: [1, 1] -->

<!-- column: 1 -->

![image:width:50%](assets/sadness.png)

<!-- column: 0 -->

1. Higher complexity / excessive features.
2. Complex interfaces.
3. Performance issues.

<!-- pause -->

<!-- new_lines: 1 -->

![image:width:70%](assets/electron.png)

<!-- end_slide -->

# Rust & TUIs

![](assets/tui-rs.png)

<!-- column_layout: [1, 1, 1] -->

<!-- column: 1 -->

<https://github.com/fdehau/tui-rs>

<!-- reset_layout -->

<!-- pause -->

![](assets/initial-commit.gif)

<!-- end_slide -->

# Rust & TUIs

```bash +exec
alacritty -qq -e kmon
```

<!-- column_layout: [1, 1, 1] -->

<!-- column: 1 -->

<https://github.com/orhun/kmon>

<!-- reset_layout -->

## But then...

> Florian Dehau (author of `tui-rs`):
>
> Some of you may have noticed that my activity as the maintainer of `tui-rs` has significantly decreased
> over the past year. There are multiple reasons for that: my job is taking most of my creative coding energy
> (for good reasons), I don't use Rust enough (even though I miss it), and I have not used `tui-rs` for
> personal/work projects in a while, so my motivation to dedicate time to it is pretty low. None of that is
> likely going to change in the near future. At the same time, the popularity of the crate keeps growing, and
> I keep seeing it used in very cool projects. So it would be a shame if all of that goes to waste. As such,
> I'm currently thinking of ways to either find more maintainers or pass the torch to others who can better
> support the community and keep the project alive and thriving.

<https://github.com/fdehau/tui-rs/issues/654>

<!-- end_slide -->

## Timeline

| Date       | Event                                                             |
| ---------- | ----------------------------------------------------------------- |
| 14-08-2022 | Discussion on the future of `tui-rs` begins.                      |
| 02-02-2023 | Discord server created to explore forking the project.            |
| 08-02-2023 | Original author proposes a plan for transferring ownership.       |
| 14-02-2023 | Fork created to continue development (`tui-rs-revival`).          |
| 18-02-2023 | First Ratatui meeting held.                                       |
| 19-03-2023 | Ratatui's first version released.                                 |
| 01-04-2023 | Second Ratatui meeting.                                           |
| 29-05-2023 | Ratatui 0.21.0 released.                                          |
| 15-07-2023 | Biggest Ratatui meeting to date!                                  |
| 17-07-2023 | Ratatui 0.22.0 released.                                          |
| 07-08-2023 | `tui-rs` archived, **Ratatui** becomes the official successor! 🎉 |

<!-- pause -->

### `RUSTSEC-2023-0049`

tui is unmaintained; use ratatui instead

> The `tui` crate is no longer maintained.
>
> Consider using the `ratatui` crate instead.

<!-- end_slide -->

## Renaissance

![image:width:50%](assets/ratatui-text-logo.png)

---

<!-- column_layout: [1, 1, 1] -->

<!-- column: 1 -->

🐁 https://ratatui.rs

⭐ https://github.com/ratatui

🌐 https://forum.ratatui.rs

<!-- reset_layout -->

<!-- pause -->

### The Vision

1\. Ease of Use

> Improving usability by addressing key issues like widget tracking, scrollable widgets, and simplifying examples.

2\. Visual Appeal

> Improving aesthetics and customization options to make Ratatui more visually appealing and attention-grabbing.

<https://github.com/ratatui/ratatui/issues/1321>

![](assets/rat.gif)

<!-- end_slide -->

## Demo

```bash +exec
alacritty -qq --working-directory /home/orhun/gh/ratatui -e \
  cargo run --example demo2 --features crossterm,palette,widget-calendar
```

<!-- end_slide -->

### Development

![image:width:20%](assets/ratcopter.gif)

```bash
$ cargo install cargo-generate

$ cargo generate ratatui/templates
```

<!-- pause -->

<!-- new_lines: 1 -->

```bash +exec
alacritty -qq --working-directory \
  /home/orhun/gh/ratatui-templates/simple-generated -e $EDITOR .
```

<!-- end_slide -->

### Minimal Example

````rust-script {1-25|1-5|7-8|9-15|17-22|24|1-25} +line_numbers +exec
//! ```cargo
//! [dependencies]
//! ratatui = "0.28.1"
//! ```

use ratatui::{
    crossterm::event::{self, Event},
    text::Text,
    Frame,
};

fn main() {
    let mut terminal = ratatui::init();
    loop {
        terminal
            .draw(|frame: &mut Frame| {
                frame
                    .render_widget(Text::raw("Hello World!"), frame.area())
            })
            .expect("failed to draw frame");

        if matches!(
            event::read().expect("failed to read event"),
            Event::Key(_)
        ) {
            break;
        }
    }
    ratatui::restore();
}
````

<!-- end_slide -->

## Concepts

### Rendering

### Widgets

### Dynamic Layouts

### Miscellaneous

#### Logging

#### Async

## Showcase
