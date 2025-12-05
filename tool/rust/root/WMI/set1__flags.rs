// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Set1_Flags
//////////////////////////////////////////////

/// Set1_Flags enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Set1_Flags {
    /// create
    #[serde(rename = "create")]
    Create = 1,
    /// createnamedpipe
    #[serde(rename = "createnamedpipe")]
    Createnamedpipe = 2,
    /// close
    #[serde(rename = "close")]
    Close = 3,
    /// read
    #[serde(rename = "read")]
    Read = 4,
    /// write
    #[serde(rename = "write")]
    Write = 5,
    /// queryinfo
    #[serde(rename = "queryinfo")]
    Queryinfo = 6,
    /// setinfo
    #[serde(rename = "setinfo")]
    Setinfo = 7,
    /// queryea
    #[serde(rename = "queryea")]
    Queryea = 8,
    /// setea
    #[serde(rename = "setea")]
    Setea = 9,
    /// flushbuffers
    #[serde(rename = "flushbuffers")]
    Flushbuffers = 10,
    /// queryvolinfo
    #[serde(rename = "queryvolinfo")]
    Queryvolinfo = 11,
    /// setvolinfo
    #[serde(rename = "setvolinfo")]
    Setvolinfo = 12,
    /// directorycontrol
    #[serde(rename = "directorycontrol")]
    Directorycontrol = 13,
    /// filesystemcontrol
    #[serde(rename = "filesystemcontrol")]
    Filesystemcontrol = 14,
    /// devicecontrol
    #[serde(rename = "devicecontrol")]
    Devicecontrol = 15,
    /// internaldevicecontrol
    #[serde(rename = "internaldevicecontrol")]
    Internaldevicecontrol = 16,
    /// shutdown
    #[serde(rename = "shutdown")]
    Shutdown = 17,
    /// lockcontrol
    #[serde(rename = "lockcontrol")]
    Lockcontrol = 18,
    /// cleanup
    #[serde(rename = "cleanup")]
    Cleanup = 19,
    /// createmailslot
    #[serde(rename = "createmailslot")]
    Createmailslot = 20,
    /// querysecurity
    #[serde(rename = "querysecurity")]
    Querysecurity = 21,
    /// setsecurity
    #[serde(rename = "setsecurity")]
    Setsecurity = 22,
    /// power
    #[serde(rename = "power")]
    Power = 23,
    /// systemcontrol
    #[serde(rename = "systemcontrol")]
    Systemcontrol = 24,
    /// devicechange
    #[serde(rename = "devicechange")]
    Devicechange = 25,
    /// queryquota
    #[serde(rename = "queryquota")]
    Queryquota = 26,
    /// setquota
    #[serde(rename = "setquota")]
    Setquota = 27,
    /// pnp
    #[serde(rename = "pnp")]
    Pnp = 28,
}

impl Default for Set1_Flags {
    fn default() -> Self {
        Self::Create
    }
}

