// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.SDNDiagnostics.Server
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// __MethodInvocationEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct __MethodInvocationEvent {
    #[serde(flatten)]
    pub base: __InstanceOperationEvent,

/// 
    #[serde(rename = "Method")]
    pub method: Option<String>,

/// 
    #[serde(rename = "Parameters")]
    pub parameters: Option<serde_json::Value>,

/// 
    #[serde(rename = "PreCall")]
    pub pre_call: Option<bool>,
}

impl __MethodInvocationEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: __InstanceOperationEvent::new(),
            method: None,
            parameters: None,
            pre_call: None,
        }
    }


    /// Sets the value of Method
    pub fn set_method(&mut self, value: String) {
        self.method = Some(value);
    }

    /// Gets the value of Method
    pub fn get_method(&self) -> Option<&String> {
        self.method.as_ref()
    }

    /// Sets the value of Parameters
    pub fn set_parameters(&mut self, value: serde_json::Value) {
        self.parameters = Some(value);
    }

    /// Gets the value of Parameters
    pub fn get_parameters(&self) -> Option<&serde_json::Value> {
        self.parameters.as_ref()
    }

    /// Sets the value of PreCall
    pub fn set_pre_call(&mut self, value: bool) {
        self.pre_call = Some(value);
    }

    /// Gets the value of PreCall
    pub fn get_pre_call(&self) -> Option<&bool> {
        self.pre_call.as_ref()
    }
}

