// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source BusError_Participation
//////////////////////////////////////////////

/// BusError_Participation enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum BusError_Participation {
    /// Local_Processor_Originated_Request
    #[serde(rename = "Local_Processor_Originated_Request")]
    LocalProcessorOriginatedRequest = 0,
    /// Local_Processor_Responded_To_Request
    #[serde(rename = "Local_Processor_Responded_To_Request")]
    LocalProcessorRespondedToRequest = 1,
    /// Local_Processor_Observed_Error_As_Third_Party
    #[serde(rename = "Local_Processor_Observed_Error_As_Third_Party")]
    LocalProcessorObservedErrorAsThirdParty = 2,
    /// Generic
    #[serde(rename = "Generic")]
    Generic = 3,
}

impl Default for BusError_Participation {
    fn default() -> Self {
        Self::LocalProcessorOriginatedRequest
    }
}

