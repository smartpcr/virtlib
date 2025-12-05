// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSSerial_HardwareConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSSerial_HardwareConfiguration {
    #[serde(flatten)]
    pub base: MSSerial,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "BaseIOAddress")]
    pub base_ioaddress: Option<u64>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// 
    #[serde(rename = "InterruptType")]
    pub interrupt_type: Option<u32>,

/// 
    #[serde(rename = "IrqAffinityMask")]
    pub irq_affinity_mask: Option<u64>,

/// 
    #[serde(rename = "IrqLevel")]
    pub irq_level: Option<u32>,

/// 
    #[serde(rename = "IrqNumber")]
    pub irq_number: Option<u32>,

/// 
    #[serde(rename = "IrqVector")]
    pub irq_vector: Option<u32>,
}

impl MSSerial_HardwareConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSSerial::new(),
            active: None,
            base_ioaddress: None,
            instance_name: None,
            interrupt_type: None,
            irq_affinity_mask: None,
            irq_level: None,
            irq_number: None,
            irq_vector: None,
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

    /// Sets the value of BaseIOAddress
    pub fn set_base_ioaddress(&mut self, value: u64) {
        self.base_ioaddress = Some(value);
    }

    /// Gets the value of BaseIOAddress
    pub fn get_base_ioaddress(&self) -> Option<&u64> {
        self.base_ioaddress.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

    /// Sets the value of InterruptType
    pub fn set_interrupt_type(&mut self, value: u32) {
        self.interrupt_type = Some(value);
    }

    /// Gets the value of InterruptType
    pub fn get_interrupt_type(&self) -> Option<&u32> {
        self.interrupt_type.as_ref()
    }

    /// Sets the value of IrqAffinityMask
    pub fn set_irq_affinity_mask(&mut self, value: u64) {
        self.irq_affinity_mask = Some(value);
    }

    /// Gets the value of IrqAffinityMask
    pub fn get_irq_affinity_mask(&self) -> Option<&u64> {
        self.irq_affinity_mask.as_ref()
    }

    /// Sets the value of IrqLevel
    pub fn set_irq_level(&mut self, value: u32) {
        self.irq_level = Some(value);
    }

    /// Gets the value of IrqLevel
    pub fn get_irq_level(&self) -> Option<&u32> {
        self.irq_level.as_ref()
    }

    /// Sets the value of IrqNumber
    pub fn set_irq_number(&mut self, value: u32) {
        self.irq_number = Some(value);
    }

    /// Gets the value of IrqNumber
    pub fn get_irq_number(&self) -> Option<&u32> {
        self.irq_number.as_ref()
    }

    /// Sets the value of IrqVector
    pub fn set_irq_vector(&mut self, value: u32) {
        self.irq_vector = Some(value);
    }

    /// Gets the value of IrqVector
    pub fn get_irq_vector(&self) -> Option<&u32> {
        self.irq_vector.as_ref()
    }
}

