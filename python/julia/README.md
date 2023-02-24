Julia, or `jfg` more precisely (Julia fractal generator) is a software able to display any of the Julia fractals. It has a CLI and is not installable via PyPi.

#### Installation
The installation process is simple, and far from being the most perfomant, but it is as it is today. So, to install `jfg` you must clone it as is and use it as is.
```
git clone https://github.com/b4-b4/julia.git
```
#### Usage
To use `jfg`, you must write your command as follows:
```
python /path/to/julia/run.py <density-pixel> <detail-density> <cr> <ci> [-c THEME]
```
`<cr>` represents the real part of $c$ and `<ci>` its imaginary part.
All you really need to remember is this part:
```
... <density-pixel> <detail-density> <cr> <ci> [-c THEME]
```
You can also add `--save FILENAME` (or `-s`) to save the figure e.g.:
```
python run.py 500 300 -0.4 0.6 -c "inferno" -s "fig1.jpg"
```
Use `--help` for more informations about CLI.
