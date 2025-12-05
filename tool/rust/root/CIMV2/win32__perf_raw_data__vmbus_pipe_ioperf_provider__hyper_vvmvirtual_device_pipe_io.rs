// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_VMBusPipeIOPerfProvider_HyperVVMVirtualDevicePipeIO struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_VMBusPipeIOPerfProvider_HyperVVMVirtualDevicePipeIO {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "ReceiveMessageQuotaExceeded")]
    pub receive_message_quota_exceeded: Option<u64>,

/// 
    #[serde(rename = "ReceiveQoSConformantMessagesPersec")]
    pub receive_qo_sconformant_messages_persec: Option<u64>,

/// 
    #[serde(rename = "ReceiveQoSExemptMessagesPersec")]
    pub receive_qo_sexempt_messages_persec: Option<u64>,

/// 
    #[serde(rename = "ReceiveQoSNonConformantMessagesPersec")]
    pub receive_qo_snon_conformant_messages_persec: Option<u64>,

/// 
    #[serde(rename = "ReceiveQoSTotalMessageDelayTime100ns")]
    pub receive_qo_stotal_message_delay_time100ns: Option<u64>,
}

impl Win32_PerfRawData_VMBusPipeIOPerfProvider_HyperVVMVirtualDevicePipeIO {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            receive_message_quota_exceeded: None,
            receive_qo_sconformant_messages_persec: None,
            receive_qo_sexempt_messages_persec: None,
            receive_qo_snon_conformant_messages_persec: None,
            receive_qo_stotal_message_delay_time100ns: None,
        }
    }


    /// Sets the value of ReceiveMessageQuotaExceeded
    pub fn set_receive_message_quota_exceeded(&mut self, value: u64) {
        self.receive_message_quota_exceeded = Some(value);
    }

    /// Gets the value of ReceiveMessageQuotaExceeded
    pub fn get_receive_message_quota_exceeded(&self) -> Option<&u64> {
        self.receive_message_quota_exceeded.as_ref()
    }

    /// Sets the value of ReceiveQoSConformantMessagesPersec
    pub fn set_receive_qo_sconformant_messages_persec(&mut self, value: u64) {
        self.receive_qo_sconformant_messages_persec = Some(value);
    }

    /// Gets the value of ReceiveQoSConformantMessagesPersec
    pub fn get_receive_qo_sconformant_messages_persec(&self) -> Option<&u64> {
        self.receive_qo_sconformant_messages_persec.as_ref()
    }

    /// Sets the value of ReceiveQoSExemptMessagesPersec
    pub fn set_receive_qo_sexempt_messages_persec(&mut self, value: u64) {
        self.receive_qo_sexempt_messages_persec = Some(value);
    }

    /// Gets the value of ReceiveQoSExemptMessagesPersec
    pub fn get_receive_qo_sexempt_messages_persec(&self) -> Option<&u64> {
        self.receive_qo_sexempt_messages_persec.as_ref()
    }

    /// Sets the value of ReceiveQoSNonConformantMessagesPersec
    pub fn set_receive_qo_snon_conformant_messages_persec(&mut self, value: u64) {
        self.receive_qo_snon_conformant_messages_persec = Some(value);
    }

    /// Gets the value of ReceiveQoSNonConformantMessagesPersec
    pub fn get_receive_qo_snon_conformant_messages_persec(&self) -> Option<&u64> {
        self.receive_qo_snon_conformant_messages_persec.as_ref()
    }

    /// Sets the value of ReceiveQoSTotalMessageDelayTime100ns
    pub fn set_receive_qo_stotal_message_delay_time100ns(&mut self, value: u64) {
        self.receive_qo_stotal_message_delay_time100ns = Some(value);
    }

    /// Gets the value of ReceiveQoSTotalMessageDelayTime100ns
    pub fn get_receive_qo_stotal_message_delay_time100ns(&self) -> Option<&u64> {
        self.receive_qo_stotal_message_delay_time100ns.as_ref()
    }
}

