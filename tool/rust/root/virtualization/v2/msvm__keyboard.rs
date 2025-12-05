// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_Keyboard struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_Keyboard {
    #[serde(flatten)]
    pub base: CIM_UserDevice,

/// 
    #[serde(rename = "Layout")]
    pub layout: Option<String>,

/// 
    #[serde(rename = "NumberOfFunctionKeys")]
    pub number_of_function_keys: Option<u16>,

/// 
    #[serde(rename = "Password")]
    pub password: Option<u16>,

/// 
    #[serde(rename = "UnicodeSupported")]
    pub unicode_supported: Option<bool>,
}

impl Msvm_Keyboard {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_UserDevice::new(),
            layout: None,
            number_of_function_keys: None,
            password: None,
            unicode_supported: None,
        }
    }


    /// Sets the value of Layout
    pub fn set_layout(&mut self, value: String) {
        self.layout = Some(value);
    }

    /// Gets the value of Layout
    pub fn get_layout(&self) -> Option<&String> {
        self.layout.as_ref()
    }

    /// Sets the value of NumberOfFunctionKeys
    pub fn set_number_of_function_keys(&mut self, value: u16) {
        self.number_of_function_keys = Some(value);
    }

    /// Gets the value of NumberOfFunctionKeys
    pub fn get_number_of_function_keys(&self) -> Option<&u16> {
        self.number_of_function_keys.as_ref()
    }

    /// Sets the value of Password
    pub fn set_password(&mut self, value: u16) {
        self.password = Some(value);
    }

    /// Gets the value of Password
    pub fn get_password(&self) -> Option<&u16> {
        self.password.as_ref()
    }

    /// Sets the value of UnicodeSupported
    pub fn set_unicode_supported(&mut self, value: bool) {
        self.unicode_supported = Some(value);
    }

    /// Gets the value of UnicodeSupported
    pub fn get_unicode_supported(&self) -> Option<&bool> {
        self.unicode_supported.as_ref()
    }

/// 

    /// * `key_code` -  (u32)

    /// * `return_value` -  (u32)
    pub fn press_key(&self, key_code: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "KeyCode".to_string(), value: key_code.into() });
        self.invoke_method("PressKey", &args)

    }


/// 

    /// * `key_code` -  (u32)

    /// * `return_value` -  (u32)
    pub fn release_key(&self, key_code: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "KeyCode".to_string(), value: key_code.into() });
        self.invoke_method("ReleaseKey", &args)

    }


/// 

    /// * `key_code` -  (u32)

    /// * `return_value` -  (u32)
    pub fn type_key(&self, key_code: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "KeyCode".to_string(), value: key_code.into() });
        self.invoke_method("TypeKey", &args)

    }


/// 

    /// * `key_code` -  (u32)

    /// * `key_state` -  (bool)
    /// * `return_value` -  (u32)
    pub fn is_key_pressed(&self, key_code: u32, key_state: &mut bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "KeyCode".to_string(), value: key_code.into() });

        let result = self.invoke_method("IsKeyPressed", &args)?;
        let key_state = result.get_value("KeyState")?;
        Ok(result.return_value)

    }


/// 

    /// * `ascii_text` -  (String)

    /// * `return_value` -  (u32)
    pub fn type_text(&self, ascii_text: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AsciiText".to_string(), value: ascii_text.into() });
        self.invoke_method("TypeText", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn type_ctrl_alt_del(&self) -> Result<(), WmiError> {
        self.invoke_method("TypeCtrlAltDel", &[])

    }


/// 

    /// * `scancodes` -  (u8[])

    /// * `return_value` -  (u32)
    pub fn type_scancodes(&self, scancodes: &Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Scancodes".to_string(), value: scancodes.into() });
        self.invoke_method("TypeScancodes", &args)

    }

}

