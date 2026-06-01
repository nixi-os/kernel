#!/bin/sh

cd initramfs
find . -depth | sed -e 's|./\(.*\)|\1|' -e '$d' | cpio -ov > ../initramfs.cpio
cd ..
