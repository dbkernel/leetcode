#!/usr/bin/env bash
set -euo pipefail

file="${1:?}"
ext="${2:?}"

if [[ "${ext,,}" != ".go" ]]; then
  echo "❌ 当前文件不是 Go（${file}）。" >&2
  exit 1
fi

exec go run "${file}"
