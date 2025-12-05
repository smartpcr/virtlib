// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PnPEntity struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PnPEntity {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,

/// 
    #[serde(rename = "ClassGuid")]
    pub class_guid: Option<String>,

/// 
    #[serde(rename = "CompatibleID")]
    pub compatible_id: Vec<String>,

/// 
    #[serde(rename = "HardwareID")]
    pub hardware_id: Vec<String>,

/// 
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// 
    #[serde(rename = "PNPClass")]
    pub pnpclass: Option<String>,

/// 
    #[serde(rename = "Present")]
    pub present: Option<bool>,

/// 
    #[serde(rename = "Service")]
    pub service: Option<String>,
}

impl Win32_PnPEntity {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
            class_guid: None,
            compatible_id: Vec::new(),
            hardware_id: Vec::new(),
            manufacturer: None,
            pnpclass: None,
            present: None,
            service: None,
        }
    }


    /// Sets the value of ClassGuid
    pub fn set_class_guid(&mut self, value: String) {
        self.class_guid = Some(value);
    }

    /// Gets the value of ClassGuid
    pub fn get_class_guid(&self) -> Option<&String> {
        self.class_guid.as_ref()
    }

    /// Sets the value of CompatibleID
    pub fn set_compatible_id(&mut self, value: Vec<String>) {
        self.compatible_id = value;
    }

    /// Gets the value of CompatibleID
    pub fn get_compatible_id(&self) -> &Vec<String> {
        &self.compatible_id
    }

    /// Sets the value of HardwareID
    pub fn set_hardware_id(&mut self, value: Vec<String>) {
        self.hardware_id = value;
    }

    /// Gets the value of HardwareID
    pub fn get_hardware_id(&self) -> &Vec<String> {
        &self.hardware_id
    }

    /// Sets the value of Manufacturer
    pub fn set_manufacturer(&mut self, value: String) {
        self.manufacturer = Some(value);
    }

    /// Gets the value of Manufacturer
    pub fn get_manufacturer(&self) -> Option<&String> {
        self.manufacturer.as_ref()
    }

    /// Sets the value of PNPClass
    pub fn set_pnpclass(&mut self, value: String) {
        self.pnpclass = Some(value);
    }

    /// Gets the value of PNPClass
    pub fn get_pnpclass(&self) -> Option<&String> {
        self.pnpclass.as_ref()
    }

    /// Sets the value of Present
    pub fn set_present(&mut self, value: bool) {
        self.present = Some(value);
    }

    /// Gets the value of Present
    pub fn get_present(&self) -> Option<&bool> {
        self.present.as_ref()
    }

    /// Sets the value of Service
    pub fn set_service(&mut self, value: String) {
        self.service = Some(value);
    }

    /// Gets the value of Service
    pub fn get_service(&self) -> Option<&String> {
        self.service.as_ref()
    }

/// 

    /// * `reboot_needed` -  (bool)
    /// * `return_value` -  (u32)
    pub fn enable(&self, reboot_needed: &mut bool) -> Result<(), WmiError> {

        let result = self.invoke_method("Enable", &[])?;
        let reboot_needed = result.get_value("rebootNeeded")?;
        Ok(result.return_value)

    }


/// 

    /// * `reboot_needed` -  (bool)
    /// * `return_value` -  (u32)
    pub fn disable(&self, reboot_needed: &mut bool) -> Result<(), WmiError> {

        let result = self.invoke_method("Disable", &[])?;
        let reboot_needed = result.get_value("rebootNeeded")?;
        Ok(result.return_value)

    }


/// 

    /// * `device_property_keys` -  (String[])

    /// * `device_properties` -  (Win32_PnPDeviceProperty[])
    /// * `return_value` -  (u32)
    pub fn get_device_properties(&self, device_property_keys: &Option<Vec<String>>, device_properties: &mut Vec<Win32_PnPDeviceProperty>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = device_property_keys {
            args.push(MethodParameter { name: "devicePropertyKeys".to_string(), value: val.into() });
        }

        let result = self.invoke_method("GetDeviceProperties", &args)?;
        let device_properties = result.get_value("deviceProperties")?;
        Ok(result.return_value)

    }

}

