
![alt tag](img/Firefly%20Create%20a%20logo%20inspired%20by%20the%20Hitman%20video%20game%20series.%20The%20logo%20should%20capture%20the%20sleek,%20s%20(8).jpg)

# Hitman

`hitman` is a command-line tool written in Rust that extracts IPv4, IPv6 addresses, and fully qualified domain names (FQDNs) from a text file. It uses the `clap` crate for command-line argument parsing and the `ripgrep` crate for efficient regular expression searching.

## Features

- Extract IPv4 addresses
- Extract IPv6 addresses
- Extract FQDNs
- Input via file or standard input
- Output to screen or file
- Quiet mode for suppressing stdout output when writing to a file

## Installation

1. Ensure you have Rust and Cargo installed. If not, install them from [here](https://www.rust-lang.org/tools/install).
2. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/hitman.git
   cd hitman
   ```
3. Build the project:
   ```sh
   cargo build --release
   ```

## Usage

### Basic Usage

You can use `hitman` by providing a text file as input or by piping text via standard input.

#### From a File

```sh
cargo run -- --file sample.txt
```

#### From Standard Input

```sh
cat sample.txt | cargo run
```

### Output to a File

To save the extracted results to a file:

```sh
cargo run -- --file sample.txt --out output.txt
```

### Quiet Mode

To suppress output to the screen when saving results to a file, use the `-q` flag:

```sh
cargo run -- --file sample.txt --out output.txt -q
```

## Command-Line Options

- `-f, --file <FILE>`: Specify the input file to read.
- `-o, --out <FILE>`: Specify the output file to write results.
- `-q, --quiet`: Quiet mode, no output to stdout.

## Example

```sh
# Extract from a file and display on screen
cargo run -- --file sample.txt

# Extract from a file and save to output.txt
cargo run -- --file sample.txt --out output.txt

# Extract from standard input and display on screen
cat sample.txt | cargo run

# Extract from standard input and save to output.txt quietly
cat sample.txt | cargo run -- --out output.txt -q
```

## License

This project is licensed under the GPL-3.0 license. See the [LICENSE](LICENSE) file for details.