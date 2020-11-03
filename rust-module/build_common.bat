@echo Target: %1

cargo build --lib --target %1
move ./%1/debug/gvim_chop.dll ./gvim_chop.dll
