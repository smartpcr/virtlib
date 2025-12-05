// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSI_HBASessionConfig struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSI_HBASessionConfig {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// maximum amount in bytes of unsolicited data an iSCSI initiator may send to the target, during the execution of a single SCSI command. This covers the immediate data (if any) and the sequence of unsolicited Data-Out PDUs (if any) that follow the command.
    #[serde(rename = "FirstBurstLength")]
    pub first_burst_length: Option<u32>,

/// The initiator and target negotiate support for immediate data. To turn immediate data off, the initiator or target must state its desire to do so.  ImmediateData can be turned on if both the initiator and target have ImmediateData=Yes.
    #[serde(rename = "ImmediateData")]
    pub immediate_data: Option<bool>,

/// The InitialR2T key is used to turn off the default use of R2T, thus allowing an initiator to start sending data to a target as if it has received an initial R2T with Buffer Offset=0 and Desired Data Transfer Length=min (FirstBurstSize, Expected Data Transfer Length).
    #[serde(rename = "InitialR2T")]
    pub initial_r2_t: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// Maximum SCSI data payload in bytes in an Data-In or a solicited Data-Out iSCSI sequence.
    #[serde(rename = "MaxBurstLength")]
    pub max_burst_length: Option<u32>,

/// Initiator and target negotiate the maximum number of outstanding R2Ts per task, excluding any implied initial R2T that might be part of that task.  An R2T is considered outstanding until the last data PDU (with the F bit set to 1) is transferred, or a sequence reception timeout (section 6.12.1) is encountered for that data sequence.
    #[serde(rename = "MaxOutstandingR2T")]
    pub max_outstanding_r2_t: Option<u32>,

/// Maximum data segment length in bytes they can receive in an iSCSI PDU.
    #[serde(rename = "MaxRecvDataSegmentLength")]
    pub max_recv_data_segment_length: Option<u32>,
}

impl MSiSCSI_HBASessionConfig {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active: None,
            first_burst_length: None,
            immediate_data: None,
            initial_r2_t: None,
            instance_name: None,
            max_burst_length: None,
            max_outstanding_r2_t: None,
            max_recv_data_segment_length: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of FirstBurstLength
    pub fn set_first_burst_length(&mut self, value: u32) {
        self.first_burst_length = Some(value);
    }

    /// Gets the value of FirstBurstLength
    pub fn get_first_burst_length(&self) -> Option<&u32> {
        self.first_burst_length.as_ref()
    }

    /// Sets the value of ImmediateData
    pub fn set_immediate_data(&mut self, value: bool) {
        self.immediate_data = Some(value);
    }

    /// Gets the value of ImmediateData
    pub fn get_immediate_data(&self) -> Option<&bool> {
        self.immediate_data.as_ref()
    }

    /// Sets the value of InitialR2T
    pub fn set_initial_r2_t(&mut self, value: bool) {
        self.initial_r2_t = Some(value);
    }

    /// Gets the value of InitialR2T
    pub fn get_initial_r2_t(&self) -> Option<&bool> {
        self.initial_r2_t.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of MaxBurstLength
    pub fn set_max_burst_length(&mut self, value: u32) {
        self.max_burst_length = Some(value);
    }

    /// Gets the value of MaxBurstLength
    pub fn get_max_burst_length(&self) -> Option<&u32> {
        self.max_burst_length.as_ref()
    }

    /// Sets the value of MaxOutstandingR2T
    pub fn set_max_outstanding_r2_t(&mut self, value: u32) {
        self.max_outstanding_r2_t = Some(value);
    }

    /// Gets the value of MaxOutstandingR2T
    pub fn get_max_outstanding_r2_t(&self) -> Option<&u32> {
        self.max_outstanding_r2_t.as_ref()
    }

    /// Sets the value of MaxRecvDataSegmentLength
    pub fn set_max_recv_data_segment_length(&mut self, value: u32) {
        self.max_recv_data_segment_length = Some(value);
    }

    /// Gets the value of MaxRecvDataSegmentLength
    pub fn get_max_recv_data_segment_length(&self) -> Option<&u32> {
        self.max_recv_data_segment_length.as_ref()
    }
}

