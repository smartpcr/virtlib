// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ProcessorBiosTStates struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProcessorBiosTStates {
    #[serde(flatten)]
    pub base: MSProcessorClass,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "FadtDutyOffset")]
    pub fadt_duty_offset: Option<u8>,

/// 
    #[serde(rename = "FadtDutyWidth")]
    pub fadt_duty_width: Option<u8>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "NtNumber")]
    pub nt_number: Option<u32>,

/// 
    #[serde(rename = "Ptc")]
    pub ptc: Option<AcpiControlStatus>,

/// 
    #[serde(rename = "Reserved1")]
    pub reserved1: Option<u32>,

/// 
    #[serde(rename = "Reserved2")]
    pub reserved2: Option<u32>,

/// 
    #[serde(rename = "Reserved3")]
    pub reserved3: Option<u64>,

/// 
    #[serde(rename = "Tpc")]
    pub tpc: Option<u32>,

/// 
    #[serde(rename = "Tss")]
    pub tss: Option<ProcessorAcpiTss>,

/// 
    #[serde(rename = "TStateVersionInUse")]
    pub tstate_version_in_use: Option<u32>,
}

impl ProcessorBiosTStates {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSProcessorClass::new(),
            active: None,
            fadt_duty_offset: None,
            fadt_duty_width: None,
            instance_name: None,
            nt_number: None,
            ptc: None,
            reserved1: None,
            reserved2: None,
            reserved3: None,
            tpc: None,
            tss: None,
            tstate_version_in_use: None,
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

    /// Sets the value of FadtDutyOffset
    pub fn set_fadt_duty_offset(&mut self, value: u8) {
        self.fadt_duty_offset = Some(value);
    }

    /// Gets the value of FadtDutyOffset
    pub fn get_fadt_duty_offset(&self) -> Option<&u8> {
        self.fadt_duty_offset.as_ref()
    }

    /// Sets the value of FadtDutyWidth
    pub fn set_fadt_duty_width(&mut self, value: u8) {
        self.fadt_duty_width = Some(value);
    }

    /// Gets the value of FadtDutyWidth
    pub fn get_fadt_duty_width(&self) -> Option<&u8> {
        self.fadt_duty_width.as_ref()
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

    /// Sets the value of Ptc
    pub fn set_ptc(&mut self, value: AcpiControlStatus) {
        self.ptc = Some(value);
    }

    /// Gets the value of Ptc
    pub fn get_ptc(&self) -> Option<&AcpiControlStatus> {
        self.ptc.as_ref()
    }

    /// Sets the value of Reserved1
    pub fn set_reserved1(&mut self, value: u32) {
        self.reserved1 = Some(value);
    }

    /// Gets the value of Reserved1
    pub fn get_reserved1(&self) -> Option<&u32> {
        self.reserved1.as_ref()
    }

    /// Sets the value of Reserved2
    pub fn set_reserved2(&mut self, value: u32) {
        self.reserved2 = Some(value);
    }

    /// Gets the value of Reserved2
    pub fn get_reserved2(&self) -> Option<&u32> {
        self.reserved2.as_ref()
    }

    /// Sets the value of Reserved3
    pub fn set_reserved3(&mut self, value: u64) {
        self.reserved3 = Some(value);
    }

    /// Gets the value of Reserved3
    pub fn get_reserved3(&self) -> Option<&u64> {
        self.reserved3.as_ref()
    }

    /// Sets the value of Tpc
    pub fn set_tpc(&mut self, value: u32) {
        self.tpc = Some(value);
    }

    /// Gets the value of Tpc
    pub fn get_tpc(&self) -> Option<&u32> {
        self.tpc.as_ref()
    }

    /// Sets the value of Tss
    pub fn set_tss(&mut self, value: ProcessorAcpiTss) {
        self.tss = Some(value);
    }

    /// Gets the value of Tss
    pub fn get_tss(&self) -> Option<&ProcessorAcpiTss> {
        self.tss.as_ref()
    }

    /// Sets the value of TStateVersionInUse
    pub fn set_tstate_version_in_use(&mut self, value: u32) {
        self.tstate_version_in_use = Some(value);
    }

    /// Gets the value of TStateVersionInUse
    pub fn get_tstate_version_in_use(&self) -> Option<&u32> {
        self.tstate_version_in_use.as_ref()
    }
}

