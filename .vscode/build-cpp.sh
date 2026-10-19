#!/usr/bin/env bash
set -euo pipefail

file="${1:?}"
dir="${2:?}"
base="${3:?}"
ext="${4:?}"
run_flag="${5:-}"

case "${ext,,}" in
  .cpp|.cc|.cxx|.c|.h|.hpp)
    ;;
  *)
    echo "❌ 当前文件不是 C/C++（${file}），不会用 g++ 编译。" >&2
    echo "   若要调试 Go：打开 .go 文件后按 F5（会自动走 Go 调试器）。" >&2
    echo "   或在「运行和调试」下拉框中选择：Debug Go (current file)" >&2
    exit 1
    ;;
esac

mkdir -p "${dir}/.build"
g++ -g -O0 -std=c++17 -Wall "${file}" -o "${dir}/.build/${base}"

if [[ "${run_flag}" == "--run" ]]; then
  exec "${dir}/.build/${base}"
fi
