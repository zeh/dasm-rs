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
COLOR_YELLOW="\033[1;93m"
COLOR_GREEN="\033[1;32m"
COLOR_RESET="\033[0m"
FAIL_LABEL="${COLOR_RED}FAIL${COLOR_RESET}"
SKIP_LABEL="${COLOR_YELLOW}skip${COLOR_RESET}"
PASS_LABEL="pass"

# 1. Generate files

echo
echo "Running build test cases"
echo

custom_files=(
  "basic/*.asm"
  "basic-strict/*.asm"
  "atari2600/*.asm"
  "atari7800/*.asm"
  "channel-f/*.asm"
  "jeff-jetton-examples/*.asm"
)
custom_params=(
  "-f1 -Ibasic -Ibasic/incdirtest"
  "-f1 -Ibasic -Ibasic/incdirtest -S"
  "-f3 -I../machines/atari2600/"
  "-f3 -I../machines/atari7800/"
  "-f3 -I../../machines/channel-f/"
  "-f3"
)
wants_error=(
  "0"
  "1"
  "0"
  "0"
  "0"
  "0"
)

errors=0

for ((i = 0; i < ${#custom_files[@]}; i++)); do
  FILE_MASK=${custom_files[i]}
  DIR_NAME=${FILE_MASK%/*}
  STRICT_SUFFIX=""

  if [[ "${wants_error[i]}" == "1" ]]; then
    STRICT_SUFFIX=".strict"
  fi

  # Removes files first
  if [[ -z "$SUFFIX" ]]; then
    rm -r ${DIR_NAME}/*.txt
    rm -r ${DIR_NAME}/*.bin
    rm -r ${DIR_NAME}/*.hex
  else
    rm -r ${DIR_NAME}/*$SUFFIX
  fi

  for j in ${FILE_MASK}; do
    NAME="${j//.asm/}"

    # Compile .asm into .bin, .list.txt, and symbols.txt
    $DASM "$NAME.asm" ${custom_params[i]} -o"$NAME$STRICT_SUFFIX.bin$SUFFIX" -l"$NAME.list$STRICT_SUFFIX.txt$SUFFIX" -s"$NAME.symbols$STRICT_SUFFIX.txt$SUFFIX" -DINEEPROM 2>&1 | \
      tee "$NAME.stdout$STRICT_SUFFIX.txt$SUFFIX" >/dev/null
      # | \grep -vE 'error|Complete|Fatal|Warning^?'
    $DASM "$NAME.asm" -v4 ${custom_params[i]} -o/dev/null -DINEEPROM > "$NAME.stdout-verbose$STRICT_SUFFIX.txt$SUFFIX" 2>&1
    $DASM "$NAME.asm" -d1 ${custom_params[i]} -o/dev/null -DINEEPROM > "$NAME.stdout-debug$STRICT_SUFFIX.txt$SUFFIX" 2>&1

    # Obfuscate memory addresses (they change every time)
    sed -i -r 's/^[0-9a-f]{12} /0MEMORYADDR0 /g' "$NAME.stdout-debug$STRICT_SUFFIX.txt$SUFFIX"

    # Obfuscate line numbers for error messages (useless)
    sed -i -r 's/.rs:[0-9]+:[0-9]+$/.rs:0000:0/g' "$NAME.stdout$STRICT_SUFFIX.txt$SUFFIX"
    sed -i -r 's/.rs:[0-9]+:[0-9]+$/.rs:0000:0/g' "$NAME.stdout-verbose$STRICT_SUFFIX.txt$SUFFIX"
    sed -i -r 's/.rs:[0-9]+:[0-9]+$/.rs:0000:0/g' "$NAME.stdout-debug$STRICT_SUFFIX.txt$SUFFIX"

    # Generate .hex file from .bin
    $FTOHEX 1 "$NAME$STRICT_SUFFIX.bin$SUFFIX" "$NAME$STRICT_SUFFIX.hex$SUFFIX"

    # Fixes typos in original
    if [ -n "$SUFFIX" ]; then
      sed -i -r 's/defintion/definition/g' ${DIR_NAME}/*$SUFFIX
    fi

    # Display results
    echo -ne "  * ${NAME}: "

    if [[ -z "$SUFFIX" ]]; then
      # Compare binaries, or error if expected
      if [[ "${wants_error[i]}" == "0" ]]; then
        cmp -s "$NAME.bin" "$NAME.bin.ref"
        if [ $? == 0 ]; then
          echo -ne "bin $PASS_LABEL; "
        else
          echo -ne "bin $FAIL_LABEL; "
          errors=$((errors + 1))
        fi
        cmp -s "$NAME.hex" "$NAME.hex.ref"
        if [ $? == 0 ]; then
          echo -ne "hex $PASS_LABEL; "
        else
          echo -ne "hex $FAIL_LABEL; "
          errors=$((errors + 1))
        fi
      else
        grep error "$NAME.list$STRICT_SUFFIX.txt" 2>&1 >/dev/null
        if [ $? == 0 ]; then
          echo -ne "error trigger $PASS_LABEL; "
        else
          echo -ne "error trigger $FAIL_LABEL; "
          errors=$((errors + 1))
        fi
      fi

      # Compare list
      cmp -s "$NAME.list$STRICT_SUFFIX.txt" "$NAME.list$STRICT_SUFFIX.txt.ref"
      if [ $? == 0 ]; then
        echo -ne "list $PASS_LABEL; "
      else
        echo -ne "list $FAIL_LABEL; "
        errors=$((errors + 1))
      fi

      # Compare symbols
      if [ -f "$NAME.symbols$STRICT_SUFFIX.txt.ref" ]; then
        cmp -s "$NAME.symbols$STRICT_SUFFIX.txt" "$NAME.symbols$STRICT_SUFFIX.txt.ref"
        if [ $? == 0 ]; then
          echo -ne "symbols $PASS_LABEL; "
        else
          echo -ne "symbols $FAIL_LABEL; "
          errors=$((errors + 1))
        fi
      else
        echo -ne "symbols $SKIP_LABEL; "
      fi

      # Compare stdout
      cmp -s "$NAME.stdout$STRICT_SUFFIX.txt" "$NAME.stdout$STRICT_SUFFIX.txt.ref"
      if [ $? == 0 ]; then
        echo -ne "stdout $PASS_LABEL; "
      else
        echo -ne "stdout $FAIL_LABEL; "
        errors=$((errors + 1))
      fi

      # Compare stdout (verbose)
      cmp -s "$NAME.stdout-verbose$STRICT_SUFFIX.txt" "$NAME.stdout-verbose$STRICT_SUFFIX.txt.ref"
      if [ $? == 0 ]; then
        echo -ne "stdout-verbose $PASS_LABEL; "
      else
        echo -ne "stdout-verbose $FAIL_LABEL; "
        errors=$((errors + 1))
      fi

      # Compare stdout (debug)
      cmp -s "$NAME.stdout-debug$STRICT_SUFFIX.txt" "$NAME.stdout-debug$STRICT_SUFFIX.txt.ref"
      if [ $? == 0 ]; then
        echo -ne "stdout-debug $PASS_LABEL. "
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
done

if (( errors > 0 )); then
  echo "...$errors errors detected."
else
  echo "...no errors detected."
fi
echo

# 2. Other cases

echo
echo "Other cases"
echo

errors=0

# Removes files first
if [[ -z "$SUFFIX" ]]; then
  rm *.txt
else
  rm *$SUFFIX
fi

NAME="dasm-no-args"

# Run without parameters
$DASM 2>&1 | \
  tee "$NAME.stdout.txt$SUFFIX" >/dev/null

# Display results
echo -ne "  * ${NAME}: "

if [[ -z "$SUFFIX" ]]; then
  cmp -s "$NAME.stdout.txt" "$NAME.stdout.txt.ref"
  if [ $? == 0 ]; then
    echo -ne "stdout $PASS_LABEL. "
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
