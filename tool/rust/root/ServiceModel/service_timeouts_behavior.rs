// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ServiceTimeoutsBehavior struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceTimeoutsBehavior {
    #[serde(flatten)]
    pub base: Behavior,

/// The period within which a transaction must complete.
    #[serde(rename = "TransactionTimeout")]
    pub transaction_timeout: Option<String>,
}

impl ServiceTimeoutsBehavior {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Behavior::new(),
            transaction_timeout: None,
        }
    }


    /// Sets the value of TransactionTimeout
    pub fn set_transaction_timeout(&mut self, value: String) {
        self.transaction_timeout = Some(value);
    }

    /// Gets the value of TransactionTimeout
    pub fn get_transaction_timeout(&self) -> Option<&String> {
        self.transaction_timeout.as_ref()
    }
}

