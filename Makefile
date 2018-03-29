GIR = gir/target/bin/gir
CONFIGS = $(wildcard conf/gir-*.toml)
GIR_SRC = gir/Cargo.toml gir/Cargo.lock gir/build.rs $(shell find gir/src -name '*.rs')
GIR_FILES = gir-files/Tepl-3.gir gir-files/Amtk-3.gir

LIBS = $(CONFIGS:conf/gir-%.toml=%-sys/src/lib.rs)

libs : $(LIBS)

%-sys/src/lib.rs : conf/gir-%.toml $(GIR) $(GIR_FILES)
	$(GIR) -c $< -o $(abspath $*-sys) -d gir-files

# Run `gir` generating the bindings
gir : src/auto/mod.rs

src/auto/mod.rs : Gir-amtk.toml Gir-tepl.toml $(GIR) $(GIR_FILES)
	$(GIR) -c Gir-amtk.toml -o amtk
	$(GIR) -c Gir-tepl.toml -o tepl

#
# $(GIR) : $(GIR_SRC)
# 	cd gir && cargo build --release


$(GIR) : $(GIR_SRC)
	rm -f gir/target/bin/gir
	cargo install --path gir --root gir/target
	rm -f gir/target/.crates.toml

$(GIR_SRC) $(GIR_FILES) :
	git submodule update --init
