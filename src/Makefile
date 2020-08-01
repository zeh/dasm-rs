#
# the DASM macro assembler (aka small systems cross assembler)
#
# Copyright (c) 1988-2002 by Matthew Dillon.
# Copyright (c) 1995 by Olaf "Rhialto" Seibert.
# Copyright (c) 2003-2008 by Andrew Davie.
# Copyright (c) 2008 by Peter H. Froehlich.
#
# This program is free software; you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation; either version 2 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License along
# with this program; if not, write to the Free Software Foundation, Inc.,
# 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

# Executables will be generated in the trunk/src/ directory (the
# "current directory" as far as this Makefile is concerned). If
# you want to move them somewhere, trunk/bin/ say, do so in the
# trunk/ Makefile!


all: build-release

build-release:
	cargo build --release

build-debug:
	cargo build

clean:
	cargo clean
	rm -rf *.o *.a $(ALL) \
	dasm-alpha-*.tar.gz \
	*.gcda *.gcno *.gcov gmon.out

# prepare an alpha release containing just the source code,
# nothing else is in the archive since it is not intended
# for the public, just designated volunteers

alpha:
	echo "This is an incomplete alpha release of DASM source code." >README.ALPHA
	echo "The purpose is to identify build problems, nothing more." >>README.ALPHA
	echo "Please do *not* re-distribute this release in any form!" >>README.ALPHA
	echo "Please do *not* distribute binaries derived from it either!" >>README.ALPHA
	tar zcvf dasm-alpha-`date +%F`.tar.gz README.ALPHA Makefile *.h *.c
	rm -rf README.ALPHA
