# gcovcheck

## Examples

commands for generating gcov data and pulling it back to local machine for
scale-product repo

generate data, run on buildvm

```bash
find $SRC_ROOT -name "*$1*.cpp" -exec dirname {} \; | sort -u | grep -v unittest | grep -v onboxtest | grep -v mockobjects | grep -v gen-cpp | grep -v gen_srcs | parallel "cd {}; echo \"generating coverage for {}\"; gcov -o $(targetdir) *.cpp >/dev/null 2&>1 && mv *.gcov /local/gcov 2>/dev/null"
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

Included in ale-files/ are two gcovcheck.vim scripts, after installing ale-vim
copy the linters one to the cpp directory of `ale_linters` in the plugin
directory and copy the handler one to the `handlers` directory. Then you'll need
to enable the 'gcovcheck' linter in ale with a line like this.

```vimscript
let g:ale_linters = { 'cpp' : ['rscmake', 'cppcheck', 'clangtidy', 'gcovcheck'], 'rust' : [] }
```
