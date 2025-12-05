// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ClusBfltDeviceMethods struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusBfltDeviceMethods {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl ClusBfltDeviceMethods {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active: None,
            instance_name: None,
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

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

/// Gets Device Attributes

    /// * `device_guid` - Device Id (String)

    /// * `attributes` - Attributes (u32)
    pub fn get_device_attributes(&self, device_guid: &String, attributes: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DeviceGuid".to_string(), value: device_guid.into() });

        let result = self.invoke_method("GetDeviceAttributes", &args)?;
        let attributes = result.get_value("Attributes")?;
        Ok(result.return_value)

    }


/// Sets Device Attributes

    /// * `attributes` - Attributes (u32)
    /// * `attributes_mask` - AttributesMask (u32)
    /// * `device_guid` - Device Id (String)
    pub fn set_device_attributes(&self, device_guid: &String, attributes: u32, attributes_mask: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DeviceGuid".to_string(), value: device_guid.into() });
        args.push(MethodParameter { name: "Attributes".to_string(), value: attributes.into() });
        args.push(MethodParameter { name: "AttributesMask".to_string(), value: attributes_mask.into() });
        self.invoke_method("SetDeviceAttributes", &args)

    }


/// Pauses Device IOs

    /// * `device_guid` - Device Id (String)
    /// * `time_ms` - Time (u32)
    pub fn pause_device_ios(&self, device_guid: &String, time_ms: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DeviceGuid".to_string(), value: device_guid.into() });
        args.push(MethodParameter { name: "TimeMs".to_string(), value: time_ms.into() });
        self.invoke_method("PauseDeviceIOs", &args)

    }


/// Resumes Device IOs

    /// * `device_guid` - Device Id (String)
    pub fn resume_device_ios(&self, device_guid: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DeviceGuid".to_string(), value: device_guid.into() });
        self.invoke_method("ResumeDeviceIOs", &args)

    }


/// Refresh Reg Params

    /// * `f_reboot_required` - fRebootRequired (bool)
    pub fn refresh_reg_params(&self, f_reboot_required: &mut bool) -> Result<(), WmiError> {

        let result = self.invoke_method("RefreshRegParams", &[])?;
        let f_reboot_required = result.get_value("fRebootRequired")?;
        Ok(result.return_value)

    }

}

