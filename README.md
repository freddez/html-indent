html-indent
-----------

`html-indent` is a html file indenter and checker. 

It can be used for a single file, an entire tree, or from stdin (for editor
plugin integration). 

It supports multi-line tags, and preserves relative indentation in scripts and
comments sections. Except spaces, the document structure is preserved and the
only validation check is for balanced tags.

### Command line options

```
Usage: html-indent [FILE] [options]

Options:
    -h, --help          print this help menu
    -r, --recursive     process all files in directory tree
    -e, --extension ext file extension for recursive processing
    -n, --dry-run       dry run, don't write files
        --numeric       output indentation value
    -l, --lines [start]-[end]
                        limit output to selected lines
    -p, --print         print html result to stdout
```


### Installation

`html-indent` is written in [Rust](http://rust-lang.org/). For the moment the
only option is to build it from source, so [install Rust](https://rustup.rs/)
then type

```
$ cargo install
```

### Editor integration

There is a [Vim](http://www.vim.org/) plugin under `tools`
directory. Just drop it in `~/.vim/indent/` and don't forget to have 
```
filetype plugin indent on
```
in your `.vimrc`.

### Known alternatives

* [HTML Tidy](http://www.html-tidy.org/) : Tidy is designed to clean-up html
  complete documents and I didn't succeed to indent html fragments with the
  command line version.
* [GNU Emacs](https://www.gnu.org/software/emacs/)  ~~editor~~ ~~universe~~
  environment. The included **html-mode** and **web-mode**, like many
  **Emacs**'s major modes have implemented **indent-region** function.
* [Atom](https://atom.io/) editor. Unfortunately, the indentation doesn't work
  with newlines inside tags.


