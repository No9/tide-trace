
static : ../target/native/tide-trace.o ../target/native/trace-tide.o ../target/native/libtide-trace.a

../target/native/tide-trace.o : tide-trace.c tide-trace.h
	mkdir -p ../target/native && cc tide-trace.c -c -fPIC -o ../target/native/tide-trace.o

../target/native/trace-tide.o : 
	dtrace -G -s trace-tide.d ../target/native/tide-trace.o -o ../target/native/trace-tide.o

../target/native/libtide-trace.a :
	ar crus ../target/native/libtide-trace.a ../target/native/tide-trace.o ../target/native/trace-tide.o 

clean :
	rm -fr ../target/native