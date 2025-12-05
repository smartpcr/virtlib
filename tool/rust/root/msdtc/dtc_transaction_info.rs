// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.msdtc
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// DtcTransactionInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DtcTransactionInfo {

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "IsolationLevel")]
    pub isolation_level: Option<u32>,

/// 
    #[serde(rename = "Parent")]
    pub parent: Option<String>,

/// 
    #[serde(rename = "State")]
    pub state: Option<String>,

/// 
    #[serde(rename = "TransactionId")]
    pub transaction_id: Option<String>,
}

impl DtcTransactionInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            description: None,
            isolation_level: None,
            parent: None,
            state: None,
            transaction_id: None,
        }
    }


    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of IsolationLevel
    pub fn set_isolation_level(&mut self, value: u32) {
        self.isolation_level = Some(value);
    }

    /// Gets the value of IsolationLevel
    pub fn get_isolation_level(&self) -> Option<&u32> {
        self.isolation_level.as_ref()
    }

    /// Sets the value of Parent
    pub fn set_parent(&mut self, value: String) {
        self.parent = Some(value);
    }

    /// Gets the value of Parent
    pub fn get_parent(&self) -> Option<&String> {
        self.parent.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: String) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&String> {
        self.state.as_ref()
    }

    /// Sets the value of TransactionId
    pub fn set_transaction_id(&mut self, value: String) {
        self.transaction_id = Some(value);
    }

    /// Gets the value of TransactionId
    pub fn get_transaction_id(&self) -> Option<&String> {
        self.transaction_id.as_ref()
    }
}

