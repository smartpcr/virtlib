// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source BcdDeviceData_DeviceType
//////////////////////////////////////////////

/// BcdDeviceData_DeviceType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum BcdDeviceData_DeviceType {
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
    /// QualifiedPartition
    #[serde(rename = "QualifiedPartition")]
    QualifiedPartition = 6,
    /// LocateDevice
    #[serde(rename = "LocateDevice")]
    LocateDevice = 7,
    /// LocateExDevice
    #[serde(rename = "LocateExDevice")]
    LocateExDevice = 8,
}

impl Default for BcdDeviceData_DeviceType {
    fn default() -> Self {
        Self::BootDevice
    }
}

