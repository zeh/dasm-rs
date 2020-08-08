#!/usr/bin/env bash

# Simple test script adapted from Matt Dillon's 2.16  release.
# It's harder to do this in a Makefile, so let's use a script.

# Ran without parameters, this compares existing files to reference files.
# Ran with parameters, this generates reference files, e.g.:
#   ./run_tests.sh ../bin-original/dasm .ref

DASM=${1:-../bin/dasm}
SUFFIX=$2

# 1. General cleanup

echo "Cleaning up"
echo

if [[ -z "$SUFFIX" ]]; then
  echo "... Removing all previously generated files"
  rm ./*.txt
  rm ./*.bin
  rm ./*.hex
else
  echo "... Removing all previously generated files with suffix $SUFFIX"
  rm "./*$SUFFIX"
fi
echo

# 2. Standard test cases

echo "Running standard build test cases (strict off)"
echo

errors=0

for i in *.asm; do
  i="${i//.bin.ref/.asm}"
  NAME=$(basename "$i" .asm)

  # Compile .asm into .bin and .list.txt
  $DASM "$i" -f1 -o"$NAME.bin$SUFFIX" -l"$NAME.list.txt$SUFFIX" -DINEEPROM 2>&1 | \
    tee "$NAME.stdout.txt$SUFFIX" >/dev/null
    # | \grep -vE 'error|Complete|Fatal|Warning^?'

  # Generate .hex file from .bin
  ../bin/ftohex 1 "$NAME.bin$SUFFIX" "$NAME.hex$SUFFIX"

  # Display results
  echo -n "  * ${NAME}: "

  if [[ -z "$SUFFIX" ]]; then
    cmp -s "$NAME.bin" "$NAME.bin.ref"
    if [ $? == 0 ]
    then
      echo -n "bin pass; "
    else
      echo -n "bin FAIL; "
      errors=$((errors + 1))
    fi
    cmp -s "$NAME.hex" "$NAME.hex.ref"
    if [ $? == 0 ]
    then
      echo -n "hex pass; "
    else
      echo -n "hex FAIL; "
      errors=$((errors + 1))
    fi
    cmp -s "$NAME.list.txt" "$NAME.list.txt.ref"
    if [ $? == 0 ]
    then
      echo -n "list pass; "
    else
      echo -n "list FAIL; "
      errors=$((errors + 1))
    fi
    cmp -s "$NAME.stdout.txt" "$NAME.stdout.txt.ref"
    if [ $? == 0 ]
    then
      echo -n "stdout pass. "
    else
      echo -n "stdout FAIL. "
      errors=$((errors + 1))
    fi
  else
    echo -n "generated with suffix $SUFFIX"
  fi
  echo
done

echo
if (( errors > 0 )); then
  echo "...$errors errors detected."
else
  echo "...no errors detected."
fi
echo

# 3. Test cases where an error is expected

echo "Running error build test cases (strict on)"
echo

errors=0

for i in *.fail; do
  i="${i//.fail/.asm}"
  NAME=$(basename "$i" .asm)

  # Compile .asm into .bin and .list.txt
  $DASM "$i" -S -f1 -o"$NAME.strict.bin$SUFFIX" -l"$NAME.list.strict.txt$SUFFIX" -DINEEPROM 2>&1 | \
    tee "$NAME.stdout.strict.txt$SUFFIX" >/dev/null
    # | \grep -vE 'error|Complete|Fatal|Warning^?'

  # Display results
  echo -n "  * ${NAME}: "

  if [[ -z "$SUFFIX" ]]; then
    grep error "$NAME.list.strict.txt" 2>&1 >/dev/null
    if [ $? == 0 ]
    then
      echo -n "error trigger pass; "
    else
      echo -n "error trigger FAIL; "
      errors=$((errors + 1))
    fi
    cmp -s "$NAME.list.strict.txt" "$NAME.list.strict.txt.ref"
    if [ $? == 0 ]
    then
      echo -n "list pass; "
    else
      echo -n "list FAIL; "
      errors=$((errors + 1))
    fi
    cmp -s "$NAME.stdout.strict.txt" "$NAME.stdout.strict.txt.ref"
    if [ $? == 0 ]
    then
      echo -n "stdout pass; "
    else
      echo -n "stdout FAIL; "
      errors=$((errors + 1))
    fi
  else
    echo -n " generated with suffix $SUFFIX"
  fi
  echo
done

echo
if (( errors > 0 )); then
  echo "...$errors errors detected."
else
  echo "...no errors detected."
fi
echo

# 4. Special cases

echo "Running special build test cases"
echo

custom_files=(
  "atari2600/boing26.asm"
  "atari7800/spritesample.asm"
  "channel-f/lights.asm"
  "channel-f/tetris.asm"
)
custom_params=(
  "-f3 -I../machines/atari2600/"
  "-f3 -I../machines/atari7800/"
  "-f3 -I../../machines/channel-f/"
  "-f3 -I../../machines/channel-f/"
)

errors=0

for ((i = 0; i < ${#custom_files[@]}; i++)); do
  FILE="${custom_files[i]}"
  NAME="${FILE//.asm/}"

  # Compile .asm into .bin and .list.txt
  $DASM "$NAME.asm" ${custom_params[i]} -o"$NAME.bin$SUFFIX" -l"$NAME.list.txt$SUFFIX" -DINEEPROM 2>&1 | \
    tee "$NAME.stdout.txt$SUFFIX" >/dev/null
    # | \grep -vE 'error|Complete|Fatal|Warning^?'

  # Generate .hex file from .bin
  ../bin/ftohex 1 "$NAME.bin$SUFFIX" "$NAME.hex$SUFFIX"

  # Display results
  echo -n "  * ${NAME}: "

  if [[ -z "$SUFFIX" ]]; then
    cmp -s "$NAME.bin" "$NAME.bin.ref"
    if [ $? == 0 ]
    then
      echo -n "bin pass; "
    else
      echo -n "bin FAIL; "
      errors=$((errors + 1))
    fi
    cmp -s "$NAME.hex" "$NAME.hex.ref"
    if [ $? == 0 ]
    then
      echo -n "hex pass; "
    else
      echo -n "hex FAIL; "
      errors=$((errors + 1))
    fi
    cmp -s "$NAME.list.txt" "$NAME.list.txt.ref"
    if [ $? == 0 ]
    then
      echo -n "list pass; "
    else
      echo -n "list FAIL; "
      errors=$((errors + 1))
    fi
    cmp -s "$NAME.stdout.txt" "$NAME.stdout.txt.ref"
    if [ $? == 0 ]
    then
      echo -n "stdout pass. "
    else
      echo -n "stdout FAIL. "
      errors=$((errors + 1))
    fi
  else
    echo -n "generated with suffix $SUFFIX"
  fi
  echo
done

echo
if (( errors > 0 )); then
  echo "...$errors errors detected."
else
  echo "...no errors detected."
fi
echo
