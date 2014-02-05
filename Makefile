RUSTC ?= rustc
RUST_FLAGS ?= -O

libargparse := $(shell $(RUSTC) --crate-file-name src/lib.rs)

.PHONY : all
all: $(libargparse)

.PHONY : clean
clean:
	rm -rf build/
	rm -f $(libargparse)

.PHONY : test
test: build/argparse-test
	$<

-include $(wildcard build/*.d)

$(libargparse): src/lib.rs
	@mkdir -p build
	$(RUSTC) $(RUST_FLAGS) --dep-info build/$(notdir $<).d $< --out-dir $(dir $@)

build/argparse-test: src/lib.rs
	@mkdir -p build
	$(RUSTC) $(RUST_FLAGS) --test --dep-info build/$(notdir $<).d $< -o $@
