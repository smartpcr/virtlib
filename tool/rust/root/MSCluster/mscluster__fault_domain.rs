// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_FaultDomain struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_FaultDomain {

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<String>,

/// 
    #[serde(rename = "Location")]
    pub location: Option<String>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "Type")]
    pub type: Option<u32>,
}

impl MSCluster_FaultDomain {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            description: None,
            id: None,
            location: None,
            name: None,
            type: None,
        }
    }


    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of Location
    pub fn set_location(&mut self, value: String) {
        self.location = Some(value);
    }

    /// Gets the value of Location
    pub fn get_location(&self) -> Option<&String> {
        self.location.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
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

    /// * `description` -  (String)
    /// * `fault_domain` -  (String)
    /// * `fault_domain_type` -  (u32)
    /// * `flags` -  (u32)
    /// * `location` -  (String)
    /// * `name` -  (String)

    /// * `created_fault_domain` -  (MSCluster_FaultDomain)
    /// * `return_value` -  (u32)
    pub fn create_fault_domain(&self, name: &String, fault_domain: &String, fault_domain_type: u32, description: &String, location: &String, flags: u32, created_fault_domain: &mut MSCluster_FaultDomain) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "FaultDomain".to_string(), value: fault_domain.into() });
        args.push(MethodParameter { name: "FaultDomainType".to_string(), value: fault_domain_type.into() });
        args.push(MethodParameter { name: "Description".to_string(), value: description.into() });
        args.push(MethodParameter { name: "Location".to_string(), value: location.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });

        let result = self.invoke_method("CreateFaultDomain", &args)?;
        let created_fault_domain = result.get_value("CreatedFaultDomain")?;
        Ok(result.return_value)

    }


/// 

    /// * `flags` -  (u32)
    /// * `xml` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_fault_domain_xml(&self, xml: &String, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "XML".to_string(), value: xml.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("SetFaultDomainXML", &args)

    }


/// 

    /// * `flags` -  (u32)

    /// * `return_value` -  (u32)
    /// * `xml` -  (String)
    pub fn get_fault_domain_xml(&self, xml: &mut String, flags: Option<u32>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = flags {
            args.push(MethodParameter { name: "Flags".to_string(), value: val.into() });
        }

        let result = self.invoke_method("GetFaultDomainXML", &args)?;
        let xml = result.get_value("XML")?;
        Ok(result.return_value)

    }


/// 

    /// * `fault_domain` -  (String)
    /// * `flags` -  (u32)

    /// * `return_value` -  (u32)
    pub fn move_fault_domain(&self, fault_domain: &String, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FaultDomain".to_string(), value: fault_domain.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("MoveFaultDomain", &args)

    }


/// 

    /// * `flags` -  (u32)

    /// * `children` -  (MSCluster_FaultDomain[])
    /// * `return_value` -  (u32)
    pub fn get_children(&self, children: &mut Vec<MSCluster_FaultDomain>, flags: Option<u32>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = flags {
            args.push(MethodParameter { name: "Flags".to_string(), value: val.into() });
        }

        let result = self.invoke_method("GetChildren", &args)?;
        let children = result.get_value("Children")?;
        Ok(result.return_value)

    }


/// 

    /// * `flags` -  (u32)

    /// * `return_value` -  (u32)
    /// * `storage_nodes` -  (MSCluster_StorageNode[])
    pub fn get_storage_nodes(&self, storage_nodes: &mut Vec<MSCluster_StorageNode>, flags: Option<u32>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = flags {
            args.push(MethodParameter { name: "Flags".to_string(), value: val.into() });
        }

        let result = self.invoke_method("GetStorageNodes", &args)?;
        let storage_nodes = result.get_value("StorageNodes")?;
        Ok(result.return_value)

    }


/// 

    /// * `flags` -  (u32)

    /// * `parent` -  (MSCluster_FaultDomain)
    /// * `return_value` -  (u32)
    pub fn get_parent(&self, parent: &mut MSCluster_FaultDomain, flags: Option<u32>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = flags {
            args.push(MethodParameter { name: "Flags".to_string(), value: val.into() });
        }

        let result = self.invoke_method("GetParent", &args)?;
        let parent = result.get_value("Parent")?;
        Ok(result.return_value)

    }


/// 

    /// * `description` -  (String)
    /// * `fault_domain` -  (String)
    /// * `flags` -  (u32)
    /// * `location` -  (String)
    /// * `new_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_fault_domain(&self, new_name: &String, fault_domain: &String, description: &String, location: &String, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NewName".to_string(), value: new_name.into() });
        args.push(MethodParameter { name: "FaultDomain".to_string(), value: fault_domain.into() });
        args.push(MethodParameter { name: "Description".to_string(), value: description.into() });
        args.push(MethodParameter { name: "Location".to_string(), value: location.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("SetFaultDomain", &args)

    }


/// 

    /// * `flags` -  (u32)

    /// * `return_value` -  (u32)
    pub fn remove_fault_domain(&self, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("RemoveFaultDomain", &args)

    }

}

