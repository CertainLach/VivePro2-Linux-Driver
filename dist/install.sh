#!/bin/sh

set -euo pipefail

SCRIPT=$(readlink -f "$0")
SCRIPTPATH=$(dirname $SCRIPT)

STEAMVR="${STEAMVR:-$HOME/.local/share/Steam/steamapps/common/SteamVR}"
if ! test -d $STEAMVR; then
	echo "SteamVR not found at $STEAMVR (Set \$STEAMVR manually?)"
	exit 1
fi
echo "SteamVR at $STEAMVR"

LIGHTHOUSE_DRIVER=$STEAMVR/drivers/lighthouse/bin/linux64

if ! test -f $LIGHTHOUSE_DRIVER/driver_lighthouse.so; then
	echo "Lighthouse driver not found, broken installation?"
fi
if ! test -f $LIGHTHOUSE_DRIVER/driver_lighthouse_real.so; then
	echo "Creating backup (also used by proxy driver)"
	cp $LIGHTHOUSE_DRIVER/driver_lighthouse.so $LIGHTHOUSE_DRIVER/driver_lighthouse_real.so
else
	echo "Seems like proxy driver is already installed? Proceeding with update then"
fi

rsync -a $SCRIPTPATH/driver_lighthouse.so $LIGHTHOUSE_DRIVER/driver_lighthouse.so
rsync -ar $SCRIPTPATH/lens-server/ $LIGHTHOUSE_DRIVER/lens-server

echo "Installation finished, try to start SteamVR"
