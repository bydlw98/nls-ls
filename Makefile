CARGO=cargo
PROFILE=release
INSTALL=install
prefix=/usr/local
exec_prefix=$(prefix)
bindir=$(exec_prefix)/bin

.PHONY: build
build:
	$(CARGO) build --profile $(PROFILE)

.PHONY: clean
clean:
	$(CARGO) clean

.PHONY: install
install:
	$(INSTALL) -Dm755 target/$(PROFILE)/nls $(bindir)/nls

.PHONY: uninstall
uninstall:
	$(RM) $(bindir)/nls
