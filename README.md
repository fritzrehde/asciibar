# asciibar

[![Build status](https://github.com/fritzrehde/asciibar/actions/workflows/ci.yml/badge.svg)](https://github.com/fritzrehde/asciibar/actions)
[![Version info](https://img.shields.io/crates/v/asciibar)](https://crates.io/crates/asciibar)

*Visualize percentages in the terminal with ascii progress bars.*

![screenshot](https://raw.githubusercontent.com/fritzrehde/i/master/watchbind/screenshot-light.png#gh-light-mode-only)
![screenshot](https://raw.githubusercontent.com/fritzrehde/i/master/watchbind/screenshot-dark.png#gh-dark-mode-only)

## Features

- **Customizable**: every character that is printed can also be customized via cli options
- **Speed**: written completely in rust with speed in mind


## Installation

### From [crates.io](https://crates.io/crates/asciibar)

```shell
cargo install asciibar
```

### AUR

To be added.


## How to use

`asciibar` is very simple and designed to be integrated into shell scripts.

### Specifying the percentage

You can provide the percentage with `asciibar <PERCENTAGE>`.

Percentages are usually represented either by a float value between 0 and 1 or between 0 and 100.
Therefore, we can call 0 and 1 (or 0 and 100) the minimum and maximum values of the percentage scale.
`asciibar` will automatically guess whether these default ranges are used.

For example, `asciibar 0.1` will assume a scale from 0 to 1 and `asciibar 10` will assume a scale of 0 to 100.
However, you can also specify these minimum and maximum values yourself with `--min` and `--max`.
Note that you will have to use the format `asciibar --[min|max]=-42 -- -10` when specifying negative values, otherwise the argument parser might mistake a negative value for an option.
For example, if you want to visualize a percentage of 0.1 on a scale of 0 to 100, you would have to specify `asciibar --min 0 --max 100 0.1`, otherwise `asciibar` will assume the default scale of 0 to 1.

### Printing to stdout

`asciibar` will print three "blocks" to stdout:
1. the "filled" block (customize with `--filled <CHAR>`), which represents the percentage that "has been completed" (i.e. the left block).
1. the "empty" block (customize with `--empty <CHAR>`), which represents the percentage that "has not yet been completed" (i.e. the right block).
1. the "half-filled" block (customize with `--half-filled <CHAR>`) represents a unique middle block that is supposed to add precision to the bar. It has been added as an extra (optional) level of segmentation between the "filled" and "empty" blocks. It will not always be printed (e.g. if the percentage can be displayed exactly with `asciibar 0.5`), but only when precision can be added (e.g. with `asciibar --length 10 0.55`).

This example output of `asciibar` might help understand which blocks are which:
```
filled   half-filled   empty
  |          |           |
=============>--------------
```

**Note:**
The default "half-filled" block is only printed if no customizing options are supplied (i.e. if all default chars are used).
If only "filled" and/or "empty" are provided, the block that would usually be a "half-filled" block will use the "empty" character.
The reason for this is that a default "half-filled" block might look very weird in between customized "filled" and "empty" blocks.


### Characters

You can provide the `--filled`, `--half-filled` and `--empty` options any char you would like to display.
Here is a list of combinations that I personally like (the first line represents the defaults in `asciibar`):

filled | half-filled | empty | example output
:-: | :-: | :-: | :-:
`█` | `▌` | ` ` | `███▌   `
`=` | `>` | `-` | `===>---`
`#` | none | `=` | `###====`
`█` | none | `░` | `███░░░░`
`█` | none | `▒` | `███▒▒▒▒`
`█` | none | `▓` | `███▓▓▓▓`

### Length

You can specify how long/wide you want the output to be with `--length` (the default is 10 characters).

### Miscellaneous

These last two features are possible to do in a normal shell script, but have been added as quality-of-life improvements.
You can add a custom border character with `--border` that is added before and after the bar chart.
You can also specify the bar chart to end in a newline character with `--newline`.


## Example usage
```sh
# uses default chars for filled '█', half-filled '▌' and empty ' '
asciibar 0.55
█████▌    

# custom border
asciibar --border="|" 0.55
|█████▌    |

# custom filled and empty chars
asciibar --filled="░" --empty="█" 0.5
░░░░░█████

# custom length
cargo run -- --filled "=" --half-filled ">" --empty "-" --length 20 -- 0.58
===========>--------
```
