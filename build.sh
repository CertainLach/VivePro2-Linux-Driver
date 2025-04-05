#!/bin/sh


# Credits to https://github.com/santeri3700/vive-pro-2-on-linux


set -e

export VIVEPRO2DRVDIR="$(pwd)"


cd sewer
cargo +nightly build --all-features --verbose --release



cd $VIVEPRO2DRVDIR/bin/driver-proxy
cargo +nightly build --release --all-features --verbose

cd $VIVEPRO2DRVDIR/bin/lens-server
cargo +nightly build --release --target x86_64-pc-windows-gnu --all-features --verbose


cd $VIVEPRO2DRVDIR/dist-proxy/
rm -Rf ./bin ./lens-server/ ./driver_lighthouse.so || echo "Delete failed, probably fine"
mkdir -p bin ./lens-server/

echo "Copying files"
cp $VIVEPRO2DRVDIR/sewer/target/release/sewer ./bin
cp $VIVEPRO2DRVDIR/target/x86_64-pc-windows-gnu/release/lens-server.exe ./lens-server/
cp $VIVEPRO2DRVDIR/target/release/libdriver_proxy.so ./driver_lighthouse.so


echo "Installing..."
./install.sh
