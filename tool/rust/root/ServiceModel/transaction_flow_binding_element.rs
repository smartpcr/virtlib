// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// TransactionFlowBindingElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TransactionFlowBindingElement {
    #[serde(flatten)]
    pub base: BindingElement,

/// Indicates whether support for wildcard action with transaction flow is enabled.
    #[serde(rename = "AllowWildcardAction")]
    pub allow_wildcard_action: Option<bool>,

/// Indicates whether incoming transaction flow is enabled.
    #[serde(rename = "TransactionFlow")]
    pub transaction_flow: Option<bool>,

/// The transaction protocol used in flowing a transaction.
    #[serde(rename = "TransactionProtocol")]
    pub transaction_protocol: Option<String>,
}

impl TransactionFlowBindingElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: BindingElement::new(),
            allow_wildcard_action: None,
            transaction_flow: None,
            transaction_protocol: None,
        }
    }


    /// Sets the value of AllowWildcardAction
    pub fn set_allow_wildcard_action(&mut self, value: bool) {
        self.allow_wildcard_action = Some(value);
    }

    /// Gets the value of AllowWildcardAction
    pub fn get_allow_wildcard_action(&self) -> Option<&bool> {
        self.allow_wildcard_action.as_ref()
    }

    /// Sets the value of TransactionFlow
    pub fn set_transaction_flow(&mut self, value: bool) {
        self.transaction_flow = Some(value);
    }

    /// Gets the value of TransactionFlow
    pub fn get_transaction_flow(&self) -> Option<&bool> {
        self.transaction_flow.as_ref()
    }

    /// Sets the value of TransactionProtocol
    pub fn set_transaction_protocol(&mut self, value: String) {
        self.transaction_protocol = Some(value);
    }

    /// Gets the value of TransactionProtocol
    pub fn get_transaction_protocol(&self) -> Option<&String> {
        self.transaction_protocol.as_ref()
    }
}

