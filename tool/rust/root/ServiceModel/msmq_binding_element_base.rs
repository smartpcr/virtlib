// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MsmqBindingElementBase struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MsmqBindingElementBase {
    #[serde(flatten)]
    pub base: TransportBindingElement,

/// A URI that contains the location of the per-application dead letter queue, where messages that have expired or that have failed transfer or delivery are placed.
    #[serde(rename = "CustomDeadLetterQueue")]
    pub custom_dead_letter_queue: Option<String>,

/// An enumeration value that indicates the type of dead letter queue to use.
    #[serde(rename = "DeadLetterQueue")]
    pub dead_letter_queue: Option<String>,

/// A value that indicates whether the messages processed by this binding are durable or volatile.
    #[serde(rename = "Durable")]
    pub durable: Option<bool>,

/// A Boolean value that indicates whether messages processed by this binding will be received exactly once.
    #[serde(rename = "ExactlyOnce")]
    pub exactly_once: Option<bool>,

/// The maximum number of retry cycles to attempt delivery of messages to the receiving application.
    #[serde(rename = "MaxRetryCycles")]
    pub max_retry_cycles: Option<i32>,

/// A boolean value that indicates whether messages received by this binding should be locked when received.
    #[serde(rename = "ReceiveContextEnabled")]
    pub receive_context_enabled: Option<bool>,

/// The settings for poison message handling.
    #[serde(rename = "ReceiveErrorHandling")]
    pub receive_error_handling: Option<String>,

/// The maximum number of immediate retry attempts on a message that is read from the application queue.
    #[serde(rename = "ReceiveRetryCount")]
    pub receive_retry_count: Option<i32>,

/// A value that indicates the time delay between retry cycles when attempting to deliver a message that could not be delivered immediately.
    #[serde(rename = "RetryCycleDelay")]
    pub retry_cycle_delay: Option<String>,

/// The interval of time that indicates how long the messages processed by this binding can be in the queue before they expire.
    #[serde(rename = "TimeToLive")]
    pub time_to_live: Option<String>,

/// A Boolean value that indicates whether messages processed by this binding should be traced.
    #[serde(rename = "UseMsmqTracing")]
    pub use_msmq_tracing: Option<bool>,

/// A Boolean value that indicates whether copies of messages processed by this binding should be stored in the source journal queue.
    #[serde(rename = "UseSourceJournal")]
    pub use_source_journal: Option<bool>,

/// Gets or sets the interval of time before a message locked by ReceiveContext is unlocked and returned to the Queue.
    #[serde(rename = "ValidityDuration")]
    pub validity_duration: Option<String>,
}

impl MsmqBindingElementBase {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: TransportBindingElement::new(),
            custom_dead_letter_queue: None,
            dead_letter_queue: None,
            durable: None,
            exactly_once: None,
            max_retry_cycles: None,
            receive_context_enabled: None,
            receive_error_handling: None,
            receive_retry_count: None,
            retry_cycle_delay: None,
            time_to_live: None,
            use_msmq_tracing: None,
            use_source_journal: None,
            validity_duration: None,
        }
    }


    /// Sets the value of CustomDeadLetterQueue
    pub fn set_custom_dead_letter_queue(&mut self, value: String) {
        self.custom_dead_letter_queue = Some(value);
    }

    /// Gets the value of CustomDeadLetterQueue
    pub fn get_custom_dead_letter_queue(&self) -> Option<&String> {
        self.custom_dead_letter_queue.as_ref()
    }

    /// Sets the value of DeadLetterQueue
    pub fn set_dead_letter_queue(&mut self, value: String) {
        self.dead_letter_queue = Some(value);
    }

    /// Gets the value of DeadLetterQueue
    pub fn get_dead_letter_queue(&self) -> Option<&String> {
        self.dead_letter_queue.as_ref()
    }

    /// Sets the value of Durable
    pub fn set_durable(&mut self, value: bool) {
        self.durable = Some(value);
    }

    /// Gets the value of Durable
    pub fn get_durable(&self) -> Option<&bool> {
        self.durable.as_ref()
    }

    /// Sets the value of ExactlyOnce
    pub fn set_exactly_once(&mut self, value: bool) {
        self.exactly_once = Some(value);
    }

    /// Gets the value of ExactlyOnce
    pub fn get_exactly_once(&self) -> Option<&bool> {
        self.exactly_once.as_ref()
    }

    /// Sets the value of MaxRetryCycles
    pub fn set_max_retry_cycles(&mut self, value: i32) {
        self.max_retry_cycles = Some(value);
    }

    /// Gets the value of MaxRetryCycles
    pub fn get_max_retry_cycles(&self) -> Option<&i32> {
        self.max_retry_cycles.as_ref()
    }

    /// Sets the value of ReceiveContextEnabled
    pub fn set_receive_context_enabled(&mut self, value: bool) {
        self.receive_context_enabled = Some(value);
    }

    /// Gets the value of ReceiveContextEnabled
    pub fn get_receive_context_enabled(&self) -> Option<&bool> {
        self.receive_context_enabled.as_ref()
    }

    /// Sets the value of ReceiveErrorHandling
    pub fn set_receive_error_handling(&mut self, value: String) {
        self.receive_error_handling = Some(value);
    }

    /// Gets the value of ReceiveErrorHandling
    pub fn get_receive_error_handling(&self) -> Option<&String> {
        self.receive_error_handling.as_ref()
    }

    /// Sets the value of ReceiveRetryCount
    pub fn set_receive_retry_count(&mut self, value: i32) {
        self.receive_retry_count = Some(value);
    }

    /// Gets the value of ReceiveRetryCount
    pub fn get_receive_retry_count(&self) -> Option<&i32> {
        self.receive_retry_count.as_ref()
    }

    /// Sets the value of RetryCycleDelay
    pub fn set_retry_cycle_delay(&mut self, value: String) {
        self.retry_cycle_delay = Some(value);
    }

    /// Gets the value of RetryCycleDelay
    pub fn get_retry_cycle_delay(&self) -> Option<&String> {
        self.retry_cycle_delay.as_ref()
    }

    /// Sets the value of TimeToLive
    pub fn set_time_to_live(&mut self, value: String) {
        self.time_to_live = Some(value);
    }

    /// Gets the value of TimeToLive
    pub fn get_time_to_live(&self) -> Option<&String> {
        self.time_to_live.as_ref()
    }

    /// Sets the value of UseMsmqTracing
    pub fn set_use_msmq_tracing(&mut self, value: bool) {
        self.use_msmq_tracing = Some(value);
    }

    /// Gets the value of UseMsmqTracing
    pub fn get_use_msmq_tracing(&self) -> Option<&bool> {
        self.use_msmq_tracing.as_ref()
    }

    /// Sets the value of UseSourceJournal
    pub fn set_use_source_journal(&mut self, value: bool) {
        self.use_source_journal = Some(value);
    }

    /// Gets the value of UseSourceJournal
    pub fn get_use_source_journal(&self) -> Option<&bool> {
        self.use_source_journal.as_ref()
    }

    /// Sets the value of ValidityDuration
    pub fn set_validity_duration(&mut self, value: String) {
        self.validity_duration = Some(value);
    }

    /// Gets the value of ValidityDuration
    pub fn get_validity_duration(&self) -> Option<&String> {
        self.validity_duration.as_ref()
    }
}

