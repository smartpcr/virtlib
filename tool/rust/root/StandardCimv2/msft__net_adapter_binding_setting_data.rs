// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterBindingSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterBindingSettingData {
    #[serde(flatten)]
    pub base: MSFT_NetAdapterSettingData,

/// 
    #[serde(rename = "BindName")]
    pub bind_name: Option<String>,

/// 
    #[serde(rename = "Characteristics")]
    pub characteristics: Option<u32>,

/// 
    #[serde(rename = "ComponentClassGuid")]
    pub component_class_guid: Option<String>,

/// 
    #[serde(rename = "ComponentClassName")]
    pub component_class_name: Option<String>,

/// 
    #[serde(rename = "ComponentID")]
    pub component_id: Option<String>,

/// 
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,

/// 
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,
}

impl MSFT_NetAdapterBindingSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetAdapterSettingData::new(),
            bind_name: None,
            characteristics: None,
            component_class_guid: None,
            component_class_name: None,
            component_id: None,
            display_name: None,
            enabled: None,
        }
    }


    /// Sets the value of BindName
    pub fn set_bind_name(&mut self, value: String) {
        self.bind_name = Some(value);
    }

    /// Gets the value of BindName
    pub fn get_bind_name(&self) -> Option<&String> {
        self.bind_name.as_ref()
    }

    /// Sets the value of Characteristics
    pub fn set_characteristics(&mut self, value: u32) {
        self.characteristics = Some(value);
    }

    /// Gets the value of Characteristics
    pub fn get_characteristics(&self) -> Option<&u32> {
        self.characteristics.as_ref()
    }

    /// Sets the value of ComponentClassGuid
    pub fn set_component_class_guid(&mut self, value: String) {
        self.component_class_guid = Some(value);
    }

    /// Gets the value of ComponentClassGuid
    pub fn get_component_class_guid(&self) -> Option<&String> {
        self.component_class_guid.as_ref()
    }

    /// Sets the value of ComponentClassName
    pub fn set_component_class_name(&mut self, value: String) {
        self.component_class_name = Some(value);
    }

    /// Gets the value of ComponentClassName
    pub fn get_component_class_name(&self) -> Option<&String> {
        self.component_class_name.as_ref()
    }

    /// Sets the value of ComponentID
    pub fn set_component_id(&mut self, value: String) {
        self.component_id = Some(value);
    }

    /// Gets the value of ComponentID
    pub fn get_component_id(&self) -> Option<&String> {
        self.component_id.as_ref()
    }

    /// Sets the value of DisplayName
    pub fn set_display_name(&mut self, value: String) {
        self.display_name = Some(value);
    }

    /// Gets the value of DisplayName
    pub fn get_display_name(&self) -> Option<&String> {
        self.display_name.as_ref()
    }

    /// Sets the value of Enabled
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = Some(value);
    }

    /// Gets the value of Enabled
    pub fn get_enabled(&self) -> Option<&bool> {
        self.enabled.as_ref()
    }

/// 

    /// * `cmdlet_output` -  (MSFT_NetAdapterBindingSettingData)
    /// * `return_value` -  (u32)
    pub fn enable(&self, cmdlet_output: &mut MSFT_NetAdapterBindingSettingData) -> Result<(), WmiError> {

        let result = self.invoke_method("Enable", &[])?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `cmdlet_output` -  (MSFT_NetAdapterBindingSettingData)
    /// * `return_value` -  (u32)
    pub fn disable(&self, cmdlet_output: &mut MSFT_NetAdapterBindingSettingData) -> Result<(), WmiError> {

        let result = self.invoke_method("Disable", &[])?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

