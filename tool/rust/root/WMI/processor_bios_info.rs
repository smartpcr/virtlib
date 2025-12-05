// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ProcessorBiosInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProcessorBiosInfo {
    #[serde(flatten)]
    pub base: MSProcessorClass,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "ApicId")]
    pub apic_id: Option<u32>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "NtNumber")]
    pub nt_number: Option<u32>,

/// 
    #[serde(rename = "PBlk")]
    pub pblk: Option<u32>,

/// 
    #[serde(rename = "PBlkLen")]
    pub pblk_len: Option<u32>,

/// 
    #[serde(rename = "Pct")]
    pub pct: Option<AcpiPct>,

/// 
    #[serde(rename = "ProcessorId")]
    pub processor_id: Option<u32>,

/// 
    #[serde(rename = "Pss")]
    pub pss: Option<AcpiPss>,
}

impl ProcessorBiosInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSProcessorClass::new(),
            active: None,
            apic_id: None,
            instance_name: None,
            nt_number: None,
            pblk: None,
            pblk_len: None,
            pct: None,
            processor_id: None,
            pss: None,
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

    /// Sets the value of ApicId
    pub fn set_apic_id(&mut self, value: u32) {
        self.apic_id = Some(value);
    }

    /// Gets the value of ApicId
    pub fn get_apic_id(&self) -> Option<&u32> {
        self.apic_id.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of NtNumber
    pub fn set_nt_number(&mut self, value: u32) {
        self.nt_number = Some(value);
    }

    /// Gets the value of NtNumber
    pub fn get_nt_number(&self) -> Option<&u32> {
        self.nt_number.as_ref()
    }

    /// Sets the value of PBlk
    pub fn set_pblk(&mut self, value: u32) {
        self.pblk = Some(value);
    }

    /// Gets the value of PBlk
    pub fn get_pblk(&self) -> Option<&u32> {
        self.pblk.as_ref()
    }

    /// Sets the value of PBlkLen
    pub fn set_pblk_len(&mut self, value: u32) {
        self.pblk_len = Some(value);
    }

    /// Gets the value of PBlkLen
    pub fn get_pblk_len(&self) -> Option<&u32> {
        self.pblk_len.as_ref()
    }

    /// Sets the value of Pct
    pub fn set_pct(&mut self, value: AcpiPct) {
        self.pct = Some(value);
    }

    /// Gets the value of Pct
    pub fn get_pct(&self) -> Option<&AcpiPct> {
        self.pct.as_ref()
    }

    /// Sets the value of ProcessorId
    pub fn set_processor_id(&mut self, value: u32) {
        self.processor_id = Some(value);
    }

    /// Gets the value of ProcessorId
    pub fn get_processor_id(&self) -> Option<&u32> {
        self.processor_id.as_ref()
    }

    /// Sets the value of Pss
    pub fn set_pss(&mut self, value: AcpiPss) {
        self.pss = Some(value);
    }

    /// Gets the value of Pss
    pub fn get_pss(&self) -> Option<&AcpiPss> {
        self.pss.as_ref()
    }
}

