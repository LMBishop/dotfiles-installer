#!/bin/sh

if ! [ rustc --version > /dev/null 2>&1 ]; then
    sudo pacman -S rustup || exit $?
    rustup default stable || exit $?
fi
makepkg -f || exit $?
sudo pacman -U dotfiles-installer*.pkg.tar*
