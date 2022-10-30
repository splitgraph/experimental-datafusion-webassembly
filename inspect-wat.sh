#!/bin/bash

# Inspect the generated wat file to make sure it doesn't
# have any references to std::time or "env" (which is what
# some functions can get shunted into when they have no platform implementation)
wat_file="$1"

main() {
  echo "$wat_file"
  ls -ltrh "$wat_file"
  echo
  echo "any functions loaded into env? (this should be empty)"
  grep '"env"' "$wat_file"
  echo "---"

  echo
  echo "any \$std::time references? (this should be empty)"
  echo "------------"
  while read -r std_time_ref ; do
    echo "> $std_time_ref"
    grep "func $std_time_ref" "$wat_file" # the function declaration/signature
    echo
  done < <(all_std_time_refs)

  echo
  echo "any \$instant references? (there should be)"
  echo "------------"
  while read -r instant_ref ; do
    echo "> $instant_ref"
    grep "func $instant_ref" "$wat_file" # the function declaration/signature
    echo
  done < <(all_instant_refs)
}


all_std_time_refs() {
  while read -r std_time_ref ; do
    echo "$std_time_ref" | cut -d' ' -f2 ;
  done < <(grep '$std::time' "$wat_file" ) | sort -u
}

all_instant_refs() {
  while read -r instant ; do
    echo "$instant" | cut -d' ' -f2 ;
  done < <(grep '$instant' "$wat_file" ) | sort -u
}

main "$@"
