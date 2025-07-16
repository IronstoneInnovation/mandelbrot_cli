![a Mandelbrot cat](mandelbrot_cat.png "Mandelbrot CLI")

# Mandelbrot CLI

_A Rust-powered Mandelbrot Set generator._

## Installation

Install Mandelbrot CLI with cargo:

```bash
$ cargo install mandelbrot_cli
```

## Usage

Generate the Mandelbrot Set with default settings:

```bash
$ mandelbrot_cli
```

Open the output file "out.png" and it will look like this:

![out.png](example_out.png "out.png")

Explore the Mandelbrot Set and refine your images using the available options:

```
  -s, --size <SIZE>                    [default: 2160]
  -x, --x-offset <X_OFFSET>            [default: 0]
  -y, --y-offset <Y_OFFSET>            [default: 0]
  -m, --magnification <MAGNIFICATION>  [default: 1]
  -i, --iterations <ITERATIONS>        [default: 100]
  -o, --output-path <OUTPUT_PATH>      [default: out.png]
  -h, --help                           Print help
```

# Gallery
