libhello.a: hello.o
	ar -rcs libhello.a hello.o

hello.o: hello.c
	gcc -c -Wall -Werror -Wextra hello.c

clean:
	rm -f hello.o
