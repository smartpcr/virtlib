// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_MaskingSetToInitiatorId struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_MaskingSetToInitiatorId {

/// 
    #[serde(rename = "InitiatorId")]
    pub initiator_id: Option<MSFT_InitiatorId>,

/// 
    #[serde(rename = "MaskingSet")]
    pub masking_set: Option<MSFT_MaskingSet>,
}

impl MSFT_MaskingSetToInitiatorId {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            initiator_id: None,
            masking_set: None,
        }
    }


    /// Sets the value of InitiatorId
    pub fn set_initiator_id(&mut self, value: MSFT_InitiatorId) {
        self.initiator_id = Some(value);
    }

    /// Gets the value of InitiatorId
    pub fn get_initiator_id(&self) -> Option<&MSFT_InitiatorId> {
        self.initiator_id.as_ref()
    }

    /// Sets the value of MaskingSet
    pub fn set_masking_set(&mut self, value: MSFT_MaskingSet) {
        self.masking_set = Some(value);
    }

    /// Gets the value of MaskingSet
    pub fn get_masking_set(&self) -> Option<&MSFT_MaskingSet> {
        self.masking_set.as_ref()
    }
}

