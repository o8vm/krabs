#
# Makefile for "libbzip2" decompression module
#

CC = i386-elf-gcc
CFLAGS = -m32 -fno-common -Wall -Os -c \
         -fno-ident \
         -fomit-frame-pointer \
		 -fno-asynchronous-unwind-tables \
		 -fno-stack-protector \
		 -fno-builtin

TARGET  = bz_decomp.o
HEADERS = bzlib_private.h
OBJS    = huffman.o    \
		  crctable.o   \
		  randtable.o  \
		  decompress.o \
		  malloc.o     \
		  bzlib.o

all: ${TARGET}

${TARGET} : ${OBJS}
	i386-elf-ld -m elf_i386 -r -o $@ ${OBJS}

huffman.o: huffman.c ${HEADERS}
	${CC} ${CFLAGS} $<

crctable.o: crctable.c ${HEADERS}
	${CC} ${CFLAGS} $<

randtable.o: randtable.c ${HEADERS}
	${CC} ${CFLAGS} $<

decompress.o: decompress.c ${HEADERS}
	${CC} ${CFLAGS} $<

malloc.o: malloc.c
	${CC} ${CFLAGS} $<

bzlib.o: bzlib.c ${HEADERS}
	${CC} ${CFLAGS} $<

clean:
	rm -f *.o
