#!/usr/bin/env bash

# Simple test script adapted from Matt Dillon's 2.16  release.
# It's harder to do this in a Makefile, so let's use a script.

# Ran without parameters, this compares existing files to reference files.
# Ran with parameters, this generates reference files, e.g.:
#   ./run_tests.sh ../bin-original/dasm .ref

ORIGIN=${1:-../bin}
SUFFIX=$2
DASM="${ORIGIN}/dasm"
FTOHEX="${ORIGIN}/ftohex"
COLOR_RED="\033[1;31m"
COLOR_RESET="\033[0m"
FAIL_LABEL="${COLOR_RED}FAIL${COLOR_RESET}"

# 1. General cleanup

echo
echo "Cleaning up"
echo

if [[ -z "$SUFFIX" ]]; then
  echo "... Removing all previously generated files"
  rm -r ./*.txt
  rm -r ./*.bin
  rm -r ./*.hex
  rm -r ./atari2600/*.txt
  rm -r ./atari2600/*.bin
  rm -r ./atari2600/*.hex
  rm -r ./atari7800/*.txt
  rm -r ./atari7800/*.bin
  rm -r ./atari7800/*.hex
  rm -r ./channel-f/*.txt
  rm -r ./channel-f/*.bin
  rm -r ./channel-f/*.hex
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

  # Compile .asm into .bin, .list.txt, and symbols.txt
  $DASM "$i" -f1 -o"$NAME.bin$SUFFIX" -l"$NAME.list.txt$SUFFIX" -s"$NAME.symbols.txt$SUFFIX" -DINEEPROM 2>&1 | \
    tee "$NAME.stdout.txt$SUFFIX" >/dev/null
    # | \grep -vE 'error|Complete|Fatal|Warning^?'
  $DASM "$i" -f1 -v4 -o/dev/null -DINEEPROM > "$NAME.stdout-verbose.txt$SUFFIX" 2>&1
  $DASM "$i" -f1 -d1 -o/dev/null -DINEEPROM > "$NAME.stdout-debug.txt$SUFFIX" 2>&1

  # Generate .hex file from .bin
  $FTOHEX 1 "$NAME.bin$SUFFIX" "$NAME.hex$SUFFIX"

  # Display results
  echo -ne "  * ${NAME}: "

  if [[ -z "$SUFFIX" ]]; then
    cmp -s "$NAME.bin" "$NAME.bin.ref"
    if [ $? == 0 ]; then
      echo -ne "bin pass; "
    else
      echo -ne "bin $FAIL_LABEL; "
      errors=$((errors + 1))
    fi
    cmp -s "$NAME.hex" "$NAME.hex.ref"
    if [ $? == 0 ]; then
      echo -ne "hex pass; "
    else
      echo -ne "hex $FAIL_LABEL; "
      errors=$((errors + 1))
    fi
    cmp -s "$NAME.list.txt" "$NAME.list.txt.ref"
    if [ $? == 0 ]; then
      echo -ne "list pass; "
    else
      echo -ne "list $FAIL_LABEL; "
      errors=$((errors + 1))
    fi
    cmp -s "$NAME.symbols.txt" "$NAME.symbols.txt.ref"
    if [ $? == 0 ]; then
      echo -ne "symbols pass; "
    else
      echo -ne "symbols $FAIL_LABEL; "
      errors=$((errors + 1))
    fi
    cmp -s "$NAME.stdout.txt" "$NAME.stdout.txt.ref"
    if [ $? == 0 ]; then
      echo -ne "stdout pass; "
    else
      echo -ne "stdout $FAIL_LABEL; "
      errors=$((errors + 1))
    fi
    cmp -s "$NAME.stdout-verbose.txt" "$NAME.stdout-verbose.txt.ref"
    if [ $? == 0 ]; then
      echo -ne "stdout-verbose pass; "
    else
      echo -ne "stdout-verbose $FAIL_LABEL; "
      errors=$((errors + 1))
    fi
    cmp -s "$NAME.stdout-debug.txt" "$NAME.stdout-debug.txt.ref"
    if [ $? == 0 ]; then
      echo -ne "stdout-debug pass. "
    else
      echo -ne "stdout-debug $FAIL_LABEL. "
      errors=$((errors + 1))
    fi
  else
    echo -ne "generated with suffix $SUFFIX"
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

  # Compile .asm into .bin, .list.txt, and symbols.txt
  $DASM "$i" -S -f1 -o"$NAME.strict.bin$SUFFIX" -l"$NAME.list.strict.txt$SUFFIX" -s"$NAME.symbols.strict.txt$SUFFIX" -DINEEPROM 2>&1 | \
    tee "$NAME.stdout.strict.txt$SUFFIX" >/dev/null
    # | \grep -vE 'error|Complete|Fatal|Warning^?'
  $DASM "$i" -S -f1 -v4 -o/dev/null -DINEEPROM > "$NAME.stdout-verbose.strict.txt$SUFFIX" 2>&1
  $DASM "$i" -S -f1 -d1 -o/dev/null -DINEEPROM > "$NAME.stdout-debug.strict.txt$SUFFIX" 2>&1

  # Display results
  echo -ne "  * ${NAME}: "

  if [[ -z "$SUFFIX" ]]; then
    grep error "$NAME.list.strict.txt" 2>&1 >/dev/null
    if [ $? == 0 ]; then
      echo -ne "error trigger pass; "
    else
      echo -ne "error trigger $FAIL_LABEL; "
      errors=$((errors + 1))
    fi
    cmp -s "$NAME.list.strict.txt" "$NAME.list.strict.txt.ref"
    if [ $? == 0 ]; then
      echo -ne "list pass; "
    else
      echo -ne "list $FAIL_LABEL; "
      errors=$((errors + 1))
    fi
    if [ -f "$NAME.symbols.strict.txt.ref" ]; then
      cmp -s "$NAME.symbols.strict.txt" "$NAME.symbols.strict.txt.ref"
      if [ $? == 0 ]; then
        echo -ne "symbols pass; "
      else
        echo -ne "symbols $FAIL_LABEL; "
        errors=$((errors + 1))
      fi
    else
      if [ -f "$NAME.symbols.strict.txt" ]; then
        echo -ne "symbols $FAIL_LABEL (new); "
        errors=$((errors + 1))
      else
        echo -ne "symbols skip; "
      fi
    fi
    cmp -s "$NAME.stdout.strict.txt" "$NAME.stdout.strict.txt.ref"
    if [ $? == 0 ]; then
      echo -ne "stdout pass; "
    else
      echo -ne "stdout $FAIL_LABEL; "
      errors=$((errors + 1))
    fi
    cmp -s "$NAME.stdout-verbose.strict.txt" "$NAME.stdout-verbose.strict.txt.ref"
    if [ $? == 0 ]; then
      echo -ne "stdout-verbose pass; "
    else
      echo -ne "stdout-verbose $FAIL_LABEL; "
      errors=$((errors + 1))
    fi
    cmp -s "$NAME.stdout-debug.strict.txt" "$NAME.stdout-debug.strict.txt.ref"
    if [ $? == 0 ]; then
      echo -ne "stdout-debug pass. "
    else
      echo -ne "stdout-debug $FAIL_LABEL. "
      errors=$((errors + 1))
    fi
  else
    echo -ne " generated with suffix $SUFFIX"
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

  # Compile .asm into .bin, .list.txt, and symbols.txt
  $DASM "$NAME.asm" ${custom_params[i]} -o"$NAME.bin$SUFFIX" -l"$NAME.list.txt$SUFFIX" -s"$NAME.symbols.txt$SUFFIX" -DINEEPROM 2>&1 | \
    tee "$NAME.stdout.txt$SUFFIX" >/dev/null
    # | \grep -vE 'error|Complete|Fatal|Warning^?'
  $DASM "$NAME.asm" -v4 ${custom_params[i]} -o/dev/null -DINEEPROM > "$NAME.stdout-verbose.txt$SUFFIX" 2>&1
  $DASM "$NAME.asm" -d1 ${custom_params[i]} -o/dev/null -DINEEPROM > "$NAME.stdout-debug.txt$SUFFIX" 2>&1

  # Generate .hex file from .bin
  $FTOHEX 1 "$NAME.bin$SUFFIX" "$NAME.hex$SUFFIX"

  # Display results
  echo -ne "  * ${NAME}: "

  if [[ -z "$SUFFIX" ]]; then
    cmp -s "$NAME.bin" "$NAME.bin.ref"
    if [ $? == 0 ]; then
      echo -ne "bin pass; "
    else
      echo -ne "bin $FAIL_LABEL; "
      errors=$((errors + 1))
    fi
    cmp -s "$NAME.hex" "$NAME.hex.ref"
    if [ $? == 0 ]; then
      echo -ne "hex pass; "
    else
      echo -ne "hex $FAIL_LABEL; "
      errors=$((errors + 1))
    fi
    cmp -s "$NAME.list.txt" "$NAME.list.txt.ref"
    if [ $? == 0 ]; then
      echo -ne "list pass; "
    else
      echo -ne "list $FAIL_LABEL; "
      errors=$((errors + 1))
    fi
    cmp -s "$NAME.symbols.txt" "$NAME.symbols.txt.ref"
    if [ $? == 0 ]; then
      echo -ne "symbols pass; "
    else
      echo -ne "symbols $FAIL_LABEL; "
      errors=$((errors + 1))
    fi
    cmp -s "$NAME.stdout.txt" "$NAME.stdout.txt.ref"
    if [ $? == 0 ]; then
      echo -ne "stdout pass; "
    else
      echo -ne "stdout $FAIL_LABEL; "
      errors=$((errors + 1))
    fi
    cmp -s "$NAME.stdout-verbose.txt" "$NAME.stdout-verbose.txt.ref"
    if [ $? == 0 ]; then
      echo -ne "stdout-verbose pass; "
    else
      echo -ne "stdout-verbose $FAIL_LABEL; "
      errors=$((errors + 1))
    fi
    cmp -s "$NAME.stdout-debug.txt" "$NAME.stdout-debug.txt.ref"
    if [ $? == 0 ]; then
      echo -ne "stdout-debug pass. "
    else
      echo -ne "stdout-debug $FAIL_LABEL. "
      errors=$((errors + 1))
    fi
  else
    echo -ne "generated with suffix $SUFFIX"
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

# 5. Other

echo "Other cases"
echo

errors=0

NAME="dasm-no-args"

# Run without parameters
$DASM 2>&1 | \
  tee "$NAME.stdout.txt$SUFFIX" >/dev/null

# Display results
echo -ne "  * ${NAME}: "

if [[ -z "$SUFFIX" ]]; then
  cmp -s "$NAME.stdout.txt" "$NAME.stdout.txt.ref"
  if [ $? == 0 ]; then
    echo -ne "stdout pass. "
  else
    echo -ne "stdout $FAIL_LABEL. "
    errors=$((errors + 1))
  fi
else
  echo -ne "generated with suffix $SUFFIX"
fi
echo

echo
if (( errors > 0 )); then
  echo "...$errors errors detected."
else
  echo "...no errors detected."
fi
echo
echo
