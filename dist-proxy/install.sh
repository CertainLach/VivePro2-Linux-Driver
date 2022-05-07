#!/bin/sh

set -euo pipefail

SCRIPT=$(readlink -f "$0")
SCRIPTPATH=$(dirname $SCRIPT)

# Patreon release contains bundled sewer, use it here
PATH=$SCRIPTPATH/bin:$PATH

echo "SteamVR proxy driver for Vive Pro 2"
echo "Consider supporting developer on patreon: https://patreon.com/0lach"
sleep 3

STEAMVR="${STEAMVR:-$HOME/.local/share/Steam/steamapps/common/SteamVR}"
if ! test -d $STEAMVR; then
	echo "SteamVR not found at $STEAMVR (Set \$STEAMVR manually?)"
	exit 1
fi
echo "SteamVR at $STEAMVR"

LIGHTHOUSE_DRIVER=$STEAMVR/drivers/lighthouse/bin/linux64

if ! test -f $LIGHTHOUSE_DRIVER/driver_lighthouse.so; then
	echo "Lighthouse driver not found, broken installation?"
	exit 1
fi

if ! test -f $LIGHTHOUSE_DRIVER/driver_lighthouse_real.so; then
	echo "= Moving original driver"
	cp $LIGHTHOUSE_DRIVER/driver_lighthouse.so $LIGHTHOUSE_DRIVER/driver_lighthouse_real.so
elif ! grep -s "https://patreon.com/0lach" $LIGHTHOUSE_DRIVER/driver_lighthouse.so; then
	echo "Original driver exists, but installed - is either old proxy, or updated original"
	echo "If SteamVR will fail to start after this message - then reinstal, and start installation again"
	echo "This check works better with new version of driver"
	echo "= Moving original driver"
	cp $LIGHTHOUSE_DRIVER/driver_lighthouse.so $LIGHTHOUSE_DRIVER/driver_lighthouse_real.so
else
	echo "= Proxy driver is already installed, updating"
fi

echo "= Patching real driver"
sewer -v --backup $LIGHTHOUSE_DRIVER/driver_lighthouse_real.so.bak $LIGHTHOUSE_DRIVER/driver_lighthouse_real.so patch-file --partial driver_lighthouse_real.sew || true

echo "= Overriding current driver"
rsync -a $SCRIPTPATH/driver_lighthouse.so $LIGHTHOUSE_DRIVER/driver_lighthouse.so

echo "= Updating proxy server"
rsync -ar $SCRIPTPATH/lens-server/ $LIGHTHOUSE_DRIVER/lens-server

echo "Installation finished, try to start SteamVR"
