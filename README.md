# Example

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
