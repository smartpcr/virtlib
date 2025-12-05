// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source CtlGuidPortableDeviceClassExtension_Flags
//////////////////////////////////////////////

/// CtlGuidPortableDeviceClassExtension_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum CtlGuidPortableDeviceClassExtension_Flags {
    /// WPD_TRACE_LEVEL_CRITICAL
    #[serde(rename = "WPD_TRACE_LEVEL_CRITICAL")]
    WPDTRACELEVELCRITICAL = 1,
    /// WPD_TRACE_LEVEL_ERROR
    #[serde(rename = "WPD_TRACE_LEVEL_ERROR")]
    WPDTRACELEVELERROR = 2,
    /// WPD_TRACE_LEVEL_WARNING
    #[serde(rename = "WPD_TRACE_LEVEL_WARNING")]
    WPDTRACELEVELWARNING = 3,
    /// WPD_TRACE_LEVEL_ASSERT
    #[serde(rename = "WPD_TRACE_LEVEL_ASSERT")]
    WPDTRACELEVELASSERT = 4,
    /// WPD_TRACE_LEVEL_FUNCTRACE
    #[serde(rename = "WPD_TRACE_LEVEL_FUNCTRACE")]
    WPDTRACELEVELFUNCTRACE = 5,
    /// WPD_TRACE_LEVEL_INFORMATION
    #[serde(rename = "WPD_TRACE_LEVEL_INFORMATION")]
    WPDTRACELEVELINFORMATION = 6,
    /// WPD_TRACE_LEVEL_VERBOSE
    #[serde(rename = "WPD_TRACE_LEVEL_VERBOSE")]
    WPDTRACELEVELVERBOSE = 7,
    /// WPD_TRACE_LEVEL_RESERVED
    #[serde(rename = "WPD_TRACE_LEVEL_RESERVED")]
    WPDTRACELEVELRESERVED = 8,
    /// WPD_Install
    #[serde(rename = "WPD_Install")]
    WPDInstall = 9,
    /// WPD_F2
    #[serde(rename = "WPD_F2")]
    WPDF2 = 10,
    /// WPD_F3
    #[serde(rename = "WPD_F3")]
    WPDF3 = 11,
    /// WPD_F4
    #[serde(rename = "WPD_F4")]
    WPDF4 = 12,
    /// WPD_F5
    #[serde(rename = "WPD_F5")]
    WPDF5 = 13,
    /// WPD_F6
    #[serde(rename = "WPD_F6")]
    WPDF6 = 14,
    /// WPD_F7
    #[serde(rename = "WPD_F7")]
    WPDF7 = 15,
    /// WPD_F8
    #[serde(rename = "WPD_F8")]
    WPDF8 = 16,
    /// PortableDeviceClassExtension_f1
    #[serde(rename = "PortableDeviceClassExtension_f1")]
    PortableDeviceClassExtensionF1 = 17,
    /// PortableDeviceClassExtension_f2
    #[serde(rename = "PortableDeviceClassExtension_f2")]
    PortableDeviceClassExtensionF2 = 18,
}

impl Default for CtlGuidPortableDeviceClassExtension_Flags {
    fn default() -> Self {
        Self::WPDTRACELEVELCRITICAL
    }
}

