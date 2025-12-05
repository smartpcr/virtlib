// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// TransactionFlowAttribute struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TransactionFlowAttribute {
    #[serde(flatten)]
    pub base: Behavior,

/// Indicates whether transactions flow or not.
    #[serde(rename = "TransactionFlowOption")]
    pub transaction_flow_option: Option<String>,
}

impl TransactionFlowAttribute {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Behavior::new(),
            transaction_flow_option: None,
        }
    }


    /// Sets the value of TransactionFlowOption
    pub fn set_transaction_flow_option(&mut self, value: String) {
        self.transaction_flow_option = Some(value);
    }

    /// Gets the value of TransactionFlowOption
    pub fn get_transaction_flow_option(&self) -> Option<&String> {
        self.transaction_flow_option.as_ref()
    }
}

