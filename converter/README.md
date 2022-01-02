# converter

## Run

### Export output to window(s)

```bash
cargo run --release --features display-window -- [...other options]
```

### Export output to file(s)

```bash
cargo run --release -- [...other options]
```

## Build

```build
cargo build --release
```

> I do not recommend building with the `display-window` feature enabled. It server no other purpose apart from debugging.

## Usage / Examples

Running the help command:

```bash
cargo run -- help
```

Yields:

```text
converter 0.1.0
Virghileanu Teodor <@GaussianWonder>
CNC Converter

USAGE:
    converter [OPTIONS] <INPUT> [SUBCOMMAND]

ARGS:
    <INPUT>    Sets the input image to use

OPTIONS:
    -h, --high_threshold <FLOAT32>    Sets the high threshold for the Canny edge detector (<=1140.39) [default: 60.0]
        --help                        Print help information
    -l, --low-threshold <FLOAT32>     Sets the low threshold for the Canny edge detector (>=0) [default: 50.0]
    -o, --output <DIRECTORY PATH>     Sets a custom export path
    -V, --version                     Print version information

SUBCOMMANDS:
    export    controls export features
```

### Basic export configuring

Running:

```bash
cargo run -- export --help
```

Yields:

```text
converter-export 0.1.0
Virghileanu Teodor <@GaussianWonder>
controls export features

USAGE:
    converter <INPUT> export [OPTIONS]

OPTIONS:
    -d, --debug_preview <FLOAT32>    Exports the image with points traced on it. This comes with its own scale value for point precision. See point_precision for details
    -h, --help                       Print help information
    -i, --image                      Export image to the given export path
    -p, --p_precision <FLOAT32>      Exports edge points with a given precision. This is a scale factor for the initial image resolution
    -V, --version                    Print version information
```

#### Generate a GrayImage with edges

```bash
cargo run -- ./assets/test.jpg
```

#### Adjust min/max thresholds

```bash
cargo run -- ./assets/test.jpg -l 50 -h 60
```

#### Change output directory

```bash
cargo run -- ./assets/test.jpg -o ./assets/export
```

### 2D Point Generation

When generating 2D Points, you are required to expose the `-i` argument unless you don't want the GrayImage edges image to be exported.

#### Generate debug points

Generate the points, draw them on the original image, then export the resulting image:

```bash
cargo run -- ./assets/test.jpg -o ./assets/export export -d 0.50
```

> This exists for debugging / preview purposes.

#### Generate points

Generate the points, export them as JSON.

```bash
cargo run -- ./assets/test.jpg -o ./assets/export export -p 0.50
```

## Notes

- You can mix-match any `[OPTIONS]` with one another, none of them are mutual exclusive. This means that you can have any combination of `-i`, `-d`, `-p` in the export subcommand, and the files will be exported accordingly.
- `-p` and `-d` have independent values. Previews(Debug view) can be generated with a lower precision.
- When dealing with precision keep in mind that:
  - 1.0 precision takes each pixel as a point
  - 0.5 precision skips every other pixel
  - 0.3 precision skips every 3rd pixel
  - ...
