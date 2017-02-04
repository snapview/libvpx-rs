#!/bin/sh

BINDGEN=${1:-bindgen}

OUT=lib.rs

rm -f $OUT
echo "#![allow(non_camel_case_types)]" >> $OUT
echo "#![allow(non_snake_case)]"       >> $OUT
echo "#![allow(raw_pointer_derive)]"   >> $OUT
echo "#![allow(missing_copy_implementations)]" >> $OUT
echo "extern crate libc;" >> $OUT

(clang -std=gnu11 -dM -E -x c - < /dev/null) > builtin_defines.h

$BINDGEN --no-unstable-rust -o temp -- -nostdinc ffi.h -pthread -D__BINDGEN__ -std=gnu11

cat temp >> $OUT
rm temp
