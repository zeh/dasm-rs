echo ""
echo "Statistics:"
echo \* ftohex - $(LC_ALL=en_US.UTF-8 numfmt --grouping $(du --apparent-size --block=1 bin/ftohex | grep -o -i --regexp='[0-9]*'))b
echo \* dasm - $(LC_ALL=en_US.UTF-8 numfmt --grouping $(du --apparent-size --block=1 bin/dasm | grep -o -i --regexp='[0-9]*'))b
echo \* "libc::" - $(grep -o -r 'libc::' src | wc -l)
echo \* "unsafe" - $(grep -o -r 'unsafe' src | wc -l)
echo \* "memcpy" - $(grep -o -r 'memcpy' src | wc -l)
echo \* "malloc" - $(grep -o -r '\bmalloc' src | wc -l)
echo \* "no_mangle" - $(grep -o -r 'no_mangle' src | wc -l)
echo \* "transient_" - $(grep -o -r 'transient_' src | wc -l)
echo \* "FIXME:" - $(grep -o -r 'FIXME:' src | wc -l)
echo ""
