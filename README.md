# gcovcheck

[![Build Status](https://travis-ci.org/jrlusby/gcov-vim-parser.svg?branch=master)](https://travis-ci.org/jrlusby/gcov-vim-parser)

## Dependencies

* scale-scripts: internal repo on my buildvm, ask for a clone URL.
* rust toolchain: use <https://rustup.rs> to install rust if you dont already
  have it

## Setup

### using vim-ale for gutter markers

install as a vim plugin to enable ale / dispatch support

```vim
Plug 'w0rp/ale'
Plug 'yaahallo/gcov-vim-parser', { 'do': './install.sh' }

" important line enabling gcovcheck
let g:ale_linters = {
    \ 'cpp' : ['rscmake', 'cppcheck', 'clangtidy', 'gcovcheck'],
    \ }

" This may also work, untested, either way it makes it clear what you need
" add(g:ale_linters.cpp, 'gcovcheck')

" optional config that I use
let g:ale_echo_msg_format = '%code: %%s %linter%'
let g:ale_sign_info = 'X'
highlight link ALEInfoSign ALEInfo
```

### minimal dependencies, for quickfix list population

To run populate quickfix list and jump to first definition, use :cnext or ]q if
you have the unimpaired plugin installed to go to next issue

```vim
nnoremap <leader>c :cexpr system('gcovcheck --vimgrep ' . shellescape(expand('%:p')))<CR>
```

## Usage

Run unittests via scunit, or any wrapper to scunit like covrun. Fundamentally
gcovcheck looks for .gcov files in the `~/gcov` directory. So any script that
runs unit tests, generates gcov files, and copies them to that directory will
suffice.

With the included configuration files ALE will run gcovcheck whenever you save
or open a file, parsing the current text and the copy of the gcov file in
`~/gcov` and will display whatever coverage lines it finds that are still
valid. **It does not by default run unit tests or pull new coverage files to
your local filesystem**, it expects you to do this manually, the included
optional config for setting up vim dispatch is how I do this.

```vim
" Example program used to run covrun script for current buffer (optional)
Plug 'tpope/vim-dispatch'
nnoremap <leader>C :Dispatch covrun %:p<CR>
```

The covrun script takes the current filename, finds the appropriate unit test,
even if you're editing the unit test or the corresponding .cpp/.h source files,
and runs that unit test then runs gcovcheck. There is an included vim compiler
plugin for covrun that enables error parsing for gcovcheck and gtest unit
tests, which if used in conjunction with vim-dispatch allows you to jump
directly to lines in unit tests where errors occured.
