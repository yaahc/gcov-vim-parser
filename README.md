# gcovcheck

[![Build Status](https://travis-ci.org/jrlusby/gcov-vim-parser.svg?branch=master)](https://travis-ci.org/jrlusby/gcov-vim-parser)

## Necessary commands

For generating gcov data and pulling it back to local machine for
scale-product repo

generate data, run on buildvm

```bash
find "$SRC_ROOT" -name "*$1*.cpp" -exec dirname {} \; |
    sort -u |
    grep -v "$1unittest" |
    grep -v onboxtest |
    grep -v mockobjects |
    grep -v gen-cpp |
    grep -v gen_srcs |
    parallel "cd {}; \
                echo \"generating coverage for {}\"; \
                gcov -o $(targetdir) *.cpp >/dev/null 2>&1 && \
                mv *$1*.gcov /local/gcov 2>/dev/null"
```

copy data back to local machine

```bash
rsync -azh -e ssh $BUILDVM_HOSTNAME:/local/gcov/ ~/gcov
```

## Setup

### easy version, vim quickfix list

To run populate quickfix list and jump to first definition, use :cnext or ]q if
you have the unimpaired plugin installed to go to next issue

```vimscript
nnoremap <leader>c :cexpr system('gcovcheck --vimgrep ' . shellescape(expand('%:p')))<CR>
```

### nice version, using vim-ale

install as a vim plugin to enable ale / dispatch support

```vimscript
Plug 'jrlusby/gcov-vim-parser', { 'do': './install.sh' }

Plug 'tpope/vim-dispatch'
nnoremap <leader>d :Dispatch<CR>
nnoremap <leader>c :Copen<CR>
nnoremap <leader>C :Dispatch covrun %:p<CR>
```

```vimscript
Plug 'w0rp/ale' " You can install ale what ever way you want, heres an example
                " using vim-plugged

" important line enabling gcovcheck
let g:ale_linters = {
    \ 'cpp' : ['rscmake', 'cppcheck', 'clangtidy', 'gcovcheck'],
    \ 'rust' : [],
    \ }

" optional config that I use
let g:ale_echo_msg_format = '%code: %%s %linter%'
let g:ale_sign_info = 'X'
highlight link ALEInfoSign ALEInfo
```
