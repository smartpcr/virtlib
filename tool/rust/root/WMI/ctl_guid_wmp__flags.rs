// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source CtlGuidWMP_Flags
//////////////////////////////////////////////

/// CtlGuidWMP_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum CtlGuidWMP_Flags {
    /// _DH_DEFAULT
    #[serde(rename = "_DH_DEFAULT")]
    DHDEFAULT = 1,
    /// _DH_WMP_DPF
    #[serde(rename = "_DH_WMP_DPF")]
    DHWMPDPF = 2,
    /// _DH_WMPGRAPH
    #[serde(rename = "_DH_WMPGRAPH")]
    DHWMPGRAPH = 3,
    /// _DH_WMPRENDER
    #[serde(rename = "_DH_WMPRENDER")]
    DHWMPRENDER = 4,
    /// _DH_PLUGINS
    #[serde(rename = "_DH_PLUGINS")]
    DHPLUGINS = 5,
    /// _DH_PLAYER_FULLSCREEN
    #[serde(rename = "_DH_PLAYER_FULLSCREEN")]
    DHPLAYERFULLSCREEN = 6,
    /// _DH_WMPDMOWRAPPER
    #[serde(rename = "_DH_WMPDMOWRAPPER")]
    DHWMPDMOWRAPPER = 7,
    /// _DH_WMPCD_DPF
    #[serde(rename = "_DH_WMPCD_DPF")]
    DHWMPCDDPF = 8,
    /// _DH_LAYOUTMAN_DOCKING
    #[serde(rename = "_DH_LAYOUTMAN_DOCKING")]
    DHLAYOUTMANDOCKING = 9,
    /// _DH_WMHTML
    #[serde(rename = "_DH_WMHTML")]
    DHWMHTML = 10,
    /// _DH_WMP_SYNC
    #[serde(rename = "_DH_WMP_SYNC")]
    DHWMPSYNC = 11,
    /// _DH_WMP_DEVICE_CAPS
    #[serde(rename = "_DH_WMP_DEVICE_CAPS")]
    DHWMPDEVICECAPS = 12,
    /// _DH_WMP_DRM
    #[serde(rename = "_DH_WMP_DRM")]
    DHWMPDRM = 13,
    /// _DH_WMP_DEVICES
    #[serde(rename = "_DH_WMP_DEVICES")]
    DHWMPDEVICES = 14,
    /// _DH_WMP_CDBURN
    #[serde(rename = "_DH_WMP_CDBURN")]
    DHWMPCDBURN = 15,
    /// _DH_WMP_CACHE_PROVIDER
    #[serde(rename = "_DH_WMP_CACHE_PROVIDER")]
    DHWMPCACHEPROVIDER = 16,
    /// _DH_WMP_LIBRARY
    #[serde(rename = "_DH_WMP_LIBRARY")]
    DHWMPLIBRARY = 17,
    /// _DH_WMP_SYNCMGR
    #[serde(rename = "_DH_WMP_SYNCMGR")]
    DHWMPSYNCMGR = 18,
    /// _DH_WMP_SYNCWIZ
    #[serde(rename = "_DH_WMP_SYNCWIZ")]
    DHWMPSYNCWIZ = 19,
    /// _DH_WMP_LISL
    #[serde(rename = "_DH_WMP_LISL")]
    DHWMPLISL = 20,
    /// _DH_WMP_SYNC_STATUS
    #[serde(rename = "_DH_WMP_SYNC_STATUS")]
    DHWMPSYNCSTATUS = 21,
    /// _DH_WMP_HME
    #[serde(rename = "_DH_WMP_HME")]
    DHWMPHME = 22,
    /// _DH_WMP_MLS
    #[serde(rename = "_DH_WMP_MLS")]
    DHWMPMLS = 23,
    /// _DH_WMP_FIRMWARE_UPDATE
    #[serde(rename = "_DH_WMP_FIRMWARE_UPDATE")]
    DHWMPFIRMWAREUPDATE = 24,
    /// _DH_WMP_SYNC_PERF
    #[serde(rename = "_DH_WMP_SYNC_PERF")]
    DHWMPSYNCPERF = 25,
    /// _DH_WMP_SQM
    #[serde(rename = "_DH_WMP_SQM")]
    DHWMPSQM = 26,
}

impl Default for CtlGuidWMP_Flags {
    fn default() -> Self {
        Self::DHDEFAULT
    }
}

