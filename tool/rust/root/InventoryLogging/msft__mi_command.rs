// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.InventoryLogging
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msft_MiCommand struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msft_MiCommand {
    #[serde(flatten)]
    pub base: Msft_MiStream,

/// 
    #[serde(rename = "ClassName")]
    pub class_name: Option<String>,

/// 
    #[serde(rename = "Input")]
    pub input: Option<serde_json::Value>,

/// 
    #[serde(rename = "MethodName")]
    pub method_name: Option<String>,

/// 
    #[serde(rename = "NamespaceName")]
    pub namespace_name: Option<String>,

/// 
    #[serde(rename = "Parameters")]
    pub parameters: Option<serde_json::Value>,
}

impl Msft_MiCommand {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msft_MiStream::new(),
            class_name: None,
            input: None,
            method_name: None,
            namespace_name: None,
            parameters: None,
        }
    }


    /// Sets the value of ClassName
    pub fn set_class_name(&mut self, value: String) {
        self.class_name = Some(value);
    }

    /// Gets the value of ClassName
    pub fn get_class_name(&self) -> Option<&String> {
        self.class_name.as_ref()
    }

    /// Sets the value of Input
    pub fn set_input(&mut self, value: serde_json::Value) {
        self.input = Some(value);
    }

    /// Gets the value of Input
    pub fn get_input(&self) -> Option<&serde_json::Value> {
        self.input.as_ref()
    }

    /// Sets the value of MethodName
    pub fn set_method_name(&mut self, value: String) {
        self.method_name = Some(value);
    }

    /// Gets the value of MethodName
    pub fn get_method_name(&self) -> Option<&String> {
        self.method_name.as_ref()
    }

    /// Sets the value of NamespaceName
    pub fn set_namespace_name(&mut self, value: String) {
        self.namespace_name = Some(value);
    }

    /// Gets the value of NamespaceName
    pub fn get_namespace_name(&self) -> Option<&String> {
        self.namespace_name.as_ref()
    }

    /// Sets the value of Parameters
    pub fn set_parameters(&mut self, value: serde_json::Value) {
        self.parameters = Some(value);
    }

    /// Gets the value of Parameters
    pub fn get_parameters(&self) -> Option<&serde_json::Value> {
        self.parameters.as_ref()
    }
}

