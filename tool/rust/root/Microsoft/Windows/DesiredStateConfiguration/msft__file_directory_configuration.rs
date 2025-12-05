// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DesiredStateConfiguration
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_FileDirectoryConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_FileDirectoryConfiguration {
    #[serde(flatten)]
    pub base: OMI_BaseResource,

/// 
    #[serde(rename = "Attributes")]
    pub attributes: Vec<String>,

/// 
    #[serde(rename = "Checksum")]
    pub checksum: Option<String>,

/// 
    #[serde(rename = "Contents")]
    pub contents: Option<String>,

/// 
    #[serde(rename = "CreatedDate")]
    pub created_date: Option<String>,

/// 
    #[serde(rename = "Credential")]
    pub credential: Option<MSFT_Credential>,

/// 
    #[serde(rename = "DestinationPath")]
    pub destination_path: Option<String>,

/// 
    #[serde(rename = "Ensure")]
    pub ensure: Option<String>,

/// 
    #[serde(rename = "Force")]
    pub force: Option<bool>,

/// 
    #[serde(rename = "MatchSource")]
    pub match_source: Option<bool>,

/// 
    #[serde(rename = "ModifiedDate")]
    pub modified_date: Option<String>,

/// 
    #[serde(rename = "Recurse")]
    pub recurse: Option<bool>,

/// 
    #[serde(rename = "Size")]
    pub size: Option<u64>,

/// 
    #[serde(rename = "SourcePath")]
    pub source_path: Option<String>,

/// 
    #[serde(rename = "SubItems")]
    pub sub_items: Vec<MSFT_FileDirectoryConfiguration>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<String>,
}

impl MSFT_FileDirectoryConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: OMI_BaseResource::new(),
            attributes: Vec::new(),
            checksum: None,
            contents: None,
            created_date: None,
            credential: None,
            destination_path: None,
            ensure: None,
            force: None,
            match_source: None,
            modified_date: None,
            recurse: None,
            size: None,
            source_path: None,
            sub_items: Vec::new(),
            type: None,
        }
    }


    /// Sets the value of Attributes
    pub fn set_attributes(&mut self, value: Vec<String>) {
        self.attributes = value;
    }

    /// Gets the value of Attributes
    pub fn get_attributes(&self) -> &Vec<String> {
        &self.attributes
    }

    /// Sets the value of Checksum
    pub fn set_checksum(&mut self, value: String) {
        self.checksum = Some(value);
    }

    /// Gets the value of Checksum
    pub fn get_checksum(&self) -> Option<&String> {
        self.checksum.as_ref()
    }

    /// Sets the value of Contents
    pub fn set_contents(&mut self, value: String) {
        self.contents = Some(value);
    }

    /// Gets the value of Contents
    pub fn get_contents(&self) -> Option<&String> {
        self.contents.as_ref()
    }

    /// Sets the value of CreatedDate
    pub fn set_created_date(&mut self, value: String) {
        self.created_date = Some(value);
    }

    /// Gets the value of CreatedDate
    pub fn get_created_date(&self) -> Option<&String> {
        self.created_date.as_ref()
    }

    /// Sets the value of Credential
    pub fn set_credential(&mut self, value: MSFT_Credential) {
        self.credential = Some(value);
    }

    /// Gets the value of Credential
    pub fn get_credential(&self) -> Option<&MSFT_Credential> {
        self.credential.as_ref()
    }

    /// Sets the value of DestinationPath
    pub fn set_destination_path(&mut self, value: String) {
        self.destination_path = Some(value);
    }

    /// Gets the value of DestinationPath
    pub fn get_destination_path(&self) -> Option<&String> {
        self.destination_path.as_ref()
    }

    /// Sets the value of Ensure
    pub fn set_ensure(&mut self, value: String) {
        self.ensure = Some(value);
    }

    /// Gets the value of Ensure
    pub fn get_ensure(&self) -> Option<&String> {
        self.ensure.as_ref()
    }

    /// Sets the value of Force
    pub fn set_force(&mut self, value: bool) {
        self.force = Some(value);
    }

    /// Gets the value of Force
    pub fn get_force(&self) -> Option<&bool> {
        self.force.as_ref()
    }

    /// Sets the value of MatchSource
    pub fn set_match_source(&mut self, value: bool) {
        self.match_source = Some(value);
    }

    /// Gets the value of MatchSource
    pub fn get_match_source(&self) -> Option<&bool> {
        self.match_source.as_ref()
    }

    /// Sets the value of ModifiedDate
    pub fn set_modified_date(&mut self, value: String) {
        self.modified_date = Some(value);
    }

    /// Gets the value of ModifiedDate
    pub fn get_modified_date(&self) -> Option<&String> {
        self.modified_date.as_ref()
    }

    /// Sets the value of Recurse
    pub fn set_recurse(&mut self, value: bool) {
        self.recurse = Some(value);
    }

    /// Gets the value of Recurse
    pub fn get_recurse(&self) -> Option<&bool> {
        self.recurse.as_ref()
    }

    /// Sets the value of Size
    pub fn set_size(&mut self, value: u64) {
        self.size = Some(value);
    }

    /// Gets the value of Size
    pub fn get_size(&self) -> Option<&u64> {
        self.size.as_ref()
    }

    /// Sets the value of SourcePath
    pub fn set_source_path(&mut self, value: String) {
        self.source_path = Some(value);
    }

    /// Gets the value of SourcePath
    pub fn get_source_path(&self) -> Option<&String> {
        self.source_path.as_ref()
    }

    /// Sets the value of SubItems
    pub fn set_sub_items(&mut self, value: Vec<MSFT_FileDirectoryConfiguration>) {
        self.sub_items = value;
    }

    /// Gets the value of SubItems
    pub fn get_sub_items(&self) -> &Vec<MSFT_FileDirectoryConfiguration> {
        &self.sub_items
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: String) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&String> {
        self.type.as_ref()
    }

/// 

    /// * `flags` -  (u32)
    /// * `input_resource` -  (MSFT_FileDirectoryConfiguration)

    /// * `output_resource` -  (MSFT_FileDirectoryConfiguration)
    /// * `return_value` -  (u32)
    pub fn get_target_resource(&self, input_resource: MSFT_FileDirectoryConfiguration, flags: u32, output_resource: &mut MSFT_FileDirectoryConfiguration) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InputResource".to_string(), value: input_resource.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });

        let result = self.invoke_method("GetTargetResource", &args)?;
        let output_resource = result.get_value("OutputResource")?;
        Ok(result.return_value)

    }


/// 

    /// * `flags` -  (u32)
    /// * `input_resource` -  (MSFT_FileDirectoryConfiguration)

    /// * `provider_context` -  (u64)
    /// * `result` -  (bool)
    /// * `return_value` -  (u32)
    pub fn test_target_resource(&self, input_resource: MSFT_FileDirectoryConfiguration, flags: u32, result: &mut bool, provider_context: &mut u64) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InputResource".to_string(), value: input_resource.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });

        let result = self.invoke_method("TestTargetResource", &args)?;
        let provider_context = result.get_value("ProviderContext")?;
        let result = result.get_value("Result")?;
        Ok(result.return_value)

    }


/// 

    /// * `flags` -  (u32)
    /// * `input_resource` -  (MSFT_FileDirectoryConfiguration)
    /// * `provider_context` -  (u64)

    /// * `return_value` -  (u32)
    pub fn set_target_resource(&self, input_resource: MSFT_FileDirectoryConfiguration, provider_context: u64, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "InputResource".to_string(), value: input_resource.into() });
        args.push(MethodParameter { name: "ProviderContext".to_string(), value: provider_context.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("SetTargetResource", &args)

    }

}

