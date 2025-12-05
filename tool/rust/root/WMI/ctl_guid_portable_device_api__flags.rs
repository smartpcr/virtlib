// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source CtlGuidPortableDeviceAPI_Flags
//////////////////////////////////////////////

/// CtlGuidPortableDeviceAPI_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum CtlGuidPortableDeviceAPI_Flags {
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
    /// WPD_F1
    #[serde(rename = "WPD_F1")]
    WPDF1 = 9,
    /// Event
    #[serde(rename = "Event")]
    Event = 10,
    /// WPD_F3
    #[serde(rename = "WPD_F3")]
    WPDF3 = 11,
    /// Perf
    #[serde(rename = "Perf")]
    Perf = 12,
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
    /// PortableDeviceAPI_General
    #[serde(rename = "PortableDeviceAPI_General")]
    PortableDeviceAPIGeneral = 17,
    /// PortableDeviceAPI_DriverCommunication
    #[serde(rename = "PortableDeviceAPI_DriverCommunication")]
    PortableDeviceAPIDriverCommunication = 18,
    /// PortableDeviceAPI_Internal
    #[serde(rename = "PortableDeviceAPI_Internal")]
    PortableDeviceAPIInternal = 19,
    /// PortableDeviceAPI_Properties
    #[serde(rename = "PortableDeviceAPI_Properties")]
    PortableDeviceAPIProperties = 20,
    /// PortableDeviceAPI_Resources
    #[serde(rename = "PortableDeviceAPI_Resources")]
    PortableDeviceAPIResources = 21,
    /// PortableDeviceAPI_Enumeration
    #[serde(rename = "PortableDeviceAPI_Enumeration")]
    PortableDeviceAPIEnumeration = 22,
    /// PortableDeviceAPI_Events
    #[serde(rename = "PortableDeviceAPI_Events")]
    PortableDeviceAPIEvents = 23,
    /// PortableDeviceAPI_Service
    #[serde(rename = "PortableDeviceAPI_Service")]
    PortableDeviceAPIService = 24,
    /// PortableDeviceAPI_Automation
    #[serde(rename = "PortableDeviceAPI_Automation")]
    PortableDeviceAPIAutomation = 25,
}

impl Default for CtlGuidPortableDeviceAPI_Flags {
    fn default() -> Self {
        Self::WPDTRACELEVELCRITICAL
    }
}

