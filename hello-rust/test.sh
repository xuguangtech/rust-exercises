#!/bin/sh

set -u

ensure() {
    echo "$@"

    if ! "$@"; then err "command failed: $*"; fi
}

# 标准输出显示字符串
say() {
    printf 'rustup: %s\n' "$1"
}

# 报错并返回
err() {
    say "$1" >&2
    exit 1
}

ensure touch ""

echo "heere"

# # 为变量设置一个默认值
# RTEST="${RTEST:-https}"
# echo $RTEST

# # 方法定义
# fun() {
#     # local _arr=(`echo $1 | cut -d " " --output-delimiter=" " -f 1-`) # linux
#     local _arr=(`echo $1 | cut -f1- -d " "`) # mac
#     local _n_arr=${#_arr[@]}
#     for ((i=0;i<$_n_arr;i++));
#     do
#         elem=${_arr[$i]}
#         echo "$i : $elem"
#     done;
# }

# # 使用方法
# array=(a b c)
# fun "$(echo ${array[@]})"