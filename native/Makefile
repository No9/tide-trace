
static : tide-trace.o trace-tide.o libtide-trace.a

tide-trace.o : tide-trace.c tide-trace.h
	cc tide-trace.c -c -fPIC -o tide-trace.o

trace-tide.o : 
	dtrace -G -s trace-tide.d tide-trace.o

libtide-trace.a :
	ar crus libtide-trace.a tide-trace.o trace-tide.o 

clean :
	rm tide-trace.o trace-tide.o libtide-trace.a