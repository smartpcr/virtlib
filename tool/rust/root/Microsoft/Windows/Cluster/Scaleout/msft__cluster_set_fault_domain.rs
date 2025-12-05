// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Cluster.Scaleout
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ClusterSetFaultDomain struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ClusterSetFaultDomain {

/// 
    #[serde(rename = "ClusterName")]
    pub cluster_name: Vec<String>,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "FaultDomainType")]
    pub fault_domain_type: Option<u32>,

/// 
    #[serde(rename = "FDName")]
    pub fdname: Option<String>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<u64>,
}

impl MSFT_ClusterSetFaultDomain {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            cluster_name: Vec::new(),
            description: None,
            fault_domain_type: None,
            fdname: None,
            id: None,
        }
    }


    /// Sets the value of ClusterName
    pub fn set_cluster_name(&mut self, value: Vec<String>) {
        self.cluster_name = value;
    }

    /// Gets the value of ClusterName
    pub fn get_cluster_name(&self) -> &Vec<String> {
        &self.cluster_name
    }

    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of FaultDomainType
    pub fn set_fault_domain_type(&mut self, value: u32) {
        self.fault_domain_type = Some(value);
    }

    /// Gets the value of FaultDomainType
    pub fn get_fault_domain_type(&self) -> Option<&u32> {
        self.fault_domain_type.as_ref()
    }

    /// Sets the value of FDName
    pub fn set_fdname(&mut self, value: String) {
        self.fdname = Some(value);
    }

    /// Gets the value of FDName
    pub fn get_fdname(&self) -> Option<&String> {
        self.fdname.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: u64) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&u64> {
        self.id.as_ref()
    }

/// 

    /// * `cluster_name` -  (String[])
    /// * `description` -  (String)
    /// * `fdname` -  (String)
    /// * `fdtype` -  (u32)
    /// * `flags` -  (u32)

    /// * `created_fault_domain` -  (MSFT_ClusterSetFaultDomain)
    /// * `return_value` -  (u32)
    pub fn create_fault_domain(&self, fdname: &String, cluster_name: &Vec<String>, fdtype: u32, description: &String, flags: u32, created_fault_domain: &mut MSFT_ClusterSetFaultDomain) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FDName".to_string(), value: fdname.into() });
        args.push(MethodParameter { name: "ClusterName".to_string(), value: cluster_name.into() });
        args.push(MethodParameter { name: "FDType".to_string(), value: fdtype.into() });
        args.push(MethodParameter { name: "Description".to_string(), value: description.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });

        let result = self.invoke_method("CreateFaultDomain", &args)?;
        let created_fault_domain = result.get_value("CreatedFaultDomain")?;
        Ok(result.return_value)

    }


/// 

    /// * `flags` -  (u32)

    /// * `return_value` -  (u32)
    pub fn remove_fault_domain(&self, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("RemoveFaultDomain", &args)

    }


/// 

    /// * `cluster_name` -  (String[])
    /// * `flags` -  (u32)

    /// * `return_value` -  (u32)
    pub fn add_members(&self, cluster_name: &Vec<String>, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ClusterName".to_string(), value: cluster_name.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("AddMembers", &args)

    }


/// 

    /// * `cluster_name` -  (String[])
    /// * `flags` -  (u32)

    /// * `return_value` -  (u32)
    pub fn remove_members(&self, cluster_name: &Vec<String>, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ClusterName".to_string(), value: cluster_name.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("RemoveMembers", &args)

    }

}

