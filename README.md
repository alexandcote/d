## What is `d`?

`d` is a tool to standardize your projects structure on your local machine.

## Installation

```bash
eval "$(curl -sSL https://bit.ly/2Kzj4Xd)"
```

Add this in your `.bashrc` or `.zshrc` file.

```bash
if [ -f /opt/d/d.sh ]; then
  source /opt/d/d.sh
fi
```

## Usage

![Usage example](https://github.com/alexandcote/d/blob/main/screenshots/demo.gif?raw=true)

You can easily clone a new git repository by running:
```
d clone https://github.com/alexandcote/d
```
`d` will clone the repository in `~/src/github.com/alexandcote/d`.

You can use `d` to navigate between your projects by running: 
```
d cd alexandcote/d
```
`d` use fuzzy finder to find the right directory so you can simply run:

```
d cd ad
```