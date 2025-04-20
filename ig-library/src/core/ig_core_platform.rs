use crate::core::ig_core_platform::IG_CORE_PLATFORM::*;
use crate::core::meta::ig_metadata_manager::MetaEnumImpl;
use ig_proc_macros::MetaEnum;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

///
/// Used to associate other objects with platform.
///
#[derive(Debug, Hash, PartialEq, Eq, Clone, MetaEnum)]
pub enum IG_CORE_PLATFORM {
    IG_CORE_PLATFORM_DEFAULT,
    /// Any platform that is marked for removal in alchemy.
    IG_CORE_PLATFORM_DEPRECATED,
    /// Windows 32-bit
    IG_CORE_PLATFORM_WIN32,
    /// Wii
    IG_CORE_PLATFORM_WII,
    /// Xbox One
    IG_CORE_PLATFORM_DURANGO,
    /// iOS 32-bit
    IG_CORE_PLATFORM_ASPEN,
    /// Xbox 360
    IG_CORE_PLATFORM_XENON,
    /// Playstation 3
    IG_CORE_PLATFORM_PS3,
    /// MacOS
    IG_CORE_PLATFORM_OSX,
    /// Windows 64-bit
    IG_CORE_PLATFORM_WIN64,
    /// WiiU
    IG_CORE_PLATFORM_CAFE,
    /// Playstation Vita
    IG_CORE_PLATFORM_NGP,
    /// unknown platform
    IG_CORE_PLATFORM_MARMALADE,
    /// Raspberry Pi
    IG_CORE_PLATFORM_RASPI,
    /// Android
    IG_CORE_PLATFORM_ANDROID,
    /// iOS 64-bit
    IG_CORE_PLATFORM_ASPEN64,
    /// LG tv
    IG_CORE_PLATFORM_LGTV,
    /// Playstation 4
    IG_CORE_PLATFORM_PS4,
    /// Windows 8 phone
    IG_CORE_PLATFORM_WP8,
    /// Linux
    IG_CORE_PLATFORM_LINUX,
    /// Nintendo Switch
    IG_CORE_PLATFORM_NX,
    /// This variant indicates that there can be no more valid platforms beyond this limit.
    IG_CORE_PLATFORM_MAX,
}

impl IG_CORE_PLATFORM {
    pub const IG_CORE_PLATFORM_DEPRECATED_2: IG_CORE_PLATFORM = IG_CORE_PLATFORM_DEPRECATED;
    pub const IG_CORE_PLATFORM_DEPRECATED_3: IG_CORE_PLATFORM = IG_CORE_PLATFORM_DEPRECATED;
    pub const IG_CORE_PLATFORM_DEPRECATED_4: IG_CORE_PLATFORM = IG_CORE_PLATFORM_DEPRECATED;

    pub fn get_pointer_size(&self) -> usize {
        if self.is_64bit() {
            8
        } else {
            4
        }
    }

    pub fn is_64bit(&self) -> bool {
        match &self {
            IG_CORE_PLATFORM_DEFAULT
            | IG_CORE_PLATFORM_DEPRECATED
            | IG_CORE_PLATFORM_WIN32
            | IG_CORE_PLATFORM_WII
            | IG_CORE_PLATFORM_ASPEN
            | IG_CORE_PLATFORM_XENON
            | IG_CORE_PLATFORM_PS3
            | IG_CORE_PLATFORM_OSX
            | IG_CORE_PLATFORM_CAFE
            | IG_CORE_PLATFORM_NGP
            | IG_CORE_PLATFORM_MARMALADE
            | IG_CORE_PLATFORM_RASPI
            | IG_CORE_PLATFORM_ANDROID
            | IG_CORE_PLATFORM_LGTV
            | IG_CORE_PLATFORM_MAX => false,

            IG_CORE_PLATFORM_DURANGO
            | IG_CORE_PLATFORM_WIN64
            | IG_CORE_PLATFORM_ASPEN64
            | IG_CORE_PLATFORM_PS4
            | IG_CORE_PLATFORM_WP8
            | IG_CORE_PLATFORM_NX
            | IG_CORE_PLATFORM_LINUX => true,
        }
    }
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
        IG_CORE_PLATFORM::from_str(value.as_str())
    }
}

impl FromStr for IG_CORE_PLATFORM {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
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
