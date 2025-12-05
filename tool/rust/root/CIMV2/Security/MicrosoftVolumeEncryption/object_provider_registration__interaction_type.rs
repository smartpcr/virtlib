// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source ObjectProviderRegistration_InteractionType
//////////////////////////////////////////////

/// ObjectProviderRegistration_InteractionType enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum ObjectProviderRegistration_InteractionType {
    /// Pull
    #[serde(rename = "Pull")]
    Pull = 0,
    /// Push
    #[serde(rename = "Push")]
    Push = 1,
    /// PushVerify
    #[serde(rename = "PushVerify")]
    PushVerify = 2,
}

impl Default for ObjectProviderRegistration_InteractionType {
    fn default() -> Self {
        Self::Pull
    }
}

