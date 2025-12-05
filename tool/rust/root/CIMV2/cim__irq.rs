// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_IRQ struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_IRQ {
    #[serde(flatten)]
    pub base: CIM_SystemResource,

/// 
    #[serde(rename = "Availability")]
    pub availability: Option<u16>,

/// 
    #[serde(rename = "CreationClassName")]
    pub creation_class_name: Option<String>,

/// 
    #[serde(rename = "CSCreationClassName")]
    pub cscreation_class_name: Option<String>,

/// 
    #[serde(rename = "CSName")]
    pub csname: Option<String>,

/// 
    #[serde(rename = "IRQNumber")]
    pub irqnumber: Option<u32>,

/// 
    #[serde(rename = "Shareable")]
    pub shareable: Option<bool>,

/// 
    #[serde(rename = "TriggerLevel")]
    pub trigger_level: Option<u16>,

/// 
    #[serde(rename = "TriggerType")]
    pub trigger_type: Option<u16>,
}

impl CIM_IRQ {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SystemResource::new(),
            availability: None,
            creation_class_name: None,
            cscreation_class_name: None,
            csname: None,
            irqnumber: None,
            shareable: None,
            trigger_level: None,
            trigger_type: None,
        }
    }


    /// Sets the value of Availability
    pub fn set_availability(&mut self, value: u16) {
        self.availability = Some(value);
    }

    /// Gets the value of Availability
    pub fn get_availability(&self) -> Option<&u16> {
        self.availability.as_ref()
    }

    /// Sets the value of CreationClassName
    pub fn set_creation_class_name(&mut self, value: String) {
        self.creation_class_name = Some(value);
    }

    /// Gets the value of CreationClassName
    pub fn get_creation_class_name(&self) -> Option<&String> {
        self.creation_class_name.as_ref()
    }

    /// Sets the value of CSCreationClassName
    pub fn set_cscreation_class_name(&mut self, value: String) {
        self.cscreation_class_name = Some(value);
    }

    /// Gets the value of CSCreationClassName
    pub fn get_cscreation_class_name(&self) -> Option<&String> {
        self.cscreation_class_name.as_ref()
    }

    /// Sets the value of CSName
    pub fn set_csname(&mut self, value: String) {
        self.csname = Some(value);
    }

    /// Gets the value of CSName
    pub fn get_csname(&self) -> Option<&String> {
        self.csname.as_ref()
    }

    /// Sets the value of IRQNumber
    pub fn set_irqnumber(&mut self, value: u32) {
        self.irqnumber = Some(value);
    }

    /// Gets the value of IRQNumber
    pub fn get_irqnumber(&self) -> Option<&u32> {
        self.irqnumber.as_ref()
    }

    /// Sets the value of Shareable
    pub fn set_shareable(&mut self, value: bool) {
        self.shareable = Some(value);
    }

    /// Gets the value of Shareable
    pub fn get_shareable(&self) -> Option<&bool> {
        self.shareable.as_ref()
    }

    /// Sets the value of TriggerLevel
    pub fn set_trigger_level(&mut self, value: u16) {
        self.trigger_level = Some(value);
    }

    /// Gets the value of TriggerLevel
    pub fn get_trigger_level(&self) -> Option<&u16> {
        self.trigger_level.as_ref()
    }

    /// Sets the value of TriggerType
    pub fn set_trigger_type(&mut self, value: u16) {
        self.trigger_type = Some(value);
    }

    /// Gets the value of TriggerType
    pub fn get_trigger_type(&self) -> Option<&u16> {
        self.trigger_type.as_ref()
    }
}

