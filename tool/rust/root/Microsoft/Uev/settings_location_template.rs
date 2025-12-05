// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Uev
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SettingsLocationTemplate struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SettingsLocationTemplate {

/// Flag indicating whether the template defers to MS account.
    #[serde(rename = "DeferToMSAccount")]
    pub defer_to_msaccount: Option<bool>,

/// Flag indicating whether the template is enabled for the current user.
    #[serde(rename = "Enabled")]
    pub enabled: Option<bool>,

/// Enable state of the settings location template.
    #[serde(rename = "EnableStateLocation")]
    pub enable_state_location: Option<String>,

/// Flag indicating whether the template is a suite parent.
    #[serde(rename = "IsSuiteParent")]
    pub is_suite_parent: Option<bool>,

/// Flag indicating whether the template represents a template file.
    #[serde(rename = "IsTemplateFile")]
    pub is_template_file: Option<bool>,

/// ID of the suite parent template.
    #[serde(rename = "SuiteParentId")]
    pub suite_parent_id: Option<String>,

/// Description of the settings location template.
    #[serde(rename = "TemplateDescription")]
    pub template_description: Option<String>,

/// Unique ID of the settings location template.
    #[serde(rename = "TemplateId")]
    pub template_id: Option<String>,

/// Friendly name of the settings location template.
    #[serde(rename = "TemplateName")]
    pub template_name: Option<String>,

/// Profile the template is associated with.
    #[serde(rename = "TemplateProfile")]
    pub template_profile: Option<String>,

/// Type of the settings location template (OS, Application).
    #[serde(rename = "TemplateType")]
    pub template_type: Option<String>,

/// Version of the settings location template.
    #[serde(rename = "TemplateVersion")]
    pub template_version: Option<u32>,
}

impl SettingsLocationTemplate {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            defer_to_msaccount: None,
            enabled: None,
            enable_state_location: None,
            is_suite_parent: None,
            is_template_file: None,
            suite_parent_id: None,
            template_description: None,
            template_id: None,
            template_name: None,
            template_profile: None,
            template_type: None,
            template_version: None,
        }
    }


    /// Sets the value of DeferToMSAccount
    pub fn set_defer_to_msaccount(&mut self, value: bool) {
        self.defer_to_msaccount = Some(value);
    }

    /// Gets the value of DeferToMSAccount
    pub fn get_defer_to_msaccount(&self) -> Option<&bool> {
        self.defer_to_msaccount.as_ref()
    }

    /// Sets the value of Enabled
    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = Some(value);
    }

    /// Gets the value of Enabled
    pub fn get_enabled(&self) -> Option<&bool> {
        self.enabled.as_ref()
    }

    /// Sets the value of EnableStateLocation
    pub fn set_enable_state_location(&mut self, value: String) {
        self.enable_state_location = Some(value);
    }

    /// Gets the value of EnableStateLocation
    pub fn get_enable_state_location(&self) -> Option<&String> {
        self.enable_state_location.as_ref()
    }

    /// Sets the value of IsSuiteParent
    pub fn set_is_suite_parent(&mut self, value: bool) {
        self.is_suite_parent = Some(value);
    }

    /// Gets the value of IsSuiteParent
    pub fn get_is_suite_parent(&self) -> Option<&bool> {
        self.is_suite_parent.as_ref()
    }

    /// Sets the value of IsTemplateFile
    pub fn set_is_template_file(&mut self, value: bool) {
        self.is_template_file = Some(value);
    }

    /// Gets the value of IsTemplateFile
    pub fn get_is_template_file(&self) -> Option<&bool> {
        self.is_template_file.as_ref()
    }

    /// Sets the value of SuiteParentId
    pub fn set_suite_parent_id(&mut self, value: String) {
        self.suite_parent_id = Some(value);
    }

    /// Gets the value of SuiteParentId
    pub fn get_suite_parent_id(&self) -> Option<&String> {
        self.suite_parent_id.as_ref()
    }

    /// Sets the value of TemplateDescription
    pub fn set_template_description(&mut self, value: String) {
        self.template_description = Some(value);
    }

    /// Gets the value of TemplateDescription
    pub fn get_template_description(&self) -> Option<&String> {
        self.template_description.as_ref()
    }

    /// Sets the value of TemplateId
    pub fn set_template_id(&mut self, value: String) {
        self.template_id = Some(value);
    }

    /// Gets the value of TemplateId
    pub fn get_template_id(&self) -> Option<&String> {
        self.template_id.as_ref()
    }

    /// Sets the value of TemplateName
    pub fn set_template_name(&mut self, value: String) {
        self.template_name = Some(value);
    }

    /// Gets the value of TemplateName
    pub fn get_template_name(&self) -> Option<&String> {
        self.template_name.as_ref()
    }

    /// Sets the value of TemplateProfile
    pub fn set_template_profile(&mut self, value: String) {
        self.template_profile = Some(value);
    }

    /// Gets the value of TemplateProfile
    pub fn get_template_profile(&self) -> Option<&String> {
        self.template_profile.as_ref()
    }

    /// Sets the value of TemplateType
    pub fn set_template_type(&mut self, value: String) {
        self.template_type = Some(value);
    }

    /// Gets the value of TemplateType
    pub fn get_template_type(&self) -> Option<&String> {
        self.template_type.as_ref()
    }

    /// Sets the value of TemplateVersion
    pub fn set_template_version(&mut self, value: u32) {
        self.template_version = Some(value);
    }

    /// Gets the value of TemplateVersion
    pub fn get_template_version(&self) -> Option<&u32> {
        self.template_version.as_ref()
    }

/// Get the content of the template file of the current instance.

    /// * `return_value` -  (String)
    pub fn get_content(&self) -> Result<(), WmiError> {
        self.invoke_method("GetContent", &[])

    }


/// Get the content of a setting location template file by template ID.

    /// * `template_id` - Unique ID of the settings location template (String)

    /// * `return_value` -  (String)
    pub fn get_content_by_template_id(&self, template_id: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TemplateId".to_string(), value: template_id.into() });
        self.invoke_method("GetContentByTemplateId", &args)

    }


/// Register a settings location template.

    /// * `absolute_path_to_template` - Absolute path to the settings location template file (String)
    pub fn register(&self, absolute_path_to_template: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AbsolutePathToTemplate".to_string(), value: absolute_path_to_template.into() });
        self.invoke_method("Register", &args)

    }


/// Unregister the current settings location template.
    pub fn unregister(&self) -> Result<(), WmiError> {
        self.invoke_method("Unregister", &[])

    }


/// Unregister all settings location templates.
    pub fn unregister_all(&self) -> Result<(), WmiError> {
        self.invoke_method("UnregisterAll", &[])

    }


/// Unregister a settings location template by template ID.

    /// * `template_id` - Unique ID of the settings location template (String)
    pub fn unregister_by_template_id(&self, template_id: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TemplateId".to_string(), value: template_id.into() });
        self.invoke_method("UnregisterByTemplateId", &args)

    }


/// Validate a settings location template.

    /// * `absolute_path_to_template` - Absolute path to the settings location template file (String)

    /// * `return_value` -  (String)
    pub fn validate(&self, absolute_path_to_template: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AbsolutePathToTemplate".to_string(), value: absolute_path_to_template.into() });
        self.invoke_method("Validate", &args)

    }


/// Update a settings location template by template file.

    /// * `absolute_path_to_template` - Absolute path to the settings location template file (String)
    pub fn update(&self, absolute_path_to_template: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AbsolutePathToTemplate".to_string(), value: absolute_path_to_template.into() });
        self.invoke_method("Update", &args)

    }


/// Rebuild the template index from the existing registered template files.
    pub fn rebuild_index(&self) -> Result<(), WmiError> {
        self.invoke_method("RebuildIndex", &[])

    }


/// Enable the current settings location template.
    pub fn enable(&self) -> Result<(), WmiError> {
        self.invoke_method("Enable", &[])

    }


/// Enable a settings location template.

    /// * `template_id` - Unique ID of the settings location template (String)
    pub fn enable_by_template_id(&self, template_id: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TemplateId".to_string(), value: template_id.into() });
        self.invoke_method("EnableByTemplateId", &args)

    }


/// Disable the current settings location template.
    pub fn disable(&self) -> Result<(), WmiError> {
        self.invoke_method("Disable", &[])

    }


/// Disable a settings location template.

    /// * `template_id` - Unique ID of the settings location template (String)
    pub fn disable_by_template_id(&self, template_id: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TemplateId".to_string(), value: template_id.into() });
        self.invoke_method("DisableByTemplateId", &args)

    }


/// Get the information about the processes monitored by the current settings location template.

    /// * `return_value` -  (String)
    pub fn get_process_info(&self) -> Result<(), WmiError> {
        self.invoke_method("GetProcessInfo", &[])

    }


/// Get the information about the processes monitored by the given settings location template.

    /// * `template_id` - Unique ID of the settings location template (String)

    /// * `return_value` -  (String)
    pub fn get_process_info_by_template_id(&self, template_id: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TemplateId".to_string(), value: template_id.into() });
        self.invoke_method("GetProcessInfoByTemplateId", &args)

    }


/// Get the schema used by the current settings location template.

    /// * `return_value` -  (String)
    pub fn get_schema(&self) -> Result<(), WmiError> {
        self.invoke_method("GetSchema", &[])

    }


/// Get the schema used by a settings location template.

    /// * `template_id` - Unique ID of the settings location template (String)

    /// * `return_value` -  (String)
    pub fn get_schema_by_template_id(&self, template_id: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TemplateId".to_string(), value: template_id.into() });
        self.invoke_method("GetSchemaByTemplateId", &args)

    }


/// Associate a template with a profile

    /// * `profile` - The profile (String)
    /// * `template_id` - The template ID (String)
    pub fn set_template_profile_by_template_id(&self, template_id: &String, profile: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TemplateId".to_string(), value: template_id.into() });
        args.push(MethodParameter { name: "Profile".to_string(), value: profile.into() });
        self.invoke_method("SetTemplateProfileByTemplateId", &args)

    }

}

