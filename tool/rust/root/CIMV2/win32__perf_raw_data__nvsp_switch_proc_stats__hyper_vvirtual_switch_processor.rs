// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_NvspSwitchProcStats_HyperVVirtualSwitchProcessor struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_NvspSwitchProcStats_HyperVVirtualSwitchProcessor {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "NumberofTransmitCompletesPersec")]
    pub numberof_transmit_completes_persec: Option<u64>,

/// 
    #[serde(rename = "NumberofVMQs")]
    pub numberof_vmqs: Option<u64>,

/// 
    #[serde(rename = "PacketsfromExternalPersec")]
    pub packetsfrom_external_persec: Option<u64>,

/// 
    #[serde(rename = "PacketsfromInternalPersec")]
    pub packetsfrom_internal_persec: Option<u64>,
}

impl Win32_PerfRawData_NvspSwitchProcStats_HyperVVirtualSwitchProcessor {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            numberof_transmit_completes_persec: None,
            numberof_vmqs: None,
            packetsfrom_external_persec: None,
            packetsfrom_internal_persec: None,
        }
    }


    /// Sets the value of NumberofTransmitCompletesPersec
    pub fn set_numberof_transmit_completes_persec(&mut self, value: u64) {
        self.numberof_transmit_completes_persec = Some(value);
    }

    /// Gets the value of NumberofTransmitCompletesPersec
    pub fn get_numberof_transmit_completes_persec(&self) -> Option<&u64> {
        self.numberof_transmit_completes_persec.as_ref()
    }

    /// Sets the value of NumberofVMQs
    pub fn set_numberof_vmqs(&mut self, value: u64) {
        self.numberof_vmqs = Some(value);
    }

    /// Gets the value of NumberofVMQs
    pub fn get_numberof_vmqs(&self) -> Option<&u64> {
        self.numberof_vmqs.as_ref()
    }

    /// Sets the value of PacketsfromExternalPersec
    pub fn set_packetsfrom_external_persec(&mut self, value: u64) {
        self.packetsfrom_external_persec = Some(value);
    }

    /// Gets the value of PacketsfromExternalPersec
    pub fn get_packetsfrom_external_persec(&self) -> Option<&u64> {
        self.packetsfrom_external_persec.as_ref()
    }

    /// Sets the value of PacketsfromInternalPersec
    pub fn set_packetsfrom_internal_persec(&mut self, value: u64) {
        self.packetsfrom_internal_persec = Some(value);
    }

    /// Gets the value of PacketsfromInternalPersec
    pub fn get_packetsfrom_internal_persec(&self) -> Option<&u64> {
        self.packetsfrom_internal_persec.as_ref()
    }
}

