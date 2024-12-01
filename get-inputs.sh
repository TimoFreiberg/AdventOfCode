#!/bin/sh

if ! [ -f session.cookie ]; then
echo "store session cookie in file 'session.cookie'"
exit 1
fi

usage() {
  echo "Usage: $0 YEAR [START_DAY [END_DAY]]"
  exit 1
}


SESSION_COOKIE=$(cat session.cookie)
echo "session cookie: $SESSION_COOKIE"

get_input() {
  YEAR=$1
  DAY=$2

  URL="https://adventofcode.com/$YEAR/day/$DAY/input"

  curl "$URL" \
    --fail \
    -H "Cookie: session=$SESSION_COOKIE"
}

YEAR=$1

if [ -z "$YEAR" ]; then
usage
fi;

START_DAY=${2:-1}
END_DAY=${3:-25}

mkdir -p "inputs/$YEAR"

for DAY in $(seq "$START_DAY" "$END_DAY"); do
  INPUT_PATH="inputs/$YEAR/$DAY"
  if [ -f "$INPUT_PATH" ]; then
    echo "$INPUT_PATH already exists"
    continue
  else
    INPUT=$(get_input "$YEAR" "$DAY") || continue
    echo "$INPUT" > "$INPUT_PATH"
    echo "$INPUT_PATH created"
  fi
done
