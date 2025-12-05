// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapterAdvancedPropertySettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapterAdvancedPropertySettingData {
    #[serde(flatten)]
    pub base: MSFT_NetAdapterSettingData,

/// 
    #[serde(rename = "DefaultDisplayValue")]
    pub default_display_value: Option<String>,

/// 
    #[serde(rename = "DefaultRegistryValue")]
    pub default_registry_value: Option<String>,

/// 
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,

/// 
    #[serde(rename = "DisplayParameterType")]
    pub display_parameter_type: Option<u32>,

/// 
    #[serde(rename = "DisplayValue")]
    pub display_value: Option<String>,

/// 
    #[serde(rename = "NumericParameterBaseValue")]
    pub numeric_parameter_base_value: Option<String>,

/// 
    #[serde(rename = "NumericParameterMaxValue")]
    pub numeric_parameter_max_value: Option<String>,

/// 
    #[serde(rename = "NumericParameterMinValue")]
    pub numeric_parameter_min_value: Option<String>,

/// 
    #[serde(rename = "NumericParameterStepValue")]
    pub numeric_parameter_step_value: Option<String>,

/// 
    #[serde(rename = "Optional")]
    pub optional: Option<bool>,

/// 
    #[serde(rename = "RegistryDataType")]
    pub registry_data_type: Option<u32>,

/// 
    #[serde(rename = "RegistryKeyword")]
    pub registry_keyword: Option<String>,

/// 
    #[serde(rename = "RegistryValue")]
    pub registry_value: Vec<String>,

/// 
    #[serde(rename = "ValidDisplayValues")]
    pub valid_display_values: Vec<String>,

/// 
    #[serde(rename = "ValidRegistryValues")]
    pub valid_registry_values: Vec<String>,
}

impl MSFT_NetAdapterAdvancedPropertySettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetAdapterSettingData::new(),
            default_display_value: None,
            default_registry_value: None,
            display_name: None,
            display_parameter_type: None,
            display_value: None,
            numeric_parameter_base_value: None,
            numeric_parameter_max_value: None,
            numeric_parameter_min_value: None,
            numeric_parameter_step_value: None,
            optional: None,
            registry_data_type: None,
            registry_keyword: None,
            registry_value: Vec::new(),
            valid_display_values: Vec::new(),
            valid_registry_values: Vec::new(),
        }
    }


    /// Sets the value of DefaultDisplayValue
    pub fn set_default_display_value(&mut self, value: String) {
        self.default_display_value = Some(value);
    }

    /// Gets the value of DefaultDisplayValue
    pub fn get_default_display_value(&self) -> Option<&String> {
        self.default_display_value.as_ref()
    }

    /// Sets the value of DefaultRegistryValue
    pub fn set_default_registry_value(&mut self, value: String) {
        self.default_registry_value = Some(value);
    }

    /// Gets the value of DefaultRegistryValue
    pub fn get_default_registry_value(&self) -> Option<&String> {
        self.default_registry_value.as_ref()
    }

    /// Sets the value of DisplayName
    pub fn set_display_name(&mut self, value: String) {
        self.display_name = Some(value);
    }

    /// Gets the value of DisplayName
    pub fn get_display_name(&self) -> Option<&String> {
        self.display_name.as_ref()
    }

    /// Sets the value of DisplayParameterType
    pub fn set_display_parameter_type(&mut self, value: u32) {
        self.display_parameter_type = Some(value);
    }

    /// Gets the value of DisplayParameterType
    pub fn get_display_parameter_type(&self) -> Option<&u32> {
        self.display_parameter_type.as_ref()
    }

    /// Sets the value of DisplayValue
    pub fn set_display_value(&mut self, value: String) {
        self.display_value = Some(value);
    }

    /// Gets the value of DisplayValue
    pub fn get_display_value(&self) -> Option<&String> {
        self.display_value.as_ref()
    }

    /// Sets the value of NumericParameterBaseValue
    pub fn set_numeric_parameter_base_value(&mut self, value: String) {
        self.numeric_parameter_base_value = Some(value);
    }

    /// Gets the value of NumericParameterBaseValue
    pub fn get_numeric_parameter_base_value(&self) -> Option<&String> {
        self.numeric_parameter_base_value.as_ref()
    }

    /// Sets the value of NumericParameterMaxValue
    pub fn set_numeric_parameter_max_value(&mut self, value: String) {
        self.numeric_parameter_max_value = Some(value);
    }

    /// Gets the value of NumericParameterMaxValue
    pub fn get_numeric_parameter_max_value(&self) -> Option<&String> {
        self.numeric_parameter_max_value.as_ref()
    }

    /// Sets the value of NumericParameterMinValue
    pub fn set_numeric_parameter_min_value(&mut self, value: String) {
        self.numeric_parameter_min_value = Some(value);
    }

    /// Gets the value of NumericParameterMinValue
    pub fn get_numeric_parameter_min_value(&self) -> Option<&String> {
        self.numeric_parameter_min_value.as_ref()
    }

    /// Sets the value of NumericParameterStepValue
    pub fn set_numeric_parameter_step_value(&mut self, value: String) {
        self.numeric_parameter_step_value = Some(value);
    }

    /// Gets the value of NumericParameterStepValue
    pub fn get_numeric_parameter_step_value(&self) -> Option<&String> {
        self.numeric_parameter_step_value.as_ref()
    }

    /// Sets the value of Optional
    pub fn set_optional(&mut self, value: bool) {
        self.optional = Some(value);
    }

    /// Gets the value of Optional
    pub fn get_optional(&self) -> Option<&bool> {
        self.optional.as_ref()
    }

    /// Sets the value of RegistryDataType
    pub fn set_registry_data_type(&mut self, value: u32) {
        self.registry_data_type = Some(value);
    }

    /// Gets the value of RegistryDataType
    pub fn get_registry_data_type(&self) -> Option<&u32> {
        self.registry_data_type.as_ref()
    }

    /// Sets the value of RegistryKeyword
    pub fn set_registry_keyword(&mut self, value: String) {
        self.registry_keyword = Some(value);
    }

    /// Gets the value of RegistryKeyword
    pub fn get_registry_keyword(&self) -> Option<&String> {
        self.registry_keyword.as_ref()
    }

    /// Sets the value of RegistryValue
    pub fn set_registry_value(&mut self, value: Vec<String>) {
        self.registry_value = value;
    }

    /// Gets the value of RegistryValue
    pub fn get_registry_value(&self) -> &Vec<String> {
        &self.registry_value
    }

    /// Sets the value of ValidDisplayValues
    pub fn set_valid_display_values(&mut self, value: Vec<String>) {
        self.valid_display_values = value;
    }

    /// Gets the value of ValidDisplayValues
    pub fn get_valid_display_values(&self) -> &Vec<String> {
        &self.valid_display_values
    }

    /// Sets the value of ValidRegistryValues
    pub fn set_valid_registry_values(&mut self, value: Vec<String>) {
        self.valid_registry_values = value;
    }

    /// Gets the value of ValidRegistryValues
    pub fn get_valid_registry_values(&self) -> &Vec<String> {
        &self.valid_registry_values
    }

/// 

    /// * `cmdlet_output` -  (MSFT_NetAdapterAdvancedPropertySettingData)
    /// * `return_value` -  (u32)
    pub fn reset(&self, cmdlet_output: &mut MSFT_NetAdapterAdvancedPropertySettingData) -> Result<(), WmiError> {

        let result = self.invoke_method("Reset", &[])?;
        let cmdlet_output = result.get_value("CmdletOutput")?;
        Ok(result.return_value)

    }

}

