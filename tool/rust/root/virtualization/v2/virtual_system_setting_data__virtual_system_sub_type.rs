// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source VirtualSystemSettingData_VirtualSystemSubType
//////////////////////////////////////////////

/// VirtualSystemSettingData_VirtualSystemSubType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum VirtualSystemSettingData_VirtualSystemSubType {
    /// Microsoft_Hyper_V_SubType_1
    #[serde(rename = "Microsoft_Hyper_V_SubType_1")]
    MicrosoftHyperVSubType1 = 0,
    /// Microsoft_Hyper_V_SubType_2
    #[serde(rename = "Microsoft_Hyper_V_SubType_2")]
    MicrosoftHyperVSubType2 = 1,
}

impl Default for VirtualSystemSettingData_VirtualSystemSubType {
    fn default() -> Self {
        Self::MicrosoftHyperVSubType1
    }
}

