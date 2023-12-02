CARGO=cargo
PROFILE=release
INSTALL=install
prefix=/usr/local
exec_prefix=$(prefix)
bindir=$(exec_prefix)/bin
datadir=$(prefix)/share
COMPLETIONS_DIR=./completions
BASH_COMPLETION_DIR=$(datadir)/bash-completion/completions
FISH_COMPLETION_DIR=$(datadir)/fish/vendor_completions.d
ZSH_COMPLETION_DIR=$(datadir)/zsh/vendor-completions
MAN1_DIR=$(datadir)/man/man1

.PHONY: build
build:
	BUILD_DATA_FILES=1 $(CARGO) build --profile $(PROFILE)

.PHONY: clean
clean:
	$(CARGO) clean

.PHONY: install
install:
	$(INSTALL) -Dm755 target/$(PROFILE)/nls $(DESTDIR)/$(bindir)/nls
	$(INSTALL) -Dm644 ${COMPLETIONS_DIR}/nls.bash $(DESTDIR)/$(BASH_COMPLETION_DIR)/nls.bash
	$(INSTALL) -Dm644 ${COMPLETIONS_DIR}/nls.fish $(DESTDIR)/$(FISH_COMPLETION_DIR)/nls.fish
	$(INSTALL) -Dm644 ${COMPLETIONS_DIR}/_nls $(DESTDIR)/$(ZSH_COMPLETION_DIR)/_nls
	$(INSTALL) -Dm644 doc/nls.1 $(DESTDIR)/$(MAN1_DIR)/nls.1

.PHONY: test
test:
	$(CARGO) test

.PHONY: uninstall
uninstall:
	$(RM) $(DESTDIR)/$(bindir)/nls
	$(RM) $(DESTDIR)/$(BASH_COMPLETION_DIR)/nls.bash
	$(RM) $(DESTDIR)/$(FISH_COMPLETION_DIR)/nls.fish
	$(RM) $(DESTDIR)/$(ZSH_COMPLETION_DIR)/_nls
	$(RM) $(DESTDIR)/$(MAN1_DIR)/nls.1
