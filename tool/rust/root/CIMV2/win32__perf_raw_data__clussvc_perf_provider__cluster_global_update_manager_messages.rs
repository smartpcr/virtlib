// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_ClussvcPerfProvider_ClusterGlobalUpdateManagerMessages struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_ClussvcPerfProvider_ClusterGlobalUpdateManagerMessages {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "AverageDatabaseMessagesExecutionTime")]
    pub average_database_messages_execution_time: Option<u32>,

/// 
    #[serde(rename = "AverageDatabaseMessagesExecutionTime_Base")]
    pub average_database_messages_execution_time__base: Option<u32>,

/// 
    #[serde(rename = "AverageMessagesExecutionTime")]
    pub average_messages_execution_time: Option<u32>,

/// 
    #[serde(rename = "AverageMessagesExecutionTime_Base")]
    pub average_messages_execution_time__base: Option<u32>,

/// 
    #[serde(rename = "AverageWaitingTimeToExecuteDatabaseMessages")]
    pub average_waiting_time_to_execute_database_messages: Option<u32>,

/// 
    #[serde(rename = "AverageWaitingTimeToExecuteDatabaseMessages_Base")]
    pub average_waiting_time_to_execute_database_messages__base: Option<u32>,

/// 
    #[serde(rename = "AverageWaitingTimeToExecuteMessages")]
    pub average_waiting_time_to_execute_messages: Option<u32>,

/// 
    #[serde(rename = "AverageWaitingTimeToExecuteMessages_Base")]
    pub average_waiting_time_to_execute_messages__base: Option<u32>,

/// 
    #[serde(rename = "DatabaseMessagesQueueLength")]
    pub database_messages_queue_length: Option<u64>,

/// 
    #[serde(rename = "DatabaseUpdateMessages")]
    pub database_update_messages: Option<u64>,

/// 
    #[serde(rename = "DatabaseUpdateMessagesPersec")]
    pub database_update_messages_persec: Option<u64>,

/// 
    #[serde(rename = "MessagesExecutionQueueLength")]
    pub messages_execution_queue_length: Option<u64>,

/// 
    #[serde(rename = "MessagesQueueLength")]
    pub messages_queue_length: Option<u64>,

/// 
    #[serde(rename = "UpdateMessages")]
    pub update_messages: Option<u64>,

/// 
    #[serde(rename = "UpdateMessagesPersec")]
    pub update_messages_persec: Option<u64>,
}

impl Win32_PerfRawData_ClussvcPerfProvider_ClusterGlobalUpdateManagerMessages {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            average_database_messages_execution_time: None,
            average_database_messages_execution_time__base: None,
            average_messages_execution_time: None,
            average_messages_execution_time__base: None,
            average_waiting_time_to_execute_database_messages: None,
            average_waiting_time_to_execute_database_messages__base: None,
            average_waiting_time_to_execute_messages: None,
            average_waiting_time_to_execute_messages__base: None,
            database_messages_queue_length: None,
            database_update_messages: None,
            database_update_messages_persec: None,
            messages_execution_queue_length: None,
            messages_queue_length: None,
            update_messages: None,
            update_messages_persec: None,
        }
    }


    /// Sets the value of AverageDatabaseMessagesExecutionTime
    pub fn set_average_database_messages_execution_time(&mut self, value: u32) {
        self.average_database_messages_execution_time = Some(value);
    }

    /// Gets the value of AverageDatabaseMessagesExecutionTime
    pub fn get_average_database_messages_execution_time(&self) -> Option<&u32> {
        self.average_database_messages_execution_time.as_ref()
    }

    /// Sets the value of AverageDatabaseMessagesExecutionTime_Base
    pub fn set_average_database_messages_execution_time__base(&mut self, value: u32) {
        self.average_database_messages_execution_time__base = Some(value);
    }

    /// Gets the value of AverageDatabaseMessagesExecutionTime_Base
    pub fn get_average_database_messages_execution_time__base(&self) -> Option<&u32> {
        self.average_database_messages_execution_time__base.as_ref()
    }

    /// Sets the value of AverageMessagesExecutionTime
    pub fn set_average_messages_execution_time(&mut self, value: u32) {
        self.average_messages_execution_time = Some(value);
    }

    /// Gets the value of AverageMessagesExecutionTime
    pub fn get_average_messages_execution_time(&self) -> Option<&u32> {
        self.average_messages_execution_time.as_ref()
    }

    /// Sets the value of AverageMessagesExecutionTime_Base
    pub fn set_average_messages_execution_time__base(&mut self, value: u32) {
        self.average_messages_execution_time__base = Some(value);
    }

    /// Gets the value of AverageMessagesExecutionTime_Base
    pub fn get_average_messages_execution_time__base(&self) -> Option<&u32> {
        self.average_messages_execution_time__base.as_ref()
    }

    /// Sets the value of AverageWaitingTimeToExecuteDatabaseMessages
    pub fn set_average_waiting_time_to_execute_database_messages(&mut self, value: u32) {
        self.average_waiting_time_to_execute_database_messages = Some(value);
    }

    /// Gets the value of AverageWaitingTimeToExecuteDatabaseMessages
    pub fn get_average_waiting_time_to_execute_database_messages(&self) -> Option<&u32> {
        self.average_waiting_time_to_execute_database_messages.as_ref()
    }

    /// Sets the value of AverageWaitingTimeToExecuteDatabaseMessages_Base
    pub fn set_average_waiting_time_to_execute_database_messages__base(&mut self, value: u32) {
        self.average_waiting_time_to_execute_database_messages__base = Some(value);
    }

    /// Gets the value of AverageWaitingTimeToExecuteDatabaseMessages_Base
    pub fn get_average_waiting_time_to_execute_database_messages__base(&self) -> Option<&u32> {
        self.average_waiting_time_to_execute_database_messages__base.as_ref()
    }

    /// Sets the value of AverageWaitingTimeToExecuteMessages
    pub fn set_average_waiting_time_to_execute_messages(&mut self, value: u32) {
        self.average_waiting_time_to_execute_messages = Some(value);
    }

    /// Gets the value of AverageWaitingTimeToExecuteMessages
    pub fn get_average_waiting_time_to_execute_messages(&self) -> Option<&u32> {
        self.average_waiting_time_to_execute_messages.as_ref()
    }

    /// Sets the value of AverageWaitingTimeToExecuteMessages_Base
    pub fn set_average_waiting_time_to_execute_messages__base(&mut self, value: u32) {
        self.average_waiting_time_to_execute_messages__base = Some(value);
    }

    /// Gets the value of AverageWaitingTimeToExecuteMessages_Base
    pub fn get_average_waiting_time_to_execute_messages__base(&self) -> Option<&u32> {
        self.average_waiting_time_to_execute_messages__base.as_ref()
    }

    /// Sets the value of DatabaseMessagesQueueLength
    pub fn set_database_messages_queue_length(&mut self, value: u64) {
        self.database_messages_queue_length = Some(value);
    }

    /// Gets the value of DatabaseMessagesQueueLength
    pub fn get_database_messages_queue_length(&self) -> Option<&u64> {
        self.database_messages_queue_length.as_ref()
    }

    /// Sets the value of DatabaseUpdateMessages
    pub fn set_database_update_messages(&mut self, value: u64) {
        self.database_update_messages = Some(value);
    }

    /// Gets the value of DatabaseUpdateMessages
    pub fn get_database_update_messages(&self) -> Option<&u64> {
        self.database_update_messages.as_ref()
    }

    /// Sets the value of DatabaseUpdateMessagesPersec
    pub fn set_database_update_messages_persec(&mut self, value: u64) {
        self.database_update_messages_persec = Some(value);
    }

    /// Gets the value of DatabaseUpdateMessagesPersec
    pub fn get_database_update_messages_persec(&self) -> Option<&u64> {
        self.database_update_messages_persec.as_ref()
    }

    /// Sets the value of MessagesExecutionQueueLength
    pub fn set_messages_execution_queue_length(&mut self, value: u64) {
        self.messages_execution_queue_length = Some(value);
    }

    /// Gets the value of MessagesExecutionQueueLength
    pub fn get_messages_execution_queue_length(&self) -> Option<&u64> {
        self.messages_execution_queue_length.as_ref()
    }

    /// Sets the value of MessagesQueueLength
    pub fn set_messages_queue_length(&mut self, value: u64) {
        self.messages_queue_length = Some(value);
    }

    /// Gets the value of MessagesQueueLength
    pub fn get_messages_queue_length(&self) -> Option<&u64> {
        self.messages_queue_length.as_ref()
    }

    /// Sets the value of UpdateMessages
    pub fn set_update_messages(&mut self, value: u64) {
        self.update_messages = Some(value);
    }

    /// Gets the value of UpdateMessages
    pub fn get_update_messages(&self) -> Option<&u64> {
        self.update_messages.as_ref()
    }

    /// Sets the value of UpdateMessagesPersec
    pub fn set_update_messages_persec(&mut self, value: u64) {
        self.update_messages_persec = Some(value);
    }

    /// Gets the value of UpdateMessagesPersec
    pub fn get_update_messages_persec(&self) -> Option<&u64> {
        self.update_messages_persec.as_ref()
    }
}

