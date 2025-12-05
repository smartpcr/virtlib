// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// OperationBehaviorAttribute struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OperationBehaviorAttribute {
    #[serde(flatten)]
    pub base: Behavior,

/// The state of the auto dispose feature for parameters. 
    #[serde(rename = "AutoDisposeParameters")]
    pub auto_dispose_parameters: Option<bool>,

/// Indicates the level of caller impersonation that the operation supports.
    #[serde(rename = "Impersonation")]
    pub impersonation: Option<String>,

/// Indicates when in the course of an operation invocation to recycle the object.
    #[serde(rename = "ReleaseInstanceMode")]
    pub release_instance_mode: Option<String>,

/// Indicates whether to automatically commit the current transaction if no unhandled exceptions occur.
    #[serde(rename = "TransactionAutoComplete")]
    pub transaction_auto_complete: Option<bool>,

/// Indicates whether the operation requires a transaction.
    #[serde(rename = "TransactionScopeRequired")]
    pub transaction_scope_required: Option<bool>,
}

impl OperationBehaviorAttribute {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Behavior::new(),
            auto_dispose_parameters: None,
            impersonation: None,
            release_instance_mode: None,
            transaction_auto_complete: None,
            transaction_scope_required: None,
        }
    }


    /// Sets the value of AutoDisposeParameters
    pub fn set_auto_dispose_parameters(&mut self, value: bool) {
        self.auto_dispose_parameters = Some(value);
    }

    /// Gets the value of AutoDisposeParameters
    pub fn get_auto_dispose_parameters(&self) -> Option<&bool> {
        self.auto_dispose_parameters.as_ref()
    }

    /// Sets the value of Impersonation
    pub fn set_impersonation(&mut self, value: String) {
        self.impersonation = Some(value);
    }

    /// Gets the value of Impersonation
    pub fn get_impersonation(&self) -> Option<&String> {
        self.impersonation.as_ref()
    }

    /// Sets the value of ReleaseInstanceMode
    pub fn set_release_instance_mode(&mut self, value: String) {
        self.release_instance_mode = Some(value);
    }

    /// Gets the value of ReleaseInstanceMode
    pub fn get_release_instance_mode(&self) -> Option<&String> {
        self.release_instance_mode.as_ref()
    }

    /// Sets the value of TransactionAutoComplete
    pub fn set_transaction_auto_complete(&mut self, value: bool) {
        self.transaction_auto_complete = Some(value);
    }

    /// Gets the value of TransactionAutoComplete
    pub fn get_transaction_auto_complete(&self) -> Option<&bool> {
        self.transaction_auto_complete.as_ref()
    }

    /// Sets the value of TransactionScopeRequired
    pub fn set_transaction_scope_required(&mut self, value: bool) {
        self.transaction_scope_required = Some(value);
    }

    /// Gets the value of TransactionScopeRequired
    pub fn get_transaction_scope_required(&self) -> Option<&bool> {
        self.transaction_scope_required.as_ref()
    }
}

