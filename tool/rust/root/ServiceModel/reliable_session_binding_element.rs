// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ReliableSessionBindingElement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReliableSessionBindingElement {
    #[serde(flatten)]
    pub base: BindingElement,

/// The interval of time that a destination waits before sending an acknowledgement to the message source on reliable channels that are created by the factory.
    #[serde(rename = "AcknowledgementInterval")]
    pub acknowledgement_interval: Option<String>,

/// A boolean value that specifies if flow control is enabled.
    #[serde(rename = "FlowControlEnabled")]
    pub flow_control_enabled: Option<bool>,

/// Specifies the maximum duration the channel is going to allow the other communicating party not to send any messages before faulting the channel.
    #[serde(rename = "InactivityTimeout")]
    pub inactivity_timeout: Option<String>,

/// The maximum number of channels that can wait to be accepted on the listener.
    #[serde(rename = "MaxPendingChannels")]
    pub max_pending_channels: Option<i32>,

/// The maximum number of times a reliable channel attempts to retransmit a message it has not received an acknowledgement for, by calling Send on its underlying channel.
    #[serde(rename = "MaxRetryCount")]
    pub max_retry_count: Option<i32>,

/// The maximum transfer window size for the reliable session.
    #[serde(rename = "MaxTransferWindowSize")]
    pub max_transfer_window_size: Option<i32>,

/// A Boolean value that specifies whether messages are guaranteed to arrive in the order they were sent.
    #[serde(rename = "Ordered")]
    pub ordered: Option<bool>,

/// The WS-ReliableMessaging protocol version used in the reliable session.
    #[serde(rename = "ReliableMessagingVersion")]
    pub reliable_messaging_version: Option<String>,
}

impl ReliableSessionBindingElement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: BindingElement::new(),
            acknowledgement_interval: None,
            flow_control_enabled: None,
            inactivity_timeout: None,
            max_pending_channels: None,
            max_retry_count: None,
            max_transfer_window_size: None,
            ordered: None,
            reliable_messaging_version: None,
        }
    }


    /// Sets the value of AcknowledgementInterval
    pub fn set_acknowledgement_interval(&mut self, value: String) {
        self.acknowledgement_interval = Some(value);
    }

    /// Gets the value of AcknowledgementInterval
    pub fn get_acknowledgement_interval(&self) -> Option<&String> {
        self.acknowledgement_interval.as_ref()
    }

    /// Sets the value of FlowControlEnabled
    pub fn set_flow_control_enabled(&mut self, value: bool) {
        self.flow_control_enabled = Some(value);
    }

    /// Gets the value of FlowControlEnabled
    pub fn get_flow_control_enabled(&self) -> Option<&bool> {
        self.flow_control_enabled.as_ref()
    }

    /// Sets the value of InactivityTimeout
    pub fn set_inactivity_timeout(&mut self, value: String) {
        self.inactivity_timeout = Some(value);
    }

    /// Gets the value of InactivityTimeout
    pub fn get_inactivity_timeout(&self) -> Option<&String> {
        self.inactivity_timeout.as_ref()
    }

    /// Sets the value of MaxPendingChannels
    pub fn set_max_pending_channels(&mut self, value: i32) {
        self.max_pending_channels = Some(value);
    }

    /// Gets the value of MaxPendingChannels
    pub fn get_max_pending_channels(&self) -> Option<&i32> {
        self.max_pending_channels.as_ref()
    }

    /// Sets the value of MaxRetryCount
    pub fn set_max_retry_count(&mut self, value: i32) {
        self.max_retry_count = Some(value);
    }

    /// Gets the value of MaxRetryCount
    pub fn get_max_retry_count(&self) -> Option<&i32> {
        self.max_retry_count.as_ref()
    }

    /// Sets the value of MaxTransferWindowSize
    pub fn set_max_transfer_window_size(&mut self, value: i32) {
        self.max_transfer_window_size = Some(value);
    }

    /// Gets the value of MaxTransferWindowSize
    pub fn get_max_transfer_window_size(&self) -> Option<&i32> {
        self.max_transfer_window_size.as_ref()
    }

    /// Sets the value of Ordered
    pub fn set_ordered(&mut self, value: bool) {
        self.ordered = Some(value);
    }

    /// Gets the value of Ordered
    pub fn get_ordered(&self) -> Option<&bool> {
        self.ordered.as_ref()
    }

    /// Sets the value of ReliableMessagingVersion
    pub fn set_reliable_messaging_version(&mut self, value: String) {
        self.reliable_messaging_version = Some(value);
    }

    /// Gets the value of ReliableMessagingVersion
    pub fn get_reliable_messaging_version(&self) -> Option<&String> {
        self.reliable_messaging_version.as_ref()
    }
}

