// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Binding struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Binding {

/// The collection of binding elements implemented by the binding.
    #[serde(rename = "BindingElements")]
    pub binding_elements: Vec<BindingElement>,

/// The interval of time provided for a close operation to complete. 
    #[serde(rename = "CloseTimeout")]
    pub close_timeout: Option<String>,

/// The name of the binding.
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// The XML namespace of the binding.
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,

/// The interval of time provided for an open operation to complete.
    #[serde(rename = "OpenTimeout")]
    pub open_timeout: Option<String>,

/// The interval of time provided for a receive operation to complete.
    #[serde(rename = "ReceiveTimeout")]
    pub receive_timeout: Option<String>,

/// The URI transport scheme that is used by the channel and listener factories that are built by the binding.
    #[serde(rename = "Scheme")]
    pub scheme: Option<String>,

/// The interval of time provided for a send operation to complete.
    #[serde(rename = "SendTimeout")]
    pub send_timeout: Option<String>,
}

impl Binding {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            binding_elements: Vec::new(),
            close_timeout: None,
            name: None,
            namespace: None,
            open_timeout: None,
            receive_timeout: None,
            scheme: None,
            send_timeout: None,
        }
    }


    /// Sets the value of BindingElements
    pub fn set_binding_elements(&mut self, value: Vec<BindingElement>) {
        self.binding_elements = value;
    }

    /// Gets the value of BindingElements
    pub fn get_binding_elements(&self) -> &Vec<BindingElement> {
        &self.binding_elements
    }

    /// Sets the value of CloseTimeout
    pub fn set_close_timeout(&mut self, value: String) {
        self.close_timeout = Some(value);
    }

    /// Gets the value of CloseTimeout
    pub fn get_close_timeout(&self) -> Option<&String> {
        self.close_timeout.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Namespace
    pub fn set_namespace(&mut self, value: String) {
        self.namespace = Some(value);
    }

    /// Gets the value of Namespace
    pub fn get_namespace(&self) -> Option<&String> {
        self.namespace.as_ref()
    }

    /// Sets the value of OpenTimeout
    pub fn set_open_timeout(&mut self, value: String) {
        self.open_timeout = Some(value);
    }

    /// Gets the value of OpenTimeout
    pub fn get_open_timeout(&self) -> Option<&String> {
        self.open_timeout.as_ref()
    }

    /// Sets the value of ReceiveTimeout
    pub fn set_receive_timeout(&mut self, value: String) {
        self.receive_timeout = Some(value);
    }

    /// Gets the value of ReceiveTimeout
    pub fn get_receive_timeout(&self) -> Option<&String> {
        self.receive_timeout.as_ref()
    }

    /// Sets the value of Scheme
    pub fn set_scheme(&mut self, value: String) {
        self.scheme = Some(value);
    }

    /// Gets the value of Scheme
    pub fn get_scheme(&self) -> Option<&String> {
        self.scheme.as_ref()
    }

    /// Sets the value of SendTimeout
    pub fn set_send_timeout(&mut self, value: String) {
        self.send_timeout = Some(value);
    }

    /// Gets the value of SendTimeout
    pub fn get_send_timeout(&self) -> Option<&String> {
        self.send_timeout.as_ref()
    }
}

