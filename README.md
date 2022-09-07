# Vive Pro 2 linux driver
/driver_lighthouse.so
## Why

Because HTC doesn't care about non-windows users, even if OG Vive worked just fine for them

## How

Current implementation of driver intercepts some calls between SteamVR and common headset driver `driver_lighthouse`, which is used for OG Vive, and makes it work with newer Vive Pro 2

## Progress

### Working

- HMD image - standard interface (DP) used, however there are some things missing in kernel, see patches below
- Audio output - standard interface used
- Front facing camera - works, with minor noise/distortion, standard interface (UVC) used
- Lighthouse power management
- Headset, controllers, Vive tracker (tracking) - part of original driver
- Headset/controllers firmware updates - part of original driver

### TODO

- Configuration utilities - most of reconfiguration (resolution, noise cancelation, brightness, lighthouse power management) abilities are already reverse-engineered, they just aren't easly configurable, some GUI utility should be written
- Focus knob overlay (Some third-party may work though). Focusing does work, but there is no visual helper.
- Audio output is not targeted to correct device yet (You need to manually switch it every time), it should be possible to implement this feature in this driver however
- Front facing camera noise - can be solved with some kernel tinkering (UVC driver)
- Front facing camera distortion - distortion matrix should be submitted to SteamVR
- Standalone driver/integration with OpenHMD/Monado - most complex parts (tracking) is already implemented in open source, only thing needed - is to port vive pro 2 specific features/tools
- Vive Wireless Adapter - support may be implemented, i have some ideas about how it should work, however i dont have one for testing, see donate section below, first received donations will be spent on one

### Will not work

- Vive Console software (GUI utilities etc) - this driver own utilities should be used instead


## Installation

This driver can be built using [nix package manager](https://nixos.org/download.html) (it works on any distribution), build is fully reproducible, all needed dependencies will be downloaded automatically:

```sh
# Note: the #driver-proxy-release part of this command is not a comment, it's just github syntax highlighter is wrong
nix build --extra-experimental-features nix-command --extra-experimental-features flakes .#driver-proxy-release
```

...or using manual building instructions from here (i dont provide any guarantees about contents of this repo) https://github.com/santeri3700/vive-pro-2-on-linux#install-vive-pro-2-linux-driver-by-certainlach

...or be downloaded from patreon page, see donate section below

And then installed via

```sh
./install.sh
```

Latest version of driver [automatically patches](https://github.com/CertainLach/VivePro2-Linux-Driver/commit/70687011f80d58c78ee77868895def9d77adf262) SteamVR, so VIVE Console no longer required to be installed

## Configuration

In `steamvr.vrsettings`:

`vivepro2.resolution`: `0-5`, 0 by default, to make it most compatible with every hardware

Reconfigures helmet to specified resolution/framerate before startup

- 0 - 2448x1224 90fps
- 1 - 2448x1224 120fps
- 2 - 3264x1632 90fps
- 3 - 3680x1836 90fps
- 4 - 4896x2448 90fps
- 5 - 4896x2448 120fps

Similar to original vive console utility

`vivepro2.brightness`: `1-130`, 130 by default

Display brightness

Original vive console seems to fade from 0 to 130 on start, and then no longer touch this setting

`vivepro2.noiseCancel`: `true/false`, disabled by default

Toggle built-in microphone noise canceling

Similar option exists in vive console

## Required kernel patches

- Mark HMD as non-display (Otherwise SteamVR will not be able to occupy display) https://lore.kernel.org/linux-kernel/20220118170037.14584-1-iam@lach.pw/ - should be fixed in kernel 5.18 with another patch

- Support type VII timings in DisplayID (VIVE reports mode in them) https://lore.kernel.org/linux-kernel/20220118215956.17229-1-iam@lach.pw/ - merged to kernel in 5.18

- Support fixed DSC BPP rate https://lore.kernel.org/linux-kernel/20220220151940.58327-1-iam@lach.pw/, https://lore.kernel.org/linux-kernel/20220220151940.58327-2-iam@lach.pw/ - not yet merged, currently planning to propose it to 5.19, without this patch highest resolution modes will not work

All of those patches are kept up-to-date in this repo, in `kernel-patches` subdirectory

I recommend you to use kernel 5.17+, as there is other issues in older kernels

If you use NixOS, then you can use kernelPatches from this flake:
```nix
boot.kernelPatches = vivepro2-linux-driver.kernelPatches;
```

If you use arch btw, then you can use this kernel package with all required patches applied (i have not tested it, and can't provide any guarantees about contents of this repo): https://github.com/santeri3700/vive-pro-2-on-linux

I don't recommend using other distributions, because it will be harder, because of usage of bleeding-edge kernel, but it should work, and I will fix any issues with them (I.e I have fixed usage of this driver on ubuntu)

## Donate

I dont have enough motivation making this thing work for everyone/adding features everyone wants/needs

You can, however, help me to develop this motivation, here: https://patreon.com/0lach

# Thanks

https://github.com/ChristophHaag for initial OpenVR guidance, which helped me to fix some issues

https://github.com/santeri3700 for writing driver/kernel build instructions here: https://github.com/santeri3700/vive-pro-2-on-linux (i don't provide any guarantees about contents of this repo)

Testers, backers, everyone else
