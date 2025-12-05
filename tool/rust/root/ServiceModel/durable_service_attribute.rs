// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// DurableServiceAttribute struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DurableServiceAttribute {
    #[serde(flatten)]
    pub base: Behavior,

/// The SaveStateInOperationTransaction boolean in DurableServiceAttribute
    #[serde(rename = "SaveStateInOperationTransaction")]
    pub save_state_in_operation_transaction: Option<bool>,
}

impl DurableServiceAttribute {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Behavior::new(),
            save_state_in_operation_transaction: None,
        }
    }


    /// Sets the value of SaveStateInOperationTransaction
    pub fn set_save_state_in_operation_transaction(&mut self, value: bool) {
        self.save_state_in_operation_transaction = Some(value);
    }

    /// Gets the value of SaveStateInOperationTransaction
    pub fn get_save_state_in_operation_transaction(&self) -> Option<&bool> {
        self.save_state_in_operation_transaction.as_ref()
    }
}

