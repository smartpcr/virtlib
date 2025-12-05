// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ConnectionOrientedTransportBindingElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConnectionOrientedTransportBindingElement {
    #[serde(flatten)]
    pub base: TransportBindingElement,

/// The Timespan that specifies how long the channel initialization has to complete before timing out.
    #[serde(rename = "ChannelInitializationTimeout")]
    pub channel_initialization_timeout: Option<String>,

/// The size of the buffer used to transmit a chunk of the serialized message on the wire from the client or service. 
    #[serde(rename = "ConnectionBufferSize")]
    pub connection_buffer_size: Option<i32>,

/// A value that indicates whether the hostname is used to reach the service when matching on the URI.
    #[serde(rename = "HostNameComparisonMode")]
    pub host_name_comparison_mode: Option<String>,

/// The maximum size of the buffer to use.
    #[serde(rename = "MaxBufferSize")]
    pub max_buffer_size: Option<i32>,

/// The maximum interval of time that a chunk of a message or a full message can remain buffered in memory before being sent out.
    #[serde(rename = "MaxOutputDelay")]
    pub max_output_delay: Option<String>,

/// The maximum number of pending asynchronous accept threads that are available for processing incoming connections on the service.
    #[serde(rename = "MaxPendingAccepts")]
    pub max_pending_accepts: Option<i32>,

/// The maximum number of pending connections.
    #[serde(rename = "MaxPendingConnections")]
    pub max_pending_connections: Option<i32>,

/// A value that specifies whether the messages are buffered or streamed with the connection-oriented transport.
    #[serde(rename = "TransferMode")]
    pub transfer_mode: Option<String>,
}

impl ConnectionOrientedTransportBindingElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: TransportBindingElement::new(),
            channel_initialization_timeout: None,
            connection_buffer_size: None,
            host_name_comparison_mode: None,
            max_buffer_size: None,
            max_output_delay: None,
            max_pending_accepts: None,
            max_pending_connections: None,
            transfer_mode: None,
        }
    }


    /// Sets the value of ChannelInitializationTimeout
    pub fn set_channel_initialization_timeout(&mut self, value: String) {
        self.channel_initialization_timeout = Some(value);
    }

    /// Gets the value of ChannelInitializationTimeout
    pub fn get_channel_initialization_timeout(&self) -> Option<&String> {
        self.channel_initialization_timeout.as_ref()
    }

    /// Sets the value of ConnectionBufferSize
    pub fn set_connection_buffer_size(&mut self, value: i32) {
        self.connection_buffer_size = Some(value);
    }

    /// Gets the value of ConnectionBufferSize
    pub fn get_connection_buffer_size(&self) -> Option<&i32> {
        self.connection_buffer_size.as_ref()
    }

    /// Sets the value of HostNameComparisonMode
    pub fn set_host_name_comparison_mode(&mut self, value: String) {
        self.host_name_comparison_mode = Some(value);
    }

    /// Gets the value of HostNameComparisonMode
    pub fn get_host_name_comparison_mode(&self) -> Option<&String> {
        self.host_name_comparison_mode.as_ref()
    }

    /// Sets the value of MaxBufferSize
    pub fn set_max_buffer_size(&mut self, value: i32) {
        self.max_buffer_size = Some(value);
    }

    /// Gets the value of MaxBufferSize
    pub fn get_max_buffer_size(&self) -> Option<&i32> {
        self.max_buffer_size.as_ref()
    }

    /// Sets the value of MaxOutputDelay
    pub fn set_max_output_delay(&mut self, value: String) {
        self.max_output_delay = Some(value);
    }

    /// Gets the value of MaxOutputDelay
    pub fn get_max_output_delay(&self) -> Option<&String> {
        self.max_output_delay.as_ref()
    }

    /// Sets the value of MaxPendingAccepts
    pub fn set_max_pending_accepts(&mut self, value: i32) {
        self.max_pending_accepts = Some(value);
    }

    /// Gets the value of MaxPendingAccepts
    pub fn get_max_pending_accepts(&self) -> Option<&i32> {
        self.max_pending_accepts.as_ref()
    }

    /// Sets the value of MaxPendingConnections
    pub fn set_max_pending_connections(&mut self, value: i32) {
        self.max_pending_connections = Some(value);
    }

    /// Gets the value of MaxPendingConnections
    pub fn get_max_pending_connections(&self) -> Option<&i32> {
        self.max_pending_connections.as_ref()
    }

    /// Sets the value of TransferMode
    pub fn set_transfer_mode(&mut self, value: String) {
        self.transfer_mode = Some(value);
    }

    /// Gets the value of TransferMode
    pub fn get_transfer_mode(&self) -> Option<&String> {
        self.transfer_mode.as_ref()
    }
}

