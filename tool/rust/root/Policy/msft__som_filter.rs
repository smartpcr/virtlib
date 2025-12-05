// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Policy
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_SomFilter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_SomFilter {

/// 
    #[serde(rename = "Author")]
    pub author: Option<String>,

/// 
    #[serde(rename = "ChangeDate")]
    pub change_date: Option<String>,

/// 
    #[serde(rename = "CreationDate")]
    pub creation_date: Option<String>,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "Domain")]
    pub domain: Option<String>,

/// 
    #[serde(rename = "ID")]
    pub id: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Rules")]
    pub rules: Vec<MSFT_Rule>,

/// 
    #[serde(rename = "SourceOrganization")]
    pub source_organization: Option<String>,
}

impl MSFT_SomFilter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            author: None,
            change_date: None,
            creation_date: None,
            description: None,
            domain: None,
            id: None,
            name: None,
            rules: Vec::new(),
            source_organization: None,
        }
    }


    /// Sets the value of Author
    pub fn set_author(&mut self, value: String) {
        self.author = Some(value);
    }

    /// Gets the value of Author
    pub fn get_author(&self) -> Option<&String> {
        self.author.as_ref()
    }

    /// Sets the value of ChangeDate
    pub fn set_change_date(&mut self, value: String) {
        self.change_date = Some(value);
    }

    /// Gets the value of ChangeDate
    pub fn get_change_date(&self) -> Option<&String> {
        self.change_date.as_ref()
    }

    /// Sets the value of CreationDate
    pub fn set_creation_date(&mut self, value: String) {
        self.creation_date = Some(value);
    }

    /// Gets the value of CreationDate
    pub fn get_creation_date(&self) -> Option<&String> {
        self.creation_date.as_ref()
    }

    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of Domain
    pub fn set_domain(&mut self, value: String) {
        self.domain = Some(value);
    }

    /// Gets the value of Domain
    pub fn get_domain(&self) -> Option<&String> {
        self.domain.as_ref()
    }

    /// Sets the value of ID
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of ID
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Rules
    pub fn set_rules(&mut self, value: Vec<MSFT_Rule>) {
        self.rules = value;
    }

    /// Gets the value of Rules
    pub fn get_rules(&self) -> &Vec<MSFT_Rule> {
        &self.rules
    }

    /// Sets the value of SourceOrganization
    pub fn set_source_organization(&mut self, value: String) {
        self.source_organization = Some(value);
    }

    /// Gets the value of SourceOrganization
    pub fn get_source_organization(&self) -> Option<&String> {
        self.source_organization.as_ref()
    }

/// 

    /// * `return_value` -  (u32)
    pub fn evaluate(&self) -> Result<(), WmiError> {
        self.invoke_method("Evaluate", &[])

    }


/// 

    /// * `filters` -  (MSFT_SomFilter[])

    /// * `results` -  (u32[])
    /// * `return_value` -  (u32)
    pub fn batch_evaluate(&self, filters: &Vec<MSFT_SomFilter>, results: &mut Vec<u32>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "filters".to_string(), value: filters.into() });

        let result = self.invoke_method("BatchEvaluate", &args)?;
        let results = result.get_value("results")?;
        Ok(result.return_value)

    }

}

