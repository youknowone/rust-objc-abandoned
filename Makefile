
RC=rustc
RLIBFLAGS=
RFLAGS=-L.

all: lib
	

lib:
	$(RC) $(RLIBFLAGS) lib.rs

test: lib
	$(RC) $(RFLAGS) --test test.rs
	./test

crawl:
	$(RC) $(RFLAGS) test_crawl.rs
	./test_crawl

app:
	$(RC) $(RFLAGS) test_app.rs

clean:
	@rm test *.rlib *.dylib
