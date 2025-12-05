// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_ReceiveQueueParameters struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_ReceiveQueueParameters {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Flags")]
    pub flags: Option<u32>,

/// 
    #[serde(rename = "Header")]
    pub header: Option<MSNdis_ObjectHeader>,

/// 
    #[serde(rename = "LookaheadSize")]
    pub lookahead_size: Option<u32>,

/// 
    #[serde(rename = "MSIXTableEntry")]
    pub msixtable_entry: Option<u32>,

/// 
    #[serde(rename = "NumSuggestedReceiveBuffers")]
    pub num_suggested_receive_buffers: Option<u32>,

/// 
    #[serde(rename = "ProcessorAffinity")]
    pub processor_affinity: Option<MSNdis_GroupAffinity>,

/// 
    #[serde(rename = "QueueGroupId")]
    pub queue_group_id: Option<u32>,

/// 
    #[serde(rename = "QueueId")]
    pub queue_id: Option<u32>,

/// 
    #[serde(rename = "QueueName")]
    pub queue_name: Option<MSNdis_CountedString>,

/// 
    #[serde(rename = "QueueType")]
    pub queue_type: Option<u32>,

/// 
    #[serde(rename = "VmName")]
    pub vm_name: Option<MSNdis_CountedString>,
}

impl MSNdis_ReceiveQueueParameters {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            flags: None,
            header: None,
            lookahead_size: None,
            msixtable_entry: None,
            num_suggested_receive_buffers: None,
            processor_affinity: None,
            queue_group_id: None,
            queue_id: None,
            queue_name: None,
            queue_type: None,
            vm_name: None,
        }
    }


    /// Sets the value of Flags
    pub fn set_flags(&mut self, value: u32) {
        self.flags = Some(value);
    }

    /// Gets the value of Flags
    pub fn get_flags(&self) -> Option<&u32> {
        self.flags.as_ref()
    }

    /// Sets the value of Header
    pub fn set_header(&mut self, value: MSNdis_ObjectHeader) {
        self.header = Some(value);
    }

    /// Gets the value of Header
    pub fn get_header(&self) -> Option<&MSNdis_ObjectHeader> {
        self.header.as_ref()
    }

    /// Sets the value of LookaheadSize
    pub fn set_lookahead_size(&mut self, value: u32) {
        self.lookahead_size = Some(value);
    }

    /// Gets the value of LookaheadSize
    pub fn get_lookahead_size(&self) -> Option<&u32> {
        self.lookahead_size.as_ref()
    }

    /// Sets the value of MSIXTableEntry
    pub fn set_msixtable_entry(&mut self, value: u32) {
        self.msixtable_entry = Some(value);
    }

    /// Gets the value of MSIXTableEntry
    pub fn get_msixtable_entry(&self) -> Option<&u32> {
        self.msixtable_entry.as_ref()
    }

    /// Sets the value of NumSuggestedReceiveBuffers
    pub fn set_num_suggested_receive_buffers(&mut self, value: u32) {
        self.num_suggested_receive_buffers = Some(value);
    }

    /// Gets the value of NumSuggestedReceiveBuffers
    pub fn get_num_suggested_receive_buffers(&self) -> Option<&u32> {
        self.num_suggested_receive_buffers.as_ref()
    }

    /// Sets the value of ProcessorAffinity
    pub fn set_processor_affinity(&mut self, value: MSNdis_GroupAffinity) {
        self.processor_affinity = Some(value);
    }

    /// Gets the value of ProcessorAffinity
    pub fn get_processor_affinity(&self) -> Option<&MSNdis_GroupAffinity> {
        self.processor_affinity.as_ref()
    }

    /// Sets the value of QueueGroupId
    pub fn set_queue_group_id(&mut self, value: u32) {
        self.queue_group_id = Some(value);
    }

    /// Gets the value of QueueGroupId
    pub fn get_queue_group_id(&self) -> Option<&u32> {
        self.queue_group_id.as_ref()
    }

    /// Sets the value of QueueId
    pub fn set_queue_id(&mut self, value: u32) {
        self.queue_id = Some(value);
    }

    /// Gets the value of QueueId
    pub fn get_queue_id(&self) -> Option<&u32> {
        self.queue_id.as_ref()
    }

    /// Sets the value of QueueName
    pub fn set_queue_name(&mut self, value: MSNdis_CountedString) {
        self.queue_name = Some(value);
    }

    /// Gets the value of QueueName
    pub fn get_queue_name(&self) -> Option<&MSNdis_CountedString> {
        self.queue_name.as_ref()
    }

    /// Sets the value of QueueType
    pub fn set_queue_type(&mut self, value: u32) {
        self.queue_type = Some(value);
    }

    /// Gets the value of QueueType
    pub fn get_queue_type(&self) -> Option<&u32> {
        self.queue_type.as_ref()
    }

    /// Sets the value of VmName
    pub fn set_vm_name(&mut self, value: MSNdis_CountedString) {
        self.vm_name = Some(value);
    }

    /// Gets the value of VmName
    pub fn get_vm_name(&self) -> Option<&MSNdis_CountedString> {
        self.vm_name.as_ref()
    }
}

