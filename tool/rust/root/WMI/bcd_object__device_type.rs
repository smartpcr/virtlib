// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source BcdObject_DeviceType
//////////////////////////////////////////////

/// BcdObject_DeviceType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum BcdObject_DeviceType {
    /// BootDevice
    #[serde(rename = "BootDevice")]
    BootDevice = 1,
    /// PartitionDevice
    #[serde(rename = "PartitionDevice")]
    PartitionDevice = 2,
    /// FileDevice
    #[serde(rename = "FileDevice")]
    FileDevice = 3,
    /// RamdiskDevice
    #[serde(rename = "RamdiskDevice")]
    RamdiskDevice = 4,
    /// UnknownDevice
    #[serde(rename = "UnknownDevice")]
    UnknownDevice = 5,
}

impl Default for BcdObject_DeviceType {
    fn default() -> Self {
        Self::BootDevice
    }
}

