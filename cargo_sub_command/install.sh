#!/bin/sh

find . -name 'cargo-*' | while read file
do
	echo $file hard link creation
	chmod +x $file
	ln -f $file $HOME/.cargo/bin/
done
