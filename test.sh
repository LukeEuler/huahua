#!/usr/bin/env bash

set -e

source_file="pics/test.jpg"
target_dir="pics/target"
rm -rf ${target_dir}
mkdir -p ${target_dir}
cargo build --release

./target/release/huahua -i ${source_file} -o ${target_dir}/1977.jpg -f 1977
./target/release/huahua -i ${source_file} -o ${target_dir}/aden.jpg -f aden
./target/release/huahua -i ${source_file} -o ${target_dir}/brannan.jpg -f brannan
./target/release/huahua -i ${source_file} -o ${target_dir}/brooklyn.jpg -f brooklyn
./target/release/huahua -i ${source_file} -o ${target_dir}/clarendon.jpg -f clarendon
./target/release/huahua -i ${source_file} -o ${target_dir}/earlybird.jpg -f earlybird
./target/release/huahua -i ${source_file} -o ${target_dir}/gingham.jpg -f gingham
./target/release/huahua -i ${source_file} -o ${target_dir}/hudson.jpg -f hudson
./target/release/huahua -i ${source_file} -o ${target_dir}/inkwell.jpg -f inkwell
./target/release/huahua -i ${source_file} -o ${target_dir}/kelvin.jpg -f kelvin
./target/release/huahua -i ${source_file} -o ${target_dir}/lark.jpg -f lark
./target/release/huahua -i ${source_file} -o ${target_dir}/lofi.jpg -f lofi
./target/release/huahua -i ${source_file} -o ${target_dir}/maven.jpg -f maven
./target/release/huahua -i ${source_file} -o ${target_dir}/mayfair.jpg -f mayfair
./target/release/huahua -i ${source_file} -o ${target_dir}/moon.jpg -f moon
./target/release/huahua -i ${source_file} -o ${target_dir}/nashville.jpg -f nashville
./target/release/huahua -i ${source_file} -o ${target_dir}/reyes.jpg -f reyes
./target/release/huahua -i ${source_file} -o ${target_dir}/rise.jpg -f rise
./target/release/huahua -i ${source_file} -o ${target_dir}/slumber.jpg -f slumber
./target/release/huahua -i ${source_file} -o ${target_dir}/stinson.jpg -f stinson
./target/release/huahua -i ${source_file} -o ${target_dir}/toaster.jpg -f toaster
./target/release/huahua -i ${source_file} -o ${target_dir}/valencia.jpg -f valencia
./target/release/huahua -i ${source_file} -o ${target_dir}/walden.jpg -f walden
