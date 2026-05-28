# Myrnote
Myrnote is a CLI for annotating and reading text.

## Installation

```bash
# Clone the repository
git clone https://github.com/gabrielsh06/myrnote.git

# Move into the directory
cd myrnote

# Install the CLI tool globally on your system
cargo install --path .
```

## Usage

Here are the basic commands to use `myrnote`:

### Write a note
```bash
myrnote
# This will give you the option to enter the text you type.
```
### Read a note
```bash
myrnote --list
# This lists all the text you have saved.
```
### Clear a note
```bash
myrnote --clear
# ⚠️ WARNING: This clears everything you have saved
```

## License
[GPLv3](https://www.gnu.org/licenses/gpl-3.0.html)
