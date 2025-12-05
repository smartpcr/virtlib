// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSStorageDriver_ScsiRequestBlock struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSStorageDriver_ScsiRequestBlock {

/// CDB
    #[serde(rename = "cdb")]
    pub cdb: Vec<u8>,

/// CDB Length
    #[serde(rename = "cdbLength")]
    pub cdb_length: Option<u8>,

/// Data Buffer Pointer
    #[serde(rename = "dataBuffer")]
    pub data_buffer: Option<u64>,

/// Data Transfer Length
    #[serde(rename = "dataTransferLength")]
    pub data_transfer_length: Option<u32>,

/// Function
    #[serde(rename = "function")]
    pub function: Option<u8>,

/// Internal Status
    #[serde(rename = "internalStatus")]
    pub internal_status: Option<u32>,

/// Length
    #[serde(rename = "length")]
    pub length: Option<u16>,

/// LUN
    #[serde(rename = "lun")]
    pub lun: Option<u8>,

/// Next SRB Pointer
    #[serde(rename = "nextSRB")]
    pub next_srb: Option<u64>,

/// Original Request Pointer
    #[serde(rename = "originalRequest")]
    pub original_request: Option<u64>,

/// Path ID
    #[serde(rename = "pathID")]
    pub path_id: Option<u8>,

/// Queue Action
    #[serde(rename = "queueAction")]
    pub queue_action: Option<u8>,

/// Queue Tag
    #[serde(rename = "queueTag")]
    pub queue_tag: Option<u8>,

/// Reserved (only available in Win64)
    #[serde(rename = "reserved")]
    pub reserved: Option<u32>,

/// SCSI Status
    #[serde(rename = "scsiStatus")]
    pub scsi_status: Option<u8>,

/// Sense Info Buffer Pointer
    #[serde(rename = "senseInfoBuffer")]
    pub sense_info_buffer: Option<u64>,

/// Sense Info Buffer Length
    #[serde(rename = "senseInfoBufferLength")]
    pub sense_info_buffer_length: Option<u8>,

/// SRB Extension Pointer
    #[serde(rename = "srbExtension")]
    pub srb_extension: Option<u64>,

/// SRB Flags
    #[serde(rename = "srbFlags")]
    pub srb_flags: Option<u32>,

/// SRB Status
    #[serde(rename = "srbStatus")]
    pub srb_status: Option<u8>,

/// Target ID
    #[serde(rename = "targetID")]
    pub target_id: Option<u8>,

/// Time Out Value
    #[serde(rename = "timeOutValue")]
    pub time_out_value: Option<u32>,
}

impl MSStorageDriver_ScsiRequestBlock {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            cdb: Vec::new(),
            cdb_length: None,
            data_buffer: None,
            data_transfer_length: None,
            function: None,
            internal_status: None,
            length: None,
            lun: None,
            next_srb: None,
            original_request: None,
            path_id: None,
            queue_action: None,
            queue_tag: None,
            reserved: None,
            scsi_status: None,
            sense_info_buffer: None,
            sense_info_buffer_length: None,
            srb_extension: None,
            srb_flags: None,
            srb_status: None,
            target_id: None,
            time_out_value: None,
        }
    }


    /// Sets the value of cdb
    pub fn set_cdb(&mut self, value: Vec<u8>) {
        self.cdb = value;
    }

    /// Gets the value of cdb
    pub fn get_cdb(&self) -> &Vec<u8> {
        &self.cdb
    }

    /// Sets the value of cdbLength
    pub fn set_cdb_length(&mut self, value: u8) {
        self.cdb_length = Some(value);
    }

    /// Gets the value of cdbLength
    pub fn get_cdb_length(&self) -> Option<&u8> {
        self.cdb_length.as_ref()
    }

    /// Sets the value of dataBuffer
    pub fn set_data_buffer(&mut self, value: u64) {
        self.data_buffer = Some(value);
    }

    /// Gets the value of dataBuffer
    pub fn get_data_buffer(&self) -> Option<&u64> {
        self.data_buffer.as_ref()
    }

    /// Sets the value of dataTransferLength
    pub fn set_data_transfer_length(&mut self, value: u32) {
        self.data_transfer_length = Some(value);
    }

    /// Gets the value of dataTransferLength
    pub fn get_data_transfer_length(&self) -> Option<&u32> {
        self.data_transfer_length.as_ref()
    }

    /// Sets the value of function
    pub fn set_function(&mut self, value: u8) {
        self.function = Some(value);
    }

    /// Gets the value of function
    pub fn get_function(&self) -> Option<&u8> {
        self.function.as_ref()
    }

    /// Sets the value of internalStatus
    pub fn set_internal_status(&mut self, value: u32) {
        self.internal_status = Some(value);
    }

    /// Gets the value of internalStatus
    pub fn get_internal_status(&self) -> Option<&u32> {
        self.internal_status.as_ref()
    }

    /// Sets the value of length
    pub fn set_length(&mut self, value: u16) {
        self.length = Some(value);
    }

    /// Gets the value of length
    pub fn get_length(&self) -> Option<&u16> {
        self.length.as_ref()
    }

    /// Sets the value of lun
    pub fn set_lun(&mut self, value: u8) {
        self.lun = Some(value);
    }

    /// Gets the value of lun
    pub fn get_lun(&self) -> Option<&u8> {
        self.lun.as_ref()
    }

    /// Sets the value of nextSRB
    pub fn set_next_srb(&mut self, value: u64) {
        self.next_srb = Some(value);
    }

    /// Gets the value of nextSRB
    pub fn get_next_srb(&self) -> Option<&u64> {
        self.next_srb.as_ref()
    }

    /// Sets the value of originalRequest
    pub fn set_original_request(&mut self, value: u64) {
        self.original_request = Some(value);
    }

    /// Gets the value of originalRequest
    pub fn get_original_request(&self) -> Option<&u64> {
        self.original_request.as_ref()
    }

    /// Sets the value of pathID
    pub fn set_path_id(&mut self, value: u8) {
        self.path_id = Some(value);
    }

    /// Gets the value of pathID
    pub fn get_path_id(&self) -> Option<&u8> {
        self.path_id.as_ref()
    }

    /// Sets the value of queueAction
    pub fn set_queue_action(&mut self, value: u8) {
        self.queue_action = Some(value);
    }

    /// Gets the value of queueAction
    pub fn get_queue_action(&self) -> Option<&u8> {
        self.queue_action.as_ref()
    }

    /// Sets the value of queueTag
    pub fn set_queue_tag(&mut self, value: u8) {
        self.queue_tag = Some(value);
    }

    /// Gets the value of queueTag
    pub fn get_queue_tag(&self) -> Option<&u8> {
        self.queue_tag.as_ref()
    }

    /// Sets the value of reserved
    pub fn set_reserved(&mut self, value: u32) {
        self.reserved = Some(value);
    }

    /// Gets the value of reserved
    pub fn get_reserved(&self) -> Option<&u32> {
        self.reserved.as_ref()
    }

    /// Sets the value of scsiStatus
    pub fn set_scsi_status(&mut self, value: u8) {
        self.scsi_status = Some(value);
    }

    /// Gets the value of scsiStatus
    pub fn get_scsi_status(&self) -> Option<&u8> {
        self.scsi_status.as_ref()
    }

    /// Sets the value of senseInfoBuffer
    pub fn set_sense_info_buffer(&mut self, value: u64) {
        self.sense_info_buffer = Some(value);
    }

    /// Gets the value of senseInfoBuffer
    pub fn get_sense_info_buffer(&self) -> Option<&u64> {
        self.sense_info_buffer.as_ref()
    }

    /// Sets the value of senseInfoBufferLength
    pub fn set_sense_info_buffer_length(&mut self, value: u8) {
        self.sense_info_buffer_length = Some(value);
    }

    /// Gets the value of senseInfoBufferLength
    pub fn get_sense_info_buffer_length(&self) -> Option<&u8> {
        self.sense_info_buffer_length.as_ref()
    }

    /// Sets the value of srbExtension
    pub fn set_srb_extension(&mut self, value: u64) {
        self.srb_extension = Some(value);
    }

    /// Gets the value of srbExtension
    pub fn get_srb_extension(&self) -> Option<&u64> {
        self.srb_extension.as_ref()
    }

    /// Sets the value of srbFlags
    pub fn set_srb_flags(&mut self, value: u32) {
        self.srb_flags = Some(value);
    }

    /// Gets the value of srbFlags
    pub fn get_srb_flags(&self) -> Option<&u32> {
        self.srb_flags.as_ref()
    }

    /// Sets the value of srbStatus
    pub fn set_srb_status(&mut self, value: u8) {
        self.srb_status = Some(value);
    }

    /// Gets the value of srbStatus
    pub fn get_srb_status(&self) -> Option<&u8> {
        self.srb_status.as_ref()
    }

    /// Sets the value of targetID
    pub fn set_target_id(&mut self, value: u8) {
        self.target_id = Some(value);
    }

    /// Gets the value of targetID
    pub fn get_target_id(&self) -> Option<&u8> {
        self.target_id.as_ref()
    }

    /// Sets the value of timeOutValue
    pub fn set_time_out_value(&mut self, value: u32) {
        self.time_out_value = Some(value);
    }

    /// Gets the value of timeOutValue
    pub fn get_time_out_value(&self) -> Option<&u32> {
        self.time_out_value.as_ref()
    }
}

