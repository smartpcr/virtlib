// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MsmqTransportBindingElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MsmqTransportBindingElement {
    #[serde(flatten)]
    pub base: MsmqBindingElementBase,

/// An integer that specifies the maximum size of the pool that contains internal MSMQ message objects.
    #[serde(rename = "MaxPoolSize")]
    pub max_pool_size: Option<i32>,

/// An enumeration value that indicates the queued communication channel transport that this binding uses.
    #[serde(rename = "QueueTransferProtocol")]
    pub queue_transfer_protocol: Option<String>,

/// Returns a Boolean value that indicates whether queue addresses should be converted using Active Directory.
    #[serde(rename = "UseActiveDirectory")]
    pub use_active_directory: Option<bool>,
}

impl MsmqTransportBindingElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MsmqBindingElementBase::new(),
            max_pool_size: None,
            queue_transfer_protocol: None,
            use_active_directory: None,
        }
    }


    /// Sets the value of MaxPoolSize
    pub fn set_max_pool_size(&mut self, value: i32) {
        self.max_pool_size = Some(value);
    }

    /// Gets the value of MaxPoolSize
    pub fn get_max_pool_size(&self) -> Option<&i32> {
        self.max_pool_size.as_ref()
    }

    /// Sets the value of QueueTransferProtocol
    pub fn set_queue_transfer_protocol(&mut self, value: String) {
        self.queue_transfer_protocol = Some(value);
    }

    /// Gets the value of QueueTransferProtocol
    pub fn get_queue_transfer_protocol(&self) -> Option<&String> {
        self.queue_transfer_protocol.as_ref()
    }

    /// Sets the value of UseActiveDirectory
    pub fn set_use_active_directory(&mut self, value: bool) {
        self.use_active_directory = Some(value);
    }

    /// Gets the value of UseActiveDirectory
    pub fn get_use_active_directory(&self) -> Option<&bool> {
        self.use_active_directory.as_ref()
    }
}

