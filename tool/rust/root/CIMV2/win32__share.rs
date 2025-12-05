// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_Share struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_Share {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "AccessMask")]
    pub access_mask: Option<u32>,

/// 
    #[serde(rename = "AllowMaximum")]
    pub allow_maximum: Option<bool>,

/// 
    #[serde(rename = "MaximumAllowed")]
    pub maximum_allowed: Option<u32>,

/// 
    #[serde(rename = "Path")]
    pub path: Option<String>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<u32>,
}

impl Win32_Share {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            access_mask: None,
            allow_maximum: None,
            maximum_allowed: None,
            path: None,
            type: None,
        }
    }


    /// Sets the value of AccessMask
    pub fn set_access_mask(&mut self, value: u32) {
        self.access_mask = Some(value);
    }

    /// Gets the value of AccessMask
    pub fn get_access_mask(&self) -> Option<&u32> {
        self.access_mask.as_ref()
    }

    /// Sets the value of AllowMaximum
    pub fn set_allow_maximum(&mut self, value: bool) {
        self.allow_maximum = Some(value);
    }

    /// Gets the value of AllowMaximum
    pub fn get_allow_maximum(&self) -> Option<&bool> {
        self.allow_maximum.as_ref()
    }

    /// Sets the value of MaximumAllowed
    pub fn set_maximum_allowed(&mut self, value: u32) {
        self.maximum_allowed = Some(value);
    }

    /// Gets the value of MaximumAllowed
    pub fn get_maximum_allowed(&self) -> Option<&u32> {
        self.maximum_allowed.as_ref()
    }

    /// Sets the value of Path
    pub fn set_path(&mut self, value: String) {
        self.path = Some(value);
    }

    /// Gets the value of Path
    pub fn get_path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    /// Sets the value of Type
    pub fn set_type(&mut self, value: u32) {
        self.type = Some(value);
    }

    /// Gets the value of Type
    pub fn get_type(&self) -> Option<&u32> {
        self.type.as_ref()
    }

/// 

    /// * `access` -  (Win32_SecurityDescriptor)
    /// * `description` -  (String)
    /// * `maximum_allowed` -  (u32)
    /// * `name` -  (String)
    /// * `password` -  (String)
    /// * `path` -  (String)
    /// * `type` -  (u32)

    /// * `return_value` -  (u32)
    pub fn create(&self, path: &String, name: &String, type: u32, maximum_allowed: Option<u32>, description: &Option<String>, password: &Option<String>, access: Option<Win32_SecurityDescriptor>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Path".to_string(), value: path.into() });
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "Type".to_string(), value: type.into() });
        if let Some(val) = maximum_allowed {
            args.push(MethodParameter { name: "MaximumAllowed".to_string(), value: val.into() });
        }
        if let Some(val) = description {
            args.push(MethodParameter { name: "Description".to_string(), value: val.into() });
        }
        if let Some(val) = password {
            args.push(MethodParameter { name: "Password".to_string(), value: val.into() });
        }
        if let Some(val) = access {
            args.push(MethodParameter { name: "Access".to_string(), value: val.into() });
        }
        self.invoke_method("Create", &args)

    }


/// 

    /// * `access` -  (Win32_SecurityDescriptor)
    /// * `description` -  (String)
    /// * `maximum_allowed` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_share_info(&self, maximum_allowed: Option<u32>, description: &Option<String>, access: Option<Win32_SecurityDescriptor>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = maximum_allowed {
            args.push(MethodParameter { name: "MaximumAllowed".to_string(), value: val.into() });
        }
        if let Some(val) = description {
            args.push(MethodParameter { name: "Description".to_string(), value: val.into() });
        }
        if let Some(val) = access {
            args.push(MethodParameter { name: "Access".to_string(), value: val.into() });
        }
        self.invoke_method("SetShareInfo", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn get_access_mask(&self) -> Result<(), WmiError> {
        self.invoke_method("GetAccessMask", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn delete(&self) -> Result<(), WmiError> {
        self.invoke_method("Delete", &[])

    }

}

