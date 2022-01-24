# TODO

## Installation

- Install dependencies
- Compile `lensServer`
- Run `make enable`, and hope it will work

## Required kernel patches

- Mark HMD as non-display (Otherwise SteamVR will not be able to occupy display) https://lore.kernel.org/linux-kernel/20220118170037.14584-1-iam@lach.pw/T/#u

- Support type VII timings in DisplayID (VIVE reports mode in them) https://lore.kernel.org/linux-kernel/20220118215956.17229-1-iam@lach.pw/T/#e4b19a91cc9229280cf333c88e82dc824947ab1e1
