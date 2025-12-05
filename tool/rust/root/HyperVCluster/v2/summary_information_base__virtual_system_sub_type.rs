// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source SummaryInformationBase_VirtualSystemSubType
//////////////////////////////////////////////

/// SummaryInformationBase_VirtualSystemSubType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum SummaryInformationBase_VirtualSystemSubType {
    /// Microsoft_Hyper_V_SubType_1
    #[serde(rename = "Microsoft_Hyper_V_SubType_1")]
    MicrosoftHyperVSubType1 = 0,
    /// Microsoft_Hyper_V_SubType_2
    #[serde(rename = "Microsoft_Hyper_V_SubType_2")]
    MicrosoftHyperVSubType2 = 1,
}

impl Default for SummaryInformationBase_VirtualSystemSubType {
    fn default() -> Self {
        Self::MicrosoftHyperVSubType1
    }
}

