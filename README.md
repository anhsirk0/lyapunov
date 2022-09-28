# Lyapunov fractal

[What is a Lyapunov fractal?](https://en.wikipedia.org/wiki/Lyapunov_fractal)

```bash
$ lyapunov
Generating Lyapunov fractal with following parameters.
domain:           0, 4
range:            0, 4
sequence:         ab
n (iterations):   200

Output image properties.
name:                      output.png
resolution:                1000x1000

Fractal generated
```
output.png
![output.png](https://github.com/anhsirk0/lyapunov/blob/master/output.png)

```bash
$ lyapunov -s ab -x -4,4 -y -4,4 -o ab.png
Generating Lyapunov fractal with following parameters.
domain:           -4, 4
range:            -4, 4
sequence:         ab
n (iterations):   200

Output image properties.
name:                      ab.png
resolution:                1000x1000

Fractal generated
```
ab.png
![ab.png](https://github.com/anhsirk0/lyapunov/blob/master/ab.png)

## Usage
```text
Lyapunov fractal generator

USAGE:
    lyapunov [OPTIONS]

OPTIONS:
    -h, --help                       Print help information
    -n, --n <N>                      Number of iterations for Lyapunov Exponent [default: 200]
    -o, --output <OUTPUT>            Output image [default: output.png]
    -r, --resolution <RESOLUTION>    Resolution of generated image [default: 1000x1000]
    -s, --sequence <SEQUENCE>        Sequence of 'a' & 'b' [default: ab]
    -V, --version                    Print version information
    -x, --x <X>                      Domain (x axis) min and max value [default: 0,4]
    -y, --y <Y>                      Range (y axis) min and max value [default: 0,4]
```
