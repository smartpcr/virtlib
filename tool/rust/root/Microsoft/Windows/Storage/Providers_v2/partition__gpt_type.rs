// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source Partition_GptType
//////////////////////////////////////////////

/// Partition_GptType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum Partition_GptType {
    /// System_Partition
    #[serde(rename = "System_Partition")]
    SystemPartition = 1,
    /// Microsoft_Reserved
    #[serde(rename = "Microsoft_Reserved")]
    MicrosoftReserved = 2,
    /// Basic_data
    #[serde(rename = "Basic_data")]
    BasicData = 3,
    /// LDM_Metadata
    #[serde(rename = "LDM_Metadata")]
    LDMMetadata = 4,
    /// LDM_Data
    #[serde(rename = "LDM_Data")]
    LDMData = 5,
    /// Microsoft_Recovery
    #[serde(rename = "Microsoft_Recovery")]
    MicrosoftRecovery = 6,
}

impl Default for Partition_GptType {
    fn default() -> Self {
        Self::SystemPartition
    }
}

