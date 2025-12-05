// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_USBDevice struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_USBDevice {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,

/// 
    #[serde(rename = "ClassCode")]
    pub class_code: Option<u8>,

/// 
    #[serde(rename = "CurrentAlternateSettings")]
    pub current_alternate_settings: Vec<u8>,

/// 
    #[serde(rename = "CurrentConfigValue")]
    pub current_config_value: Option<u8>,

/// 
    #[serde(rename = "NumberOfConfigs")]
    pub number_of_configs: Option<u8>,

/// 
    #[serde(rename = "ProtocolCode")]
    pub protocol_code: Option<u8>,

/// 
    #[serde(rename = "SubclassCode")]
    pub subclass_code: Option<u8>,

/// 
    #[serde(rename = "USBVersion")]
    pub usbversion: Option<u16>,
}

impl CIM_USBDevice {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
            class_code: None,
            current_alternate_settings: Vec::new(),
            current_config_value: None,
            number_of_configs: None,
            protocol_code: None,
            subclass_code: None,
            usbversion: None,
        }
    }


    /// Sets the value of ClassCode
    pub fn set_class_code(&mut self, value: u8) {
        self.class_code = Some(value);
    }

    /// Gets the value of ClassCode
    pub fn get_class_code(&self) -> Option<&u8> {
        self.class_code.as_ref()
    }

    /// Sets the value of CurrentAlternateSettings
    pub fn set_current_alternate_settings(&mut self, value: Vec<u8>) {
        self.current_alternate_settings = value;
    }

    /// Gets the value of CurrentAlternateSettings
    pub fn get_current_alternate_settings(&self) -> &Vec<u8> {
        &self.current_alternate_settings
    }

    /// Sets the value of CurrentConfigValue
    pub fn set_current_config_value(&mut self, value: u8) {
        self.current_config_value = Some(value);
    }

    /// Gets the value of CurrentConfigValue
    pub fn get_current_config_value(&self) -> Option<&u8> {
        self.current_config_value.as_ref()
    }

    /// Sets the value of NumberOfConfigs
    pub fn set_number_of_configs(&mut self, value: u8) {
        self.number_of_configs = Some(value);
    }

    /// Gets the value of NumberOfConfigs
    pub fn get_number_of_configs(&self) -> Option<&u8> {
        self.number_of_configs.as_ref()
    }

    /// Sets the value of ProtocolCode
    pub fn set_protocol_code(&mut self, value: u8) {
        self.protocol_code = Some(value);
    }

    /// Gets the value of ProtocolCode
    pub fn get_protocol_code(&self) -> Option<&u8> {
        self.protocol_code.as_ref()
    }

    /// Sets the value of SubclassCode
    pub fn set_subclass_code(&mut self, value: u8) {
        self.subclass_code = Some(value);
    }

    /// Gets the value of SubclassCode
    pub fn get_subclass_code(&self) -> Option<&u8> {
        self.subclass_code.as_ref()
    }

    /// Sets the value of USBVersion
    pub fn set_usbversion(&mut self, value: u16) {
        self.usbversion = Some(value);
    }

    /// Gets the value of USBVersion
    pub fn get_usbversion(&self) -> Option<&u16> {
        self.usbversion.as_ref()
    }

/// 

    /// * `request_index` -  (u16)
    /// * `request_length` -  (u16)
    /// * `request_type` -  (u8)
    /// * `request_value` -  (u16)

    /// * `buffer` -  (u8[])
    /// * `request_length` -  (u16)
    /// * `return_value` -  (u32)
    pub fn get_descriptor(&self, request_type: u8, request_value: u16, request_index: u16, request_length: &mut u16, buffer: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RequestType".to_string(), value: request_type.into() });
        args.push(MethodParameter { name: "RequestValue".to_string(), value: request_value.into() });
        args.push(MethodParameter { name: "RequestIndex".to_string(), value: request_index.into() });

        let result = self.invoke_method("GetDescriptor", &args)?;
        let buffer = result.get_value("Buffer")?;
        let request_length = result.get_value("RequestLength")?;
        Ok(result.return_value)

    }

}

