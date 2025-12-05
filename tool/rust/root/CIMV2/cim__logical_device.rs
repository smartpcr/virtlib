// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_LogicalDevice struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_LogicalDevice {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "Availability")]
    pub availability: Option<u16>,

/// 
    #[serde(rename = "ConfigManagerErrorCode")]
    pub config_manager_error_code: Option<u32>,

/// 
    #[serde(rename = "ConfigManagerUserConfig")]
    pub config_manager_user_config: Option<bool>,

/// 
    #[serde(rename = "CreationClassName")]
    pub creation_class_name: Option<String>,

/// 
    #[serde(rename = "DeviceID")]
    pub device_id: Option<String>,

/// 
    #[serde(rename = "ErrorCleared")]
    pub error_cleared: Option<bool>,

/// 
    #[serde(rename = "ErrorDescription")]
    pub error_description: Option<String>,

/// 
    #[serde(rename = "LastErrorCode")]
    pub last_error_code: Option<u32>,

/// 
    #[serde(rename = "PNPDeviceID")]
    pub pnpdevice_id: Option<String>,

/// 
    #[serde(rename = "PowerManagementCapabilities")]
    pub power_management_capabilities: Vec<u16>,

/// 
    #[serde(rename = "PowerManagementSupported")]
    pub power_management_supported: Option<bool>,

/// 
    #[serde(rename = "StatusInfo")]
    pub status_info: Option<u16>,

/// 
    #[serde(rename = "SystemCreationClassName")]
    pub system_creation_class_name: Option<String>,

/// 
    #[serde(rename = "SystemName")]
    pub system_name: Option<String>,
}

impl CIM_LogicalDevice {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            availability: None,
            config_manager_error_code: None,
            config_manager_user_config: None,
            creation_class_name: None,
            device_id: None,
            error_cleared: None,
            error_description: None,
            last_error_code: None,
            pnpdevice_id: None,
            power_management_capabilities: Vec::new(),
            power_management_supported: None,
            status_info: None,
            system_creation_class_name: None,
            system_name: None,
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

    /// Sets the value of ConfigManagerErrorCode
    pub fn set_config_manager_error_code(&mut self, value: u32) {
        self.config_manager_error_code = Some(value);
    }

    /// Gets the value of ConfigManagerErrorCode
    pub fn get_config_manager_error_code(&self) -> Option<&u32> {
        self.config_manager_error_code.as_ref()
    }

    /// Sets the value of ConfigManagerUserConfig
    pub fn set_config_manager_user_config(&mut self, value: bool) {
        self.config_manager_user_config = Some(value);
    }

    /// Gets the value of ConfigManagerUserConfig
    pub fn get_config_manager_user_config(&self) -> Option<&bool> {
        self.config_manager_user_config.as_ref()
    }

    /// Sets the value of CreationClassName
    pub fn set_creation_class_name(&mut self, value: String) {
        self.creation_class_name = Some(value);
    }

    /// Gets the value of CreationClassName
    pub fn get_creation_class_name(&self) -> Option<&String> {
        self.creation_class_name.as_ref()
    }

    /// Sets the value of DeviceID
    pub fn set_device_id(&mut self, value: String) {
        self.device_id = Some(value);
    }

    /// Gets the value of DeviceID
    pub fn get_device_id(&self) -> Option<&String> {
        self.device_id.as_ref()
    }

    /// Sets the value of ErrorCleared
    pub fn set_error_cleared(&mut self, value: bool) {
        self.error_cleared = Some(value);
    }

    /// Gets the value of ErrorCleared
    pub fn get_error_cleared(&self) -> Option<&bool> {
        self.error_cleared.as_ref()
    }

    /// Sets the value of ErrorDescription
    pub fn set_error_description(&mut self, value: String) {
        self.error_description = Some(value);
    }

    /// Gets the value of ErrorDescription
    pub fn get_error_description(&self) -> Option<&String> {
        self.error_description.as_ref()
    }

    /// Sets the value of LastErrorCode
    pub fn set_last_error_code(&mut self, value: u32) {
        self.last_error_code = Some(value);
    }

    /// Gets the value of LastErrorCode
    pub fn get_last_error_code(&self) -> Option<&u32> {
        self.last_error_code.as_ref()
    }

    /// Sets the value of PNPDeviceID
    pub fn set_pnpdevice_id(&mut self, value: String) {
        self.pnpdevice_id = Some(value);
    }

    /// Gets the value of PNPDeviceID
    pub fn get_pnpdevice_id(&self) -> Option<&String> {
        self.pnpdevice_id.as_ref()
    }

    /// Sets the value of PowerManagementCapabilities
    pub fn set_power_management_capabilities(&mut self, value: Vec<u16>) {
        self.power_management_capabilities = value;
    }

    /// Gets the value of PowerManagementCapabilities
    pub fn get_power_management_capabilities(&self) -> &Vec<u16> {
        &self.power_management_capabilities
    }

    /// Sets the value of PowerManagementSupported
    pub fn set_power_management_supported(&mut self, value: bool) {
        self.power_management_supported = Some(value);
    }

    /// Gets the value of PowerManagementSupported
    pub fn get_power_management_supported(&self) -> Option<&bool> {
        self.power_management_supported.as_ref()
    }

    /// Sets the value of StatusInfo
    pub fn set_status_info(&mut self, value: u16) {
        self.status_info = Some(value);
    }

    /// Gets the value of StatusInfo
    pub fn get_status_info(&self) -> Option<&u16> {
        self.status_info.as_ref()
    }

    /// Sets the value of SystemCreationClassName
    pub fn set_system_creation_class_name(&mut self, value: String) {
        self.system_creation_class_name = Some(value);
    }

    /// Gets the value of SystemCreationClassName
    pub fn get_system_creation_class_name(&self) -> Option<&String> {
        self.system_creation_class_name.as_ref()
    }

    /// Sets the value of SystemName
    pub fn set_system_name(&mut self, value: String) {
        self.system_name = Some(value);
    }

    /// Gets the value of SystemName
    pub fn get_system_name(&self) -> Option<&String> {
        self.system_name.as_ref()
    }

/// 

    /// * `power_state` -  (u16)
    /// * `time` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_power_state(&self, power_state: u16, time: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PowerState".to_string(), value: power_state.into() });
        args.push(MethodParameter { name: "Time".to_string(), value: time.into() });
        self.invoke_method("SetPowerState", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn reset(&self) -> Result<(), WmiError> {
        self.invoke_method("Reset", &[])

    }

}

