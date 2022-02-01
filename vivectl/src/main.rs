use anyhow::{bail, Context, Result};
use clap::{ArgEnum, Parser};
use hex_literal::hex;
use hidapi::{HidApi, HidDevice};
use std::str::FromStr;

fn send_str_report(handle: &HidDevice, is_alt: bool, report: &[u8]) -> Result<()> {
    let mut buf = [0u8; 64];
    buf[0] = 4;
    buf[1] = if is_alt { 0x71 } else { 0x70 };
    buf[2] = 0x29;
    buf[3] = report.len() as u8;
    buf[4..][..report.len()].copy_from_slice(report);
    handle.send_feature_report(&buf)?;
    Ok(())
}
// wireless,0

fn send_magic_report(handle: &HidDevice, report: &[u8]) -> Result<()> {
    let mut buf = [0u8; 64];
    buf[0..][..report.len()].copy_from_slice(report);
    handle.send_feature_report(&buf)?;
    Ok(())
}
// Unknown magic requests:
//      0478293808000000000000000000000100000000000000000000000000000000
//      0000000000000000000000000000000000000000000000000000000000000000
//
//      0478293801000000000000000100000000000000000000000000000000000000
//      0000000000000000000000000000000000000000000000000000000000000000

/// Unused, as there is no useful (known) information here (only edid vid/pid)
/// Often called by official software, maybe used as ping?
fn recv_report(handle: &HidDevice) -> Result<[u8; 64]> {
    let mut buf = [0u8; 64];
    buf[0] = 4;
    handle.get_feature_report(&mut buf)?;
    Ok(buf)
}

fn toggle_mic_noise_cancel(device: &HidDevice, enabled: bool) -> Result<()> {
    if enabled {
        send_str_report(&device, true, b"codecreg=9c9,80")?;
        send_str_report(&device, true, b"codecreg=9c8,a5")?;
        send_str_report(&device, true, b"codecreg=9d0,a4")?;
        send_str_report(&device, true, b"codecreg=1c008f,1")?;
        send_str_report(&device, true, b"codecreg=1c0005,9")?;
        send_str_report(&device, true, b"codecreg=1c0005,8000")?;
    } else {
        send_str_report(&device, true, b"codecreg=9c9,8c")?;
        send_str_report(&device, true, b"codecreg=9c8,a4")?;
        send_str_report(&device, true, b"codecreg=9d0,0")?;
        send_str_report(&device, true, b"codecreg=1c008f,0")?;
        send_str_report(&device, true, b"codecreg=1c0005,9")?;
        send_str_report(&device, true, b"codecreg=1c0005,8000")?;
    }
    Ok(())
}

struct Brightness(u8);
impl FromStr for Brightness {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: u8 = u8::from_str(s).context("failed to parse brightness")?;
        if v > 130 {
            bail!("max brightness = 130")
        }
        Ok(Self(v))
    }
}
fn set_brightness(device: &HidDevice, brightness: Brightness) -> Result<()> {
    send_str_report(
        &device,
        false,
        format!("setbrightness,{}", brightness.0).as_bytes(),
    )?;
    Ok(())
}

#[derive(Clone, Copy, ArgEnum)]
enum LedState {
    /// Bright green
    On,
    /// Green
    Sleep,
}
impl LedState {
    fn magic(&self) -> [u8; 64] {
        match self {
            Self::On => hex! {"
                0478293800000000000002000000000000000000000000000000000000000000
                000000000000000000000000007a000000000000000000000000000000000000
            "},
            Self::Sleep => hex! {"
                0478293800000000000002000000000000000000000000000000000000000000
                000000000000000000000000007b000000000000000000000000000000000000
                //                         ^- vive console also sends c here, but i see no difference between b and c
                //                            perhaps brightness, but hardware only supports 2 levels?
            "},
            // Also used on bootup, i see no effect
            // Self::Idk => hex!("
            //     0478293800000000000000000000000000000000000000000000000000000000
            //     0000000000000000000000000000000000000000000000000000000000000000
            // ")
        }
    }
}
fn set_led(device: &HidDevice, state: LedState) -> Result<()> {
    send_magic_report(&device, &state.magic())?;
    Ok(())
}

fn reset_dp(device: &HidDevice) -> Result<()> {
    send_str_report(device, false, b"chipreset")?;
    Ok(())
}

/// Hardcoded in official software too
#[derive(Clone, Copy, ArgEnum)]
#[repr(u8)]
enum Resolution {
    R2448x1224f90 = 0,
    R2448x1224f120 = 1,
    R3264x1632f90 = 2,
    R3680x1836f90 = 3,
    R4896x2448f90 = 4,
    R4896x2448f120 = 5,
}

fn set_resolution(device: &HidDevice, resolution: Resolution) -> Result<()> {
    // If not sent - something is wrong
    send_str_report(&device, false, b"wireless,0")?;
    send_str_report(
        &device,
        false,
        &format!("dtd,{}", resolution as u8).as_bytes(),
    )?;
    Ok(())
}

fn set_mode(device: &HidDevice, mst: bool, dsc: bool, amdgpu: bool) -> Result<()> {
    send_str_report(
        &device,
        false,
        &format!(
            "mode,{},{},{}",
            if mst { '1' } else { '0' },
            if dsc { '1' } else { '0' },
            if amdgpu { '1' } else { '0' }
        )
        .as_bytes(),
    )
}

#[derive(Clone, Copy, ArgEnum)]
enum NoiseCancelState {
    Disable,
    Enable,
}

#[derive(Parser)]
#[clap(author, version)]
enum Args {
    /// Reconnect HMD
    ResetDp,
    /// Restart HMD with new reported resolution
    Mode {
        /// Multi-stream-transport
        #[clap(long)]
        mst: bool,
        /// Display stream compression
        #[clap(long)]
        dsc: bool,
        /// ?
        #[clap(long)]
        amdgpu: bool,
        #[clap(arg_enum)]
        resolution: Resolution,
    },
    /// Change display brightness
    SetBrightness {
        #[clap(parse(try_from_str))]
        value: Brightness,
    },
    /// Enable/disable microphone noise cancellation
    NoiseCancel {
        #[clap(arg_enum)]
        state: NoiseCancelState,
    },
    Led {
        #[clap(arg_enum)]
        state: LedState,
    },
    Payload {
        #[clap(long)]
        alt: bool,
        value: String,
    },
    /// Testing only, performs full initialization sequence (captured on windows)
    Magic,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let manager = HidApi::new()?;
    let device = manager.open(0x0bb4, 0x0342)?;
    match args {
        Args::ResetDp => reset_dp(&device)?,
        Args::Mode {
            resolution,
            mst,
            dsc,
            amdgpu,
        } => {
            set_mode(&device, mst, dsc, amdgpu)?;
            set_resolution(&device, resolution)?;
            // set_mode does strage things without DP reset
            reset_dp(&device)?;
            // TODO: Wait for device to reappear?
        }
        Args::SetBrightness { value } => {
            set_brightness(&device, value)?;
        }
        Args::NoiseCancel { state } => {
            toggle_mic_noise_cancel(&device, matches!(state, NoiseCancelState::Enable))?;
        }
        Args::Led { state } => {
            set_led(&device, state)?;
        }
        Args::Payload { alt, value } => send_str_report(&device, alt, value.as_bytes())?,
        Args::Magic => {
            send_magic_report(
                &device,
                &hex! {"
                    0478293801000000000000000000000000000000000000000000000000000000
                    0000000000000000000000000000000000000000000000000000000000000000
                "},
            )?;
            send_magic_report(
                &device,
                &hex! {"
                    0478293801000000000000000100000000000000000000000000000000000000
                    0000000000000000000000000000000000000000000000000000000000000000
                "},
            )?;
            send_magic_report(
                &device,
                &hex! {"
                    04782938010000000000000001725f766976655fb2551e4d6ff1cf1188cb0011
                    11000030686463fa9b020000b8dc63fa9b020000b2551e4d6ff1cf1100000000
                "},
            )?;
            set_led(&device, LedState::On)?;
            set_brightness(&device, Brightness(130))?;
            send_magic_report(
                &device,
                &hex! {"
                    0478293808000000000000000000000100000000000000000000000000000000
                    0000000000000000000000000000000000000000000000000000000000000000
                "},
            )?;
            toggle_mic_noise_cancel(&device, false)?;
            send_magic_report(
                &device,
                &hex! {"
                    0478293800000000000000000000000000000000000000000000000000000000
                    0000000000000000000000000000000000000000000000000000000000000000
                "},
            )?;
            set_resolution(&device, Resolution::R2448x1224f90)?;
            reset_dp(&device)?;
        }
    }
    Ok(())
}
