# CNC Converter Simulator

This is a simulator for the CNC convertor package.

## RUN

```bash
cargo run --release
```

## Usage

Make sure there is an assets folder in the current directory, with at least one image the following folder structure

```text
assets\
  export\
    edges\
  some_random_named_image.{jpg|png|jpeg}
```

When applicable, the assets folder will be filled with debug data from the conversion program, and on the screen a simulation of a CNC machine will be displayed.