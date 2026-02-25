#!/usr/bin/env bash

changes="$(git diff --cached --name-only --diff-filter=ACMRTUXB)"

if [ -z "$changes" ]; then
  exit 0;
fi

exec treefmt --fail-on-change --quiet -- $changes
