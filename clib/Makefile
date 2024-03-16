hello.o: hello.c
	gcc -c -Wall -Werror -Wextra hello.c

libhello.a: hello.o
	ar -rcs libhello.a hello.o

clean:
	rm -f hello.o
