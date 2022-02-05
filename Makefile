.PHONY: clean build install enable disable

LIGHTHOUSE_BIN:=$(HOME)/.local/share/Steam/steamapps/common/SteamVR/drivers/lighthouse/bin/linux64
LENS_SERVER_DIST:=$(PWD)/lensServer/dist
WINE:=`which wine`

clean:
	rm -f driver_lighthouse_proxy.so
format:
	clang-format -i driver.cpp -style=llvm
	clang-format -i driver.h -style=llvm


build: driver_lighthouse_proxy.so

install: build
	cp $(PWD)/driver_lighthouse_proxy.so $(LIGHTHOUSE_BIN)/
	test ! -f $(LIGHTHOUSE_BIN)/driver_lighthouse_real.so && cp $(LIGHTHOUSE_BIN)/driver_lighthouse.so $(LIGHTHOUSE_BIN)/driver_lighthouse_real.so || true

enable: install
	rm $(LIGHTHOUSE_BIN)/driver_lighthouse.so
	ln -s $(LIGHTHOUSE_BIN)/driver_lighthouse_proxy.so $(LIGHTHOUSE_BIN)/driver_lighthouse.so

disable:
	rm $(LIGHTHOUSE_BIN)/driver_lighthouse.so
	ln -s $(LIGHTHOUSE_BIN)/driver_lighthouse_real.so $(LIGHTHOUSE_BIN)/driver_lighthouse.so

driver_lighthouse_proxy.so: driver.cpp discover_resolution.cpp
	g++ \
		--pedantic -fpermissive -std=c++2a -Werror \
		-DLIGHTHOUSE_BIN=\"$(LIGHTHOUSE_BIN)\" -DLENS_SERVER_DIST=\"$(LENS_SERVER_DIST)\" -DWINE=\"$(WINE)\" \
		-shared -static-libgcc -static-libstdc++ -ldl -o $@ -fPIC $^
