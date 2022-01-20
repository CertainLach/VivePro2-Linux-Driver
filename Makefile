.PHONY: clean build install enable disable

clean:
	rm -f driver_lighthouse_proxy.so
format:
	clang-format -i driver.cpp -style=llvm

driver_lighthouse_proxy.so: driver.cpp
	gcc --pedantic -Werror -DLIGHTHOUSE_BIN=\"$(LIGHTHOUSE_BIN)\" -shared -o $@ -fPIC $<

build: driver_lighthouse_proxy.so

LIGHTHOUSE_BIN:=$(HOME)/.local/share/Steam/steamapps/common/SteamVR/drivers/lighthouse/bin/linux64

install: build
	cp $(PWD)/driver_lighthouse_proxy.so $(LIGHTHOUSE_BIN)/
	test ! -f $(LIGHTHOUSE_BIN)/driver_lighthouse_real.so && cp $(LIGHTHOUSE_BIN)/driver_lighthouse.so $(LIGHTHOUSE_BIN)/driver_lighthouse_real.so || true

enable: install
	rm $(LIGHTHOUSE_BIN)/driver_lighthouse.so
	ln -s $(LIGHTHOUSE_BIN)/driver_lighthouse_proxy.so $(LIGHTHOUSE_BIN)/driver_lighthouse.so

disable:
	rm $(LIGHTHOUSE_BIN)/driver_lighthouse.so
	ln -s $(LIGHTHOUSE_BIN)/driver_lighthouse_real.so $(LIGHTHOUSE_BIN)/driver_lighthouse.so
