
all: run


bcopy_performance: bcopy_performance.cpp
	g++ -std=gnu++11 -O3 bcopy_performance.cpp -o bcopy_performance 

run: bcopy_performance
	/usr/bin/time ./bcopy_performance

clean::
	rm -f *.o bcopy_performance
