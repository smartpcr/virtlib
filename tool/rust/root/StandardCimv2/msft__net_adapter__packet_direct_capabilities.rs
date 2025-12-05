// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapter_PacketDirectCapabilities struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapter_PacketDirectCapabilities {

/// 
    #[serde(rename = "DrainNotificationSupported")]
    pub drain_notification_supported: Option<bool>,

/// 
    #[serde(rename = "MaximumModerationInterval")]
    pub maximum_moderation_interval: Option<u32>,

/// 
    #[serde(rename = "MaximumNumberOfRxQueues")]
    pub maximum_number_of_rx_queues: Option<u32>,

/// 
    #[serde(rename = "MaximumNumberOfRxQueuesForDefaultVPort")]
    pub maximum_number_of_rx_queues_for_default_vport: Option<u32>,

/// 
    #[serde(rename = "MaximumNumberOfRxQueuesPerNonDefaultVPort")]
    pub maximum_number_of_rx_queues_per_non_default_vport: Option<u32>,

/// 
    #[serde(rename = "MaximumNumberOfTxQueues")]
    pub maximum_number_of_tx_queues: Option<u32>,

/// 
    #[serde(rename = "MaximumNumberOfTxQueuesForDefaultVPort")]
    pub maximum_number_of_tx_queues_for_default_vport: Option<u32>,

/// 
    #[serde(rename = "MaximumNumberOfTxQueuesPerNonDefaultVPort")]
    pub maximum_number_of_tx_queues_per_non_default_vport: Option<u32>,

/// 
    #[serde(rename = "MaximumRxPartialBufferCount")]
    pub maximum_rx_partial_buffer_count: Option<u32>,

/// 
    #[serde(rename = "MaximumRxQueueSize")]
    pub maximum_rx_queue_size: Option<u32>,

/// 
    #[serde(rename = "MaximumTxPartialBufferCount")]
    pub maximum_tx_partial_buffer_count: Option<u32>,

/// 
    #[serde(rename = "MaximumTxQueueSize")]
    pub maximum_tx_queue_size: Option<u32>,

/// 
    #[serde(rename = "MinimumModerationInterval")]
    pub minimum_moderation_interval: Option<u32>,

/// 
    #[serde(rename = "ModerationByCountSupported")]
    pub moderation_by_count_supported: Option<bool>,

/// 
    #[serde(rename = "ModerationByIntervalSupported")]
    pub moderation_by_interval_supported: Option<bool>,

/// 
    #[serde(rename = "ModerationIntervalGranularity")]
    pub moderation_interval_granularity: Option<u32>,
}

impl MSFT_NetAdapter_PacketDirectCapabilities {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            drain_notification_supported: None,
            maximum_moderation_interval: None,
            maximum_number_of_rx_queues: None,
            maximum_number_of_rx_queues_for_default_vport: None,
            maximum_number_of_rx_queues_per_non_default_vport: None,
            maximum_number_of_tx_queues: None,
            maximum_number_of_tx_queues_for_default_vport: None,
            maximum_number_of_tx_queues_per_non_default_vport: None,
            maximum_rx_partial_buffer_count: None,
            maximum_rx_queue_size: None,
            maximum_tx_partial_buffer_count: None,
            maximum_tx_queue_size: None,
            minimum_moderation_interval: None,
            moderation_by_count_supported: None,
            moderation_by_interval_supported: None,
            moderation_interval_granularity: None,
        }
    }


    /// Sets the value of DrainNotificationSupported
    pub fn set_drain_notification_supported(&mut self, value: bool) {
        self.drain_notification_supported = Some(value);
    }

    /// Gets the value of DrainNotificationSupported
    pub fn get_drain_notification_supported(&self) -> Option<&bool> {
        self.drain_notification_supported.as_ref()
    }

    /// Sets the value of MaximumModerationInterval
    pub fn set_maximum_moderation_interval(&mut self, value: u32) {
        self.maximum_moderation_interval = Some(value);
    }

    /// Gets the value of MaximumModerationInterval
    pub fn get_maximum_moderation_interval(&self) -> Option<&u32> {
        self.maximum_moderation_interval.as_ref()
    }

    /// Sets the value of MaximumNumberOfRxQueues
    pub fn set_maximum_number_of_rx_queues(&mut self, value: u32) {
        self.maximum_number_of_rx_queues = Some(value);
    }

    /// Gets the value of MaximumNumberOfRxQueues
    pub fn get_maximum_number_of_rx_queues(&self) -> Option<&u32> {
        self.maximum_number_of_rx_queues.as_ref()
    }

    /// Sets the value of MaximumNumberOfRxQueuesForDefaultVPort
    pub fn set_maximum_number_of_rx_queues_for_default_vport(&mut self, value: u32) {
        self.maximum_number_of_rx_queues_for_default_vport = Some(value);
    }

    /// Gets the value of MaximumNumberOfRxQueuesForDefaultVPort
    pub fn get_maximum_number_of_rx_queues_for_default_vport(&self) -> Option<&u32> {
        self.maximum_number_of_rx_queues_for_default_vport.as_ref()
    }

    /// Sets the value of MaximumNumberOfRxQueuesPerNonDefaultVPort
    pub fn set_maximum_number_of_rx_queues_per_non_default_vport(&mut self, value: u32) {
        self.maximum_number_of_rx_queues_per_non_default_vport = Some(value);
    }

    /// Gets the value of MaximumNumberOfRxQueuesPerNonDefaultVPort
    pub fn get_maximum_number_of_rx_queues_per_non_default_vport(&self) -> Option<&u32> {
        self.maximum_number_of_rx_queues_per_non_default_vport.as_ref()
    }

    /// Sets the value of MaximumNumberOfTxQueues
    pub fn set_maximum_number_of_tx_queues(&mut self, value: u32) {
        self.maximum_number_of_tx_queues = Some(value);
    }

    /// Gets the value of MaximumNumberOfTxQueues
    pub fn get_maximum_number_of_tx_queues(&self) -> Option<&u32> {
        self.maximum_number_of_tx_queues.as_ref()
    }

    /// Sets the value of MaximumNumberOfTxQueuesForDefaultVPort
    pub fn set_maximum_number_of_tx_queues_for_default_vport(&mut self, value: u32) {
        self.maximum_number_of_tx_queues_for_default_vport = Some(value);
    }

    /// Gets the value of MaximumNumberOfTxQueuesForDefaultVPort
    pub fn get_maximum_number_of_tx_queues_for_default_vport(&self) -> Option<&u32> {
        self.maximum_number_of_tx_queues_for_default_vport.as_ref()
    }

    /// Sets the value of MaximumNumberOfTxQueuesPerNonDefaultVPort
    pub fn set_maximum_number_of_tx_queues_per_non_default_vport(&mut self, value: u32) {
        self.maximum_number_of_tx_queues_per_non_default_vport = Some(value);
    }

    /// Gets the value of MaximumNumberOfTxQueuesPerNonDefaultVPort
    pub fn get_maximum_number_of_tx_queues_per_non_default_vport(&self) -> Option<&u32> {
        self.maximum_number_of_tx_queues_per_non_default_vport.as_ref()
    }

    /// Sets the value of MaximumRxPartialBufferCount
    pub fn set_maximum_rx_partial_buffer_count(&mut self, value: u32) {
        self.maximum_rx_partial_buffer_count = Some(value);
    }

    /// Gets the value of MaximumRxPartialBufferCount
    pub fn get_maximum_rx_partial_buffer_count(&self) -> Option<&u32> {
        self.maximum_rx_partial_buffer_count.as_ref()
    }

    /// Sets the value of MaximumRxQueueSize
    pub fn set_maximum_rx_queue_size(&mut self, value: u32) {
        self.maximum_rx_queue_size = Some(value);
    }

    /// Gets the value of MaximumRxQueueSize
    pub fn get_maximum_rx_queue_size(&self) -> Option<&u32> {
        self.maximum_rx_queue_size.as_ref()
    }

    /// Sets the value of MaximumTxPartialBufferCount
    pub fn set_maximum_tx_partial_buffer_count(&mut self, value: u32) {
        self.maximum_tx_partial_buffer_count = Some(value);
    }

    /// Gets the value of MaximumTxPartialBufferCount
    pub fn get_maximum_tx_partial_buffer_count(&self) -> Option<&u32> {
        self.maximum_tx_partial_buffer_count.as_ref()
    }

    /// Sets the value of MaximumTxQueueSize
    pub fn set_maximum_tx_queue_size(&mut self, value: u32) {
        self.maximum_tx_queue_size = Some(value);
    }

    /// Gets the value of MaximumTxQueueSize
    pub fn get_maximum_tx_queue_size(&self) -> Option<&u32> {
        self.maximum_tx_queue_size.as_ref()
    }

    /// Sets the value of MinimumModerationInterval
    pub fn set_minimum_moderation_interval(&mut self, value: u32) {
        self.minimum_moderation_interval = Some(value);
    }

    /// Gets the value of MinimumModerationInterval
    pub fn get_minimum_moderation_interval(&self) -> Option<&u32> {
        self.minimum_moderation_interval.as_ref()
    }

    /// Sets the value of ModerationByCountSupported
    pub fn set_moderation_by_count_supported(&mut self, value: bool) {
        self.moderation_by_count_supported = Some(value);
    }

    /// Gets the value of ModerationByCountSupported
    pub fn get_moderation_by_count_supported(&self) -> Option<&bool> {
        self.moderation_by_count_supported.as_ref()
    }

    /// Sets the value of ModerationByIntervalSupported
    pub fn set_moderation_by_interval_supported(&mut self, value: bool) {
        self.moderation_by_interval_supported = Some(value);
    }

    /// Gets the value of ModerationByIntervalSupported
    pub fn get_moderation_by_interval_supported(&self) -> Option<&bool> {
        self.moderation_by_interval_supported.as_ref()
    }

    /// Sets the value of ModerationIntervalGranularity
    pub fn set_moderation_interval_granularity(&mut self, value: u32) {
        self.moderation_interval_granularity = Some(value);
    }

    /// Gets the value of ModerationIntervalGranularity
    pub fn get_moderation_interval_granularity(&self) -> Option<&u32> {
        self.moderation_interval_granularity.as_ref()
    }
}

