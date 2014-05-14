
RC=rustc
RFLAGS=

all: lib test
	

lib:
	$(RC) $(RFLAGS) objc.rs

test:
	$(RC) $(RFLAGS) --test test.rs
	./test

clean:
	@rm test *.rlib *.dylib
