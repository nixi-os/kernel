#!/bin/sh

find "initramfs" -depth | cpio -ov > "initramfs.cpio"
