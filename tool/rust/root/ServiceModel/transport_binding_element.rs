// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// TransportBindingElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TransportBindingElement {
    #[serde(flatten)]
    pub base: BindingElement,

/// A Boolean value that specifies if the user wants to take control of message addressing. 
    #[serde(rename = "ManualAddressing")]
    pub manual_addressing: Option<bool>,

/// The maximum buffer pool size for the binding.
    #[serde(rename = "MaxBufferPoolSize")]
    pub max_buffer_pool_size: Option<i64>,

/// The maximum size for a message that is processed by this binding.
    #[serde(rename = "MaxReceivedMessageSize")]
    pub max_received_message_size: Option<i64>,

/// The URI scheme for the transport.
    #[serde(rename = "Scheme")]
    pub scheme: Option<String>,
}

impl TransportBindingElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: BindingElement::new(),
            manual_addressing: None,
            max_buffer_pool_size: None,
            max_received_message_size: None,
            scheme: None,
        }
    }


    /// Sets the value of ManualAddressing
    pub fn set_manual_addressing(&mut self, value: bool) {
        self.manual_addressing = Some(value);
    }

    /// Gets the value of ManualAddressing
    pub fn get_manual_addressing(&self) -> Option<&bool> {
        self.manual_addressing.as_ref()
    }

    /// Sets the value of MaxBufferPoolSize
    pub fn set_max_buffer_pool_size(&mut self, value: i64) {
        self.max_buffer_pool_size = Some(value);
    }

    /// Gets the value of MaxBufferPoolSize
    pub fn get_max_buffer_pool_size(&self) -> Option<&i64> {
        self.max_buffer_pool_size.as_ref()
    }

    /// Sets the value of MaxReceivedMessageSize
    pub fn set_max_received_message_size(&mut self, value: i64) {
        self.max_received_message_size = Some(value);
    }

    /// Gets the value of MaxReceivedMessageSize
    pub fn get_max_received_message_size(&self) -> Option<&i64> {
        self.max_received_message_size.as_ref()
    }

    /// Sets the value of Scheme
    pub fn set_scheme(&mut self, value: String) {
        self.scheme = Some(value);
    }

    /// Gets the value of Scheme
    pub fn get_scheme(&self) -> Option<&String> {
        self.scheme.as_ref()
    }
}

