echo ""
echo "Statistics:"
echo \* ftohex - $(LC_ALL=en_US.UTF-8 numfmt --grouping $(du --apparent-size --block=1 bin/ftohex | grep -o -i --regexp='[0-9]*'))b
echo \* dasm - $(LC_ALL=en_US.UTF-8 numfmt --grouping $(du --apparent-size --block=1 bin/dasm | grep -o -i --regexp='[0-9]*'))b
echo \* "libc::" - $(grep -o -r 'libc::' src | wc -l)
echo \* "unsafe" - $(grep -o -r 'unsafe' src | wc -l)
echo \* "extern" - $(grep -o -r 'extern' src | wc -l)
echo \* "memcpy" - $(grep -o -r 'memcpy' src | wc -l)
echo \* "malloc" - $(grep -o -r '\bmalloc' src | wc -l)
echo \* "*mut/*const" - $(grep -o -r -E '\*(mut|const)' src | wc -l)
echo \* "*printf" - $(grep -o -r 'printf' src | wc -l)
echo \* "no_mangle" - $(grep -o -r 'no_mangle' src | wc -l)
echo \* "transient::" - $(grep -o -r 'transient::' src | wc -l)
echo \* "FIXME:" - $(grep -o -r 'FIXME:' src | wc -l)
echo \* LOC - $(find ./src -name '*.rs' -print0 | xargs -0 cat | wc -l)
echo ""
