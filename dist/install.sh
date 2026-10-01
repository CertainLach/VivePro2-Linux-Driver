#!/bin/sh

set -eu

SCRIPT=$(readlink -f "$0")
SCRIPTPATH=$(dirname "$SCRIPT")

# Patreon release contains bundled sewer, use it here
PATH=$SCRIPTPATH/tools:$PATH

echo "SteamVR driver for Vive Pro 2"
echo "Consider supporting developer on patreon: https://patreon.com/0lach"
sleep 3

STEAMVR="${STEAMVR:-$HOME/.local/share/Steam/steamapps/common/SteamVR}"
if ! test -d "$STEAMVR"; then
	echo "SteamVR not found at $STEAMVR (Set \$STEAMVR manually?)"
	exit 1
fi
echo "SteamVR at $STEAMVR"

LIGHTHOUSE_DRIVER=$STEAMVR/drivers/lighthouse/bin/linux64

if ! test -f "$LIGHTHOUSE_DRIVER/driver_lighthouse.so"; then
	echo "Lighthouse driver not found, broken installation?"
	exit 1
fi

if grep -s "https://patreon.com/0lach" "$LIGHTHOUSE_DRIVER/driver_lighthouse.so"; then
	echo "Restore the SteamVR installation: Manage => Installed Files => Verify integrity in tool files"
	exit 1
fi
if test -f "$LIGHTHOUSE_DRIVER/driver_lighthouse_real.so" || test -d "$LIGHTHOUSE_DRIVER/lens-distort"; then
	echo "= Removing the leftovers of old lighthouse proxy driver"
	rm -f "$LIGHTHOUSE_DRIVER/driver_lighthouse_real.so"
	rm -rf "$LIGHTHOUSE_DRIVER/lens-proxy"
fi

ROOM_SETUP_ASSETS=$STEAMVR/tools/steamvr_room_setup/linux64/steamvr_room_setup_Data/sharedassets0.assets
if test -f "$ROOM_SETUP_ASSETS"; then
	echo "= Patching room setup crash with texture mip counts"
	sewer --backup "$ROOM_SETUP_ASSETS.bak" "$ROOM_SETUP_ASSETS" patch-file --partial "$SCRIPTPATH/steamvr_room_setup.sew" || true
fi

vrpathreg() {
	if command -v steam-run >/dev/null; then
		steam-run bash "$STEAMVR/bin/vrpathreg.sh" "$@"
	else
		bash "$STEAMVR/bin/vrpathreg.sh" "$@"
	fi
}

TARGET="${XDG_DATA_HOME:-$HOME/.local/share}/vivepro2-linux-driver"

echo "= Installing driver to $TARGET"
mkdir -p "$TARGET"
rsync -av --delete --exclude /install.sh --exclude /steamvr_room_setup.sew --exclude /tools "$SCRIPTPATH/" "$TARGET/"

vrpathreg show | sed -n 's/^\s*viveVR : //p' | while read -r OTHER; do
	if test "$OTHER" != "$TARGET"; then
		echo "= Unregistering other viveVR driver at $OTHER"
		vrpathreg removedriver "$OTHER"
	fi
done

echo "= Registering driver"
vrpathreg adddriver "$TARGET"

echo "Installation finished, try to start SteamVR"
