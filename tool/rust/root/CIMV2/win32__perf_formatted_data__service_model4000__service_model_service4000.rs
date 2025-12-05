// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_ServiceModel4000_ServiceModelService4000 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_ServiceModel4000_ServiceModelService4000 {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "Calls")]
    pub calls: Option<u32>,

/// 
    #[serde(rename = "CallsDuration")]
    pub calls_duration: Option<u32>,

/// 
    #[serde(rename = "CallsFailed")]
    pub calls_failed: Option<u32>,

/// 
    #[serde(rename = "CallsFailedPerSecond")]
    pub calls_failed_per_second: Option<u32>,

/// 
    #[serde(rename = "CallsFaulted")]
    pub calls_faulted: Option<u32>,

/// 
    #[serde(rename = "CallsFaultedPerSecond")]
    pub calls_faulted_per_second: Option<u32>,

/// 
    #[serde(rename = "CallsOutstanding")]
    pub calls_outstanding: Option<u32>,

/// 
    #[serde(rename = "CallsPerSecond")]
    pub calls_per_second: Option<u32>,

/// 
    #[serde(rename = "Instances")]
    pub instances: Option<u32>,

/// 
    #[serde(rename = "InstancesCreatedPerSecond")]
    pub instances_created_per_second: Option<u32>,

/// 
    #[serde(rename = "PercentOfMaxConcurrentCalls")]
    pub percent_of_max_concurrent_calls: Option<u32>,

/// 
    #[serde(rename = "PercentOfMaxConcurrentInstances")]
    pub percent_of_max_concurrent_instances: Option<u32>,

/// 
    #[serde(rename = "PercentOfMaxConcurrentSessions")]
    pub percent_of_max_concurrent_sessions: Option<u32>,

/// 
    #[serde(rename = "QueuedMessagesDropped")]
    pub queued_messages_dropped: Option<u32>,

/// 
    #[serde(rename = "QueuedMessagesDroppedPerSecond")]
    pub queued_messages_dropped_per_second: Option<u32>,

/// 
    #[serde(rename = "QueuedMessagesRejected")]
    pub queued_messages_rejected: Option<u32>,

/// 
    #[serde(rename = "QueuedMessagesRejectedPerSecond")]
    pub queued_messages_rejected_per_second: Option<u32>,

/// 
    #[serde(rename = "QueuedPoisonMessages")]
    pub queued_poison_messages: Option<u32>,

/// 
    #[serde(rename = "QueuedPoisonMessagesPerSecond")]
    pub queued_poison_messages_per_second: Option<u32>,

/// 
    #[serde(rename = "ReliableMessagingMessagesDropped")]
    pub reliable_messaging_messages_dropped: Option<u32>,

/// 
    #[serde(rename = "ReliableMessagingMessagesDroppedPerSecond")]
    pub reliable_messaging_messages_dropped_per_second: Option<u32>,

/// 
    #[serde(rename = "ReliableMessagingSessionsFaulted")]
    pub reliable_messaging_sessions_faulted: Option<u32>,

/// 
    #[serde(rename = "ReliableMessagingSessionsFaultedPerSecond")]
    pub reliable_messaging_sessions_faulted_per_second: Option<u32>,

/// 
    #[serde(rename = "SecurityCallsNotAuthorized")]
    pub security_calls_not_authorized: Option<u32>,

/// 
    #[serde(rename = "SecurityCallsNotAuthorizedPerSecond")]
    pub security_calls_not_authorized_per_second: Option<u32>,

/// 
    #[serde(rename = "SecurityValidationandAuthenticationFailures")]
    pub security_validationand_authentication_failures: Option<u32>,

/// 
    #[serde(rename = "SecurityValidationandAuthenticationFailuresPerSecond")]
    pub security_validationand_authentication_failures_per_second: Option<u32>,

/// 
    #[serde(rename = "TransactedOperationsAborted")]
    pub transacted_operations_aborted: Option<u32>,

/// 
    #[serde(rename = "TransactedOperationsAbortedPerSecond")]
    pub transacted_operations_aborted_per_second: Option<u32>,

/// 
    #[serde(rename = "TransactedOperationsCommitted")]
    pub transacted_operations_committed: Option<u32>,

/// 
    #[serde(rename = "TransactedOperationsCommittedPerSecond")]
    pub transacted_operations_committed_per_second: Option<u32>,

/// 
    #[serde(rename = "TransactedOperationsInDoubt")]
    pub transacted_operations_in_doubt: Option<u32>,

/// 
    #[serde(rename = "TransactedOperationsInDoubtPerSecond")]
    pub transacted_operations_in_doubt_per_second: Option<u32>,

/// 
    #[serde(rename = "TransactionsFlowed")]
    pub transactions_flowed: Option<u32>,

/// 
    #[serde(rename = "TransactionsFlowedPerSecond")]
    pub transactions_flowed_per_second: Option<u32>,
}

impl Win32_PerfFormattedData_ServiceModel4000_ServiceModelService4000 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            calls: None,
            calls_duration: None,
            calls_failed: None,
            calls_failed_per_second: None,
            calls_faulted: None,
            calls_faulted_per_second: None,
            calls_outstanding: None,
            calls_per_second: None,
            instances: None,
            instances_created_per_second: None,
            percent_of_max_concurrent_calls: None,
            percent_of_max_concurrent_instances: None,
            percent_of_max_concurrent_sessions: None,
            queued_messages_dropped: None,
            queued_messages_dropped_per_second: None,
            queued_messages_rejected: None,
            queued_messages_rejected_per_second: None,
            queued_poison_messages: None,
            queued_poison_messages_per_second: None,
            reliable_messaging_messages_dropped: None,
            reliable_messaging_messages_dropped_per_second: None,
            reliable_messaging_sessions_faulted: None,
            reliable_messaging_sessions_faulted_per_second: None,
            security_calls_not_authorized: None,
            security_calls_not_authorized_per_second: None,
            security_validationand_authentication_failures: None,
            security_validationand_authentication_failures_per_second: None,
            transacted_operations_aborted: None,
            transacted_operations_aborted_per_second: None,
            transacted_operations_committed: None,
            transacted_operations_committed_per_second: None,
            transacted_operations_in_doubt: None,
            transacted_operations_in_doubt_per_second: None,
            transactions_flowed: None,
            transactions_flowed_per_second: None,
        }
    }


    /// Sets the value of Calls
    pub fn set_calls(&mut self, value: u32) {
        self.calls = Some(value);
    }

    /// Gets the value of Calls
    pub fn get_calls(&self) -> Option<&u32> {
        self.calls.as_ref()
    }

    /// Sets the value of CallsDuration
    pub fn set_calls_duration(&mut self, value: u32) {
        self.calls_duration = Some(value);
    }

    /// Gets the value of CallsDuration
    pub fn get_calls_duration(&self) -> Option<&u32> {
        self.calls_duration.as_ref()
    }

    /// Sets the value of CallsFailed
    pub fn set_calls_failed(&mut self, value: u32) {
        self.calls_failed = Some(value);
    }

    /// Gets the value of CallsFailed
    pub fn get_calls_failed(&self) -> Option<&u32> {
        self.calls_failed.as_ref()
    }

    /// Sets the value of CallsFailedPerSecond
    pub fn set_calls_failed_per_second(&mut self, value: u32) {
        self.calls_failed_per_second = Some(value);
    }

    /// Gets the value of CallsFailedPerSecond
    pub fn get_calls_failed_per_second(&self) -> Option<&u32> {
        self.calls_failed_per_second.as_ref()
    }

    /// Sets the value of CallsFaulted
    pub fn set_calls_faulted(&mut self, value: u32) {
        self.calls_faulted = Some(value);
    }

    /// Gets the value of CallsFaulted
    pub fn get_calls_faulted(&self) -> Option<&u32> {
        self.calls_faulted.as_ref()
    }

    /// Sets the value of CallsFaultedPerSecond
    pub fn set_calls_faulted_per_second(&mut self, value: u32) {
        self.calls_faulted_per_second = Some(value);
    }

    /// Gets the value of CallsFaultedPerSecond
    pub fn get_calls_faulted_per_second(&self) -> Option<&u32> {
        self.calls_faulted_per_second.as_ref()
    }

    /// Sets the value of CallsOutstanding
    pub fn set_calls_outstanding(&mut self, value: u32) {
        self.calls_outstanding = Some(value);
    }

    /// Gets the value of CallsOutstanding
    pub fn get_calls_outstanding(&self) -> Option<&u32> {
        self.calls_outstanding.as_ref()
    }

    /// Sets the value of CallsPerSecond
    pub fn set_calls_per_second(&mut self, value: u32) {
        self.calls_per_second = Some(value);
    }

    /// Gets the value of CallsPerSecond
    pub fn get_calls_per_second(&self) -> Option<&u32> {
        self.calls_per_second.as_ref()
    }

    /// Sets the value of Instances
    pub fn set_instances(&mut self, value: u32) {
        self.instances = Some(value);
    }

    /// Gets the value of Instances
    pub fn get_instances(&self) -> Option<&u32> {
        self.instances.as_ref()
    }

    /// Sets the value of InstancesCreatedPerSecond
    pub fn set_instances_created_per_second(&mut self, value: u32) {
        self.instances_created_per_second = Some(value);
    }

    /// Gets the value of InstancesCreatedPerSecond
    pub fn get_instances_created_per_second(&self) -> Option<&u32> {
        self.instances_created_per_second.as_ref()
    }

    /// Sets the value of PercentOfMaxConcurrentCalls
    pub fn set_percent_of_max_concurrent_calls(&mut self, value: u32) {
        self.percent_of_max_concurrent_calls = Some(value);
    }

    /// Gets the value of PercentOfMaxConcurrentCalls
    pub fn get_percent_of_max_concurrent_calls(&self) -> Option<&u32> {
        self.percent_of_max_concurrent_calls.as_ref()
    }

    /// Sets the value of PercentOfMaxConcurrentInstances
    pub fn set_percent_of_max_concurrent_instances(&mut self, value: u32) {
        self.percent_of_max_concurrent_instances = Some(value);
    }

    /// Gets the value of PercentOfMaxConcurrentInstances
    pub fn get_percent_of_max_concurrent_instances(&self) -> Option<&u32> {
        self.percent_of_max_concurrent_instances.as_ref()
    }

    /// Sets the value of PercentOfMaxConcurrentSessions
    pub fn set_percent_of_max_concurrent_sessions(&mut self, value: u32) {
        self.percent_of_max_concurrent_sessions = Some(value);
    }

    /// Gets the value of PercentOfMaxConcurrentSessions
    pub fn get_percent_of_max_concurrent_sessions(&self) -> Option<&u32> {
        self.percent_of_max_concurrent_sessions.as_ref()
    }

    /// Sets the value of QueuedMessagesDropped
    pub fn set_queued_messages_dropped(&mut self, value: u32) {
        self.queued_messages_dropped = Some(value);
    }

    /// Gets the value of QueuedMessagesDropped
    pub fn get_queued_messages_dropped(&self) -> Option<&u32> {
        self.queued_messages_dropped.as_ref()
    }

    /// Sets the value of QueuedMessagesDroppedPerSecond
    pub fn set_queued_messages_dropped_per_second(&mut self, value: u32) {
        self.queued_messages_dropped_per_second = Some(value);
    }

    /// Gets the value of QueuedMessagesDroppedPerSecond
    pub fn get_queued_messages_dropped_per_second(&self) -> Option<&u32> {
        self.queued_messages_dropped_per_second.as_ref()
    }

    /// Sets the value of QueuedMessagesRejected
    pub fn set_queued_messages_rejected(&mut self, value: u32) {
        self.queued_messages_rejected = Some(value);
    }

    /// Gets the value of QueuedMessagesRejected
    pub fn get_queued_messages_rejected(&self) -> Option<&u32> {
        self.queued_messages_rejected.as_ref()
    }

    /// Sets the value of QueuedMessagesRejectedPerSecond
    pub fn set_queued_messages_rejected_per_second(&mut self, value: u32) {
        self.queued_messages_rejected_per_second = Some(value);
    }

    /// Gets the value of QueuedMessagesRejectedPerSecond
    pub fn get_queued_messages_rejected_per_second(&self) -> Option<&u32> {
        self.queued_messages_rejected_per_second.as_ref()
    }

    /// Sets the value of QueuedPoisonMessages
    pub fn set_queued_poison_messages(&mut self, value: u32) {
        self.queued_poison_messages = Some(value);
    }

    /// Gets the value of QueuedPoisonMessages
    pub fn get_queued_poison_messages(&self) -> Option<&u32> {
        self.queued_poison_messages.as_ref()
    }

    /// Sets the value of QueuedPoisonMessagesPerSecond
    pub fn set_queued_poison_messages_per_second(&mut self, value: u32) {
        self.queued_poison_messages_per_second = Some(value);
    }

    /// Gets the value of QueuedPoisonMessagesPerSecond
    pub fn get_queued_poison_messages_per_second(&self) -> Option<&u32> {
        self.queued_poison_messages_per_second.as_ref()
    }

    /// Sets the value of ReliableMessagingMessagesDropped
    pub fn set_reliable_messaging_messages_dropped(&mut self, value: u32) {
        self.reliable_messaging_messages_dropped = Some(value);
    }

    /// Gets the value of ReliableMessagingMessagesDropped
    pub fn get_reliable_messaging_messages_dropped(&self) -> Option<&u32> {
        self.reliable_messaging_messages_dropped.as_ref()
    }

    /// Sets the value of ReliableMessagingMessagesDroppedPerSecond
    pub fn set_reliable_messaging_messages_dropped_per_second(&mut self, value: u32) {
        self.reliable_messaging_messages_dropped_per_second = Some(value);
    }

    /// Gets the value of ReliableMessagingMessagesDroppedPerSecond
    pub fn get_reliable_messaging_messages_dropped_per_second(&self) -> Option<&u32> {
        self.reliable_messaging_messages_dropped_per_second.as_ref()
    }

    /// Sets the value of ReliableMessagingSessionsFaulted
    pub fn set_reliable_messaging_sessions_faulted(&mut self, value: u32) {
        self.reliable_messaging_sessions_faulted = Some(value);
    }

    /// Gets the value of ReliableMessagingSessionsFaulted
    pub fn get_reliable_messaging_sessions_faulted(&self) -> Option<&u32> {
        self.reliable_messaging_sessions_faulted.as_ref()
    }

    /// Sets the value of ReliableMessagingSessionsFaultedPerSecond
    pub fn set_reliable_messaging_sessions_faulted_per_second(&mut self, value: u32) {
        self.reliable_messaging_sessions_faulted_per_second = Some(value);
    }

    /// Gets the value of ReliableMessagingSessionsFaultedPerSecond
    pub fn get_reliable_messaging_sessions_faulted_per_second(&self) -> Option<&u32> {
        self.reliable_messaging_sessions_faulted_per_second.as_ref()
    }

    /// Sets the value of SecurityCallsNotAuthorized
    pub fn set_security_calls_not_authorized(&mut self, value: u32) {
        self.security_calls_not_authorized = Some(value);
    }

    /// Gets the value of SecurityCallsNotAuthorized
    pub fn get_security_calls_not_authorized(&self) -> Option<&u32> {
        self.security_calls_not_authorized.as_ref()
    }

    /// Sets the value of SecurityCallsNotAuthorizedPerSecond
    pub fn set_security_calls_not_authorized_per_second(&mut self, value: u32) {
        self.security_calls_not_authorized_per_second = Some(value);
    }

    /// Gets the value of SecurityCallsNotAuthorizedPerSecond
    pub fn get_security_calls_not_authorized_per_second(&self) -> Option<&u32> {
        self.security_calls_not_authorized_per_second.as_ref()
    }

    /// Sets the value of SecurityValidationandAuthenticationFailures
    pub fn set_security_validationand_authentication_failures(&mut self, value: u32) {
        self.security_validationand_authentication_failures = Some(value);
    }

    /// Gets the value of SecurityValidationandAuthenticationFailures
    pub fn get_security_validationand_authentication_failures(&self) -> Option<&u32> {
        self.security_validationand_authentication_failures.as_ref()
    }

    /// Sets the value of SecurityValidationandAuthenticationFailuresPerSecond
    pub fn set_security_validationand_authentication_failures_per_second(&mut self, value: u32) {
        self.security_validationand_authentication_failures_per_second = Some(value);
    }

    /// Gets the value of SecurityValidationandAuthenticationFailuresPerSecond
    pub fn get_security_validationand_authentication_failures_per_second(&self) -> Option<&u32> {
        self.security_validationand_authentication_failures_per_second.as_ref()
    }

    /// Sets the value of TransactedOperationsAborted
    pub fn set_transacted_operations_aborted(&mut self, value: u32) {
        self.transacted_operations_aborted = Some(value);
    }

    /// Gets the value of TransactedOperationsAborted
    pub fn get_transacted_operations_aborted(&self) -> Option<&u32> {
        self.transacted_operations_aborted.as_ref()
    }

    /// Sets the value of TransactedOperationsAbortedPerSecond
    pub fn set_transacted_operations_aborted_per_second(&mut self, value: u32) {
        self.transacted_operations_aborted_per_second = Some(value);
    }

    /// Gets the value of TransactedOperationsAbortedPerSecond
    pub fn get_transacted_operations_aborted_per_second(&self) -> Option<&u32> {
        self.transacted_operations_aborted_per_second.as_ref()
    }

    /// Sets the value of TransactedOperationsCommitted
    pub fn set_transacted_operations_committed(&mut self, value: u32) {
        self.transacted_operations_committed = Some(value);
    }

    /// Gets the value of TransactedOperationsCommitted
    pub fn get_transacted_operations_committed(&self) -> Option<&u32> {
        self.transacted_operations_committed.as_ref()
    }

    /// Sets the value of TransactedOperationsCommittedPerSecond
    pub fn set_transacted_operations_committed_per_second(&mut self, value: u32) {
        self.transacted_operations_committed_per_second = Some(value);
    }

    /// Gets the value of TransactedOperationsCommittedPerSecond
    pub fn get_transacted_operations_committed_per_second(&self) -> Option<&u32> {
        self.transacted_operations_committed_per_second.as_ref()
    }

    /// Sets the value of TransactedOperationsInDoubt
    pub fn set_transacted_operations_in_doubt(&mut self, value: u32) {
        self.transacted_operations_in_doubt = Some(value);
    }

    /// Gets the value of TransactedOperationsInDoubt
    pub fn get_transacted_operations_in_doubt(&self) -> Option<&u32> {
        self.transacted_operations_in_doubt.as_ref()
    }

    /// Sets the value of TransactedOperationsInDoubtPerSecond
    pub fn set_transacted_operations_in_doubt_per_second(&mut self, value: u32) {
        self.transacted_operations_in_doubt_per_second = Some(value);
    }

    /// Gets the value of TransactedOperationsInDoubtPerSecond
    pub fn get_transacted_operations_in_doubt_per_second(&self) -> Option<&u32> {
        self.transacted_operations_in_doubt_per_second.as_ref()
    }

    /// Sets the value of TransactionsFlowed
    pub fn set_transactions_flowed(&mut self, value: u32) {
        self.transactions_flowed = Some(value);
    }

    /// Gets the value of TransactionsFlowed
    pub fn get_transactions_flowed(&self) -> Option<&u32> {
        self.transactions_flowed.as_ref()
    }

    /// Sets the value of TransactionsFlowedPerSecond
    pub fn set_transactions_flowed_per_second(&mut self, value: u32) {
        self.transactions_flowed_per_second = Some(value);
    }

    /// Gets the value of TransactionsFlowedPerSecond
    pub fn get_transactions_flowed_per_second(&self) -> Option<&u32> {
        self.transactions_flowed_per_second.as_ref()
    }
}

