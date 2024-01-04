CC=g++
IDIR =include
CFLAGS=-I$(IDIR) -std=c++11
SRC=src
OBJP=obj
LIBS= -g -O2 -pthread -L/usr/local/lib -lntl -lgmp -lm  
TEST=test

# Dependencies:
_DEPS = ltv.h ltv_base.h util.h sample.h functions.h encoding.h
DEPS = $(patsubst %,$(IDIR)/%,$(_DEPS))
_OBJ = ltv.o ltv_base.o util.o sample.o functions.o encoding.o
OBJ = $(patsubst %,$(OBJP)/%,$(_OBJ))
_OTESTS = ltvtest.o
OTESTS = $(patsubst %,$(TEST)/%,$(_TESTS))

$(OBJP)/%.o: $(SRC)/%.cpp 
	$(CC) -g -c -o $@ $< $(CFLAGS)

# Test cases:
test: test_ltv

test_ltv:
	./test/ltvtest

all: ltvtest

# Build:
ltvtest: $(OBJ) $(OTESTS)
	$(CC) -g test/ltvtest.cpp -o $(TEST)/$@ $^ $(CFLAGS) $(LIBS)

.PHONY: test clean

# Cleaning:
clean:
	rm -f $(OBJP)/*.o *~ core $(INCDIR)/*~ 
	rm -f $(LOBJP)/*.o $(TEST)/*~

