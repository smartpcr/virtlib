// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source EncryptableVolume_VolumeType
//////////////////////////////////////////////

/// EncryptableVolume_VolumeType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum EncryptableVolume_VolumeType {
    /// OSVolume
    #[serde(rename = "OSVolume")]
    OSVolume = 0,
    /// FixedDataVolume
    #[serde(rename = "FixedDataVolume")]
    FixedDataVolume = 1,
    /// PortableDataVolume
    #[serde(rename = "PortableDataVolume")]
    PortableDataVolume = 2,
}

impl Default for EncryptableVolume_VolumeType {
    fn default() -> Self {
        Self::OSVolume
    }
}

