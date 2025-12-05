// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_PerfNet_ServerWorkQueues struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_PerfNet_ServerWorkQueues {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "ActiveThreads")]
    pub active_threads: Option<u32>,

/// 
    #[serde(rename = "AvailableThreads")]
    pub available_threads: Option<u32>,

/// 
    #[serde(rename = "AvailableWorkItems")]
    pub available_work_items: Option<u32>,

/// 
    #[serde(rename = "BorrowedWorkItems")]
    pub borrowed_work_items: Option<u32>,

/// 
    #[serde(rename = "BytesReceivedPersec")]
    pub bytes_received_persec: Option<u64>,

/// 
    #[serde(rename = "BytesSentPersec")]
    pub bytes_sent_persec: Option<u64>,

/// 
    #[serde(rename = "BytesTransferredPersec")]
    pub bytes_transferred_persec: Option<u64>,

/// 
    #[serde(rename = "ContextBlocksQueuedPersec")]
    pub context_blocks_queued_persec: Option<u32>,

/// 
    #[serde(rename = "CurrentClients")]
    pub current_clients: Option<u32>,

/// 
    #[serde(rename = "QueueLength")]
    pub queue_length: Option<u32>,

/// 
    #[serde(rename = "ReadBytesPersec")]
    pub read_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "ReadOperationsPersec")]
    pub read_operations_persec: Option<u64>,

/// 
    #[serde(rename = "TotalBytesPersec")]
    pub total_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "TotalOperationsPersec")]
    pub total_operations_persec: Option<u64>,

/// 
    #[serde(rename = "WorkItemShortages")]
    pub work_item_shortages: Option<u32>,

/// 
    #[serde(rename = "WriteBytesPersec")]
    pub write_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "WriteOperationsPersec")]
    pub write_operations_persec: Option<u64>,
}

impl Win32_PerfRawData_PerfNet_ServerWorkQueues {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            active_threads: None,
            available_threads: None,
            available_work_items: None,
            borrowed_work_items: None,
            bytes_received_persec: None,
            bytes_sent_persec: None,
            bytes_transferred_persec: None,
            context_blocks_queued_persec: None,
            current_clients: None,
            queue_length: None,
            read_bytes_persec: None,
            read_operations_persec: None,
            total_bytes_persec: None,
            total_operations_persec: None,
            work_item_shortages: None,
            write_bytes_persec: None,
            write_operations_persec: None,
        }
    }


    /// Sets the value of ActiveThreads
    pub fn set_active_threads(&mut self, value: u32) {
        self.active_threads = Some(value);
    }

    /// Gets the value of ActiveThreads
    pub fn get_active_threads(&self) -> Option<&u32> {
        self.active_threads.as_ref()
    }

    /// Sets the value of AvailableThreads
    pub fn set_available_threads(&mut self, value: u32) {
        self.available_threads = Some(value);
    }

    /// Gets the value of AvailableThreads
    pub fn get_available_threads(&self) -> Option<&u32> {
        self.available_threads.as_ref()
    }

    /// Sets the value of AvailableWorkItems
    pub fn set_available_work_items(&mut self, value: u32) {
        self.available_work_items = Some(value);
    }

    /// Gets the value of AvailableWorkItems
    pub fn get_available_work_items(&self) -> Option<&u32> {
        self.available_work_items.as_ref()
    }

    /// Sets the value of BorrowedWorkItems
    pub fn set_borrowed_work_items(&mut self, value: u32) {
        self.borrowed_work_items = Some(value);
    }

    /// Gets the value of BorrowedWorkItems
    pub fn get_borrowed_work_items(&self) -> Option<&u32> {
        self.borrowed_work_items.as_ref()
    }

    /// Sets the value of BytesReceivedPersec
    pub fn set_bytes_received_persec(&mut self, value: u64) {
        self.bytes_received_persec = Some(value);
    }

    /// Gets the value of BytesReceivedPersec
    pub fn get_bytes_received_persec(&self) -> Option<&u64> {
        self.bytes_received_persec.as_ref()
    }

    /// Sets the value of BytesSentPersec
    pub fn set_bytes_sent_persec(&mut self, value: u64) {
        self.bytes_sent_persec = Some(value);
    }

    /// Gets the value of BytesSentPersec
    pub fn get_bytes_sent_persec(&self) -> Option<&u64> {
        self.bytes_sent_persec.as_ref()
    }

    /// Sets the value of BytesTransferredPersec
    pub fn set_bytes_transferred_persec(&mut self, value: u64) {
        self.bytes_transferred_persec = Some(value);
    }

    /// Gets the value of BytesTransferredPersec
    pub fn get_bytes_transferred_persec(&self) -> Option<&u64> {
        self.bytes_transferred_persec.as_ref()
    }

    /// Sets the value of ContextBlocksQueuedPersec
    pub fn set_context_blocks_queued_persec(&mut self, value: u32) {
        self.context_blocks_queued_persec = Some(value);
    }

    /// Gets the value of ContextBlocksQueuedPersec
    pub fn get_context_blocks_queued_persec(&self) -> Option<&u32> {
        self.context_blocks_queued_persec.as_ref()
    }

    /// Sets the value of CurrentClients
    pub fn set_current_clients(&mut self, value: u32) {
        self.current_clients = Some(value);
    }

    /// Gets the value of CurrentClients
    pub fn get_current_clients(&self) -> Option<&u32> {
        self.current_clients.as_ref()
    }

    /// Sets the value of QueueLength
    pub fn set_queue_length(&mut self, value: u32) {
        self.queue_length = Some(value);
    }

    /// Gets the value of QueueLength
    pub fn get_queue_length(&self) -> Option<&u32> {
        self.queue_length.as_ref()
    }

    /// Sets the value of ReadBytesPersec
    pub fn set_read_bytes_persec(&mut self, value: u64) {
        self.read_bytes_persec = Some(value);
    }

    /// Gets the value of ReadBytesPersec
    pub fn get_read_bytes_persec(&self) -> Option<&u64> {
        self.read_bytes_persec.as_ref()
    }

    /// Sets the value of ReadOperationsPersec
    pub fn set_read_operations_persec(&mut self, value: u64) {
        self.read_operations_persec = Some(value);
    }

    /// Gets the value of ReadOperationsPersec
    pub fn get_read_operations_persec(&self) -> Option<&u64> {
        self.read_operations_persec.as_ref()
    }

    /// Sets the value of TotalBytesPersec
    pub fn set_total_bytes_persec(&mut self, value: u64) {
        self.total_bytes_persec = Some(value);
    }

    /// Gets the value of TotalBytesPersec
    pub fn get_total_bytes_persec(&self) -> Option<&u64> {
        self.total_bytes_persec.as_ref()
    }

    /// Sets the value of TotalOperationsPersec
    pub fn set_total_operations_persec(&mut self, value: u64) {
        self.total_operations_persec = Some(value);
    }

    /// Gets the value of TotalOperationsPersec
    pub fn get_total_operations_persec(&self) -> Option<&u64> {
        self.total_operations_persec.as_ref()
    }

    /// Sets the value of WorkItemShortages
    pub fn set_work_item_shortages(&mut self, value: u32) {
        self.work_item_shortages = Some(value);
    }

    /// Gets the value of WorkItemShortages
    pub fn get_work_item_shortages(&self) -> Option<&u32> {
        self.work_item_shortages.as_ref()
    }

    /// Sets the value of WriteBytesPersec
    pub fn set_write_bytes_persec(&mut self, value: u64) {
        self.write_bytes_persec = Some(value);
    }

    /// Gets the value of WriteBytesPersec
    pub fn get_write_bytes_persec(&self) -> Option<&u64> {
        self.write_bytes_persec.as_ref()
    }

    /// Sets the value of WriteOperationsPersec
    pub fn set_write_operations_persec(&mut self, value: u64) {
        self.write_operations_persec = Some(value);
    }

    /// Gets the value of WriteOperationsPersec
    pub fn get_write_operations_persec(&self) -> Option<&u64> {
        self.write_operations_persec.as_ref()
    }
}

