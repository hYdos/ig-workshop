use crate::core::ig_core_platform::IG_CORE_PLATFORM::*;
use std::fmt::{Display, Formatter};

///
/// Used to associate other objects with platform. Critical for reading .igz files due to them being processed memory dumps.
///
#[derive(Debug, PartialEq, Clone)]
pub enum IG_CORE_PLATFORM {
    IG_CORE_PLATFORM_DEFAULT,
    IG_CORE_PLATFORM_DEPRECATED,
    IG_CORE_PLATFORM_WIN32,
    IG_CORE_PLATFORM_WII,
    IG_CORE_PLATFORM_DURANGO,
    IG_CORE_PLATFORM_ASPEN,
    IG_CORE_PLATFORM_XENON,
    IG_CORE_PLATFORM_PS3,
    IG_CORE_PLATFORM_OSX,
    IG_CORE_PLATFORM_WIN64,
    IG_CORE_PLATFORM_CAFE,
    IG_CORE_PLATFORM_NGP,
    IG_CORE_PLATFORM_MARMALADE,
    IG_CORE_PLATFORM_RASPI,
    IG_CORE_PLATFORM_ANDROID,
    IG_CORE_PLATFORM_ASPEN64,
    IG_CORE_PLATFORM_LGTV,
    IG_CORE_PLATFORM_PS4,
    IG_CORE_PLATFORM_WP8,
    IG_CORE_PLATFORM_LINUX,
    IG_CORE_PLATFORM_NX,
    /// This variant indicates that there can be no more valid platforms beyond this limit.
    IG_CORE_PLATFORM_MAX,
}

impl IG_CORE_PLATFORM {
    pub const IG_CORE_PLATFORM_DEPRECATED_2: IG_CORE_PLATFORM = IG_CORE_PLATFORM_DEPRECATED;
    pub const IG_CORE_PLATFORM_DEPRECATED_3: IG_CORE_PLATFORM = IG_CORE_PLATFORM_DEPRECATED;
    pub const IG_CORE_PLATFORM_DEPRECATED_4: IG_CORE_PLATFORM = IG_CORE_PLATFORM_DEPRECATED;
}

impl Display for IG_CORE_PLATFORM {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IG_CORE_PLATFORM_DEFAULT => f.write_str("Default"),
            IG_CORE_PLATFORM_DEPRECATED => f.write_str("Deprecated"),
            IG_CORE_PLATFORM_WIN32 => f.write_str("Windows 32-bit"),
            IG_CORE_PLATFORM_WII => f.write_str("Wii"),
            IG_CORE_PLATFORM_DURANGO => f.write_str("Xbox One"),
            IG_CORE_PLATFORM_ASPEN => f.write_str("iOS 32-bit"),
            IG_CORE_PLATFORM_XENON => f.write_str("Xbox 360"),
            IG_CORE_PLATFORM_PS3 => f.write_str("PlayStation 3"),
            IG_CORE_PLATFORM_OSX => f.write_str("MacOS 32-bit"),
            IG_CORE_PLATFORM_WIN64 => f.write_str("Windows 64-bit"),
            IG_CORE_PLATFORM_CAFE => f.write_str("WiiU"),
            IG_CORE_PLATFORM_NGP => f.write_str("PlayStation Vita"),
            IG_CORE_PLATFORM_MARMALADE => f.write_str("IG_CORE_PLATFORM_MARMALADE"),
            IG_CORE_PLATFORM_RASPI => f.write_str("Raspberry Pi"),
            IG_CORE_PLATFORM_ANDROID => f.write_str("Android 32-bit"),
            IG_CORE_PLATFORM_ASPEN64 => f.write_str("iOS 64-bit"),
            IG_CORE_PLATFORM_LGTV => f.write_str("LG Smart TV"),
            IG_CORE_PLATFORM_PS4 => f.write_str("PlayStation 4"),
            IG_CORE_PLATFORM_WP8 => f.write_str("Windows8 Phone"),
            IG_CORE_PLATFORM_LINUX => f.write_str("Linux"),
            IG_CORE_PLATFORM_NX => f.write_str("Switch"),
            _ => f.write_str(format!("{:?}", self).as_str()),
        }
    }
}

impl TryFrom<String> for IG_CORE_PLATFORM {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        self::IG_CORE_PLATFORM::try_from(value.as_str()).map_err(|_| ())
    }
}

impl TryFrom<&str> for IG_CORE_PLATFORM {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "IG_CORE_PLATFORM_DEFAULT" => Ok(IG_CORE_PLATFORM_DEFAULT),
            "IG_CORE_PLATFORM_DEPRECATED" => Ok(IG_CORE_PLATFORM_DEPRECATED),
            "IG_CORE_PLATFORM_DEPRECATED_2" => Ok(Self::IG_CORE_PLATFORM_DEPRECATED_2),
            "IG_CORE_PLATFORM_DEPRECATED_3" => Ok(Self::IG_CORE_PLATFORM_DEPRECATED_3),
            "IG_CORE_PLATFORM_DEPRECATED_4" => Ok(Self::IG_CORE_PLATFORM_DEPRECATED_4),
            "IG_CORE_PLATFORM_WIN32" => Ok(IG_CORE_PLATFORM_WIN32),
            "IG_CORE_PLATFORM_WII" => Ok(IG_CORE_PLATFORM_WII),
            "IG_CORE_PLATFORM_DURANGO" => Ok(IG_CORE_PLATFORM_DURANGO),
            "IG_CORE_PLATFORM_ASPEN" => Ok(IG_CORE_PLATFORM_ASPEN),
            "IG_CORE_PLATFORM_XENON" => Ok(IG_CORE_PLATFORM_XENON),
            "IG_CORE_PLATFORM_PS3" => Ok(IG_CORE_PLATFORM_PS3),
            "IG_CORE_PLATFORM_OSX" => Ok(IG_CORE_PLATFORM_OSX),
            "IG_CORE_PLATFORM_WIN64" => Ok(IG_CORE_PLATFORM_WIN64),
            "IG_CORE_PLATFORM_CAFE" => Ok(IG_CORE_PLATFORM_CAFE),
            "IG_CORE_PLATFORM_NGP" => Ok(IG_CORE_PLATFORM_NGP),
            "IG_CORE_PLATFORM_MARMALADE" => Ok(IG_CORE_PLATFORM_MARMALADE),
            "IG_CORE_PLATFORM_RASPI" => Ok(IG_CORE_PLATFORM_RASPI),
            "IG_CORE_PLATFORM_ANDROID" => Ok(IG_CORE_PLATFORM_ANDROID),
            "IG_CORE_PLATFORM_ASPEN64" => Ok(IG_CORE_PLATFORM_ASPEN64),
            "IG_CORE_PLATFORM_LGTV" => Ok(IG_CORE_PLATFORM_LGTV),
            "IG_CORE_PLATFORM_PS4" => Ok(IG_CORE_PLATFORM_PS4),
            "IG_CORE_PLATFORM_WP8" => Ok(IG_CORE_PLATFORM_WP8),
            "IG_CORE_PLATFORM_LINUX" => Ok(IG_CORE_PLATFORM_LINUX),
            "IG_CORE_PLATFORM_MAX" => Ok(IG_CORE_PLATFORM_MAX),
            _ => Err(()),
        }
    }
}
