// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Cluster.Scaleout
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ClusterSetAvailabilitySet struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ClusterSetAvailabilitySet {

/// 
    #[serde(rename = "AvailabilitySetName")]
    pub availability_set_name: Option<String>,

/// 
    #[serde(rename = "DomainAssignments")]
    pub domain_assignments: Vec<String>,

/// 
    #[serde(rename = "FaultDomains")]
    pub fault_domains: Option<u32>,

/// 
    #[serde(rename = "FaultDomainType")]
    pub fault_domain_type: Option<u32>,

/// 
    #[serde(rename = "Id")]
    pub id: Option<u64>,

/// 
    #[serde(rename = "ParticipantIds")]
    pub participant_ids: Vec<u64>,

/// 
    #[serde(rename = "SoftEnforcement")]
    pub soft_enforcement: Option<bool>,

/// 
    #[serde(rename = "UpdateDomains")]
    pub update_domains: Option<u32>,

/// 
    #[serde(rename = "Workloads")]
    pub workloads: Vec<u64>,
}

impl MSFT_ClusterSetAvailabilitySet {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            availability_set_name: None,
            domain_assignments: Vec::new(),
            fault_domains: None,
            fault_domain_type: None,
            id: None,
            participant_ids: Vec::new(),
            soft_enforcement: None,
            update_domains: None,
            workloads: Vec::new(),
        }
    }


    /// Sets the value of AvailabilitySetName
    pub fn set_availability_set_name(&mut self, value: String) {
        self.availability_set_name = Some(value);
    }

    /// Gets the value of AvailabilitySetName
    pub fn get_availability_set_name(&self) -> Option<&String> {
        self.availability_set_name.as_ref()
    }

    /// Sets the value of DomainAssignments
    pub fn set_domain_assignments(&mut self, value: Vec<String>) {
        self.domain_assignments = value;
    }

    /// Gets the value of DomainAssignments
    pub fn get_domain_assignments(&self) -> &Vec<String> {
        &self.domain_assignments
    }

    /// Sets the value of FaultDomains
    pub fn set_fault_domains(&mut self, value: u32) {
        self.fault_domains = Some(value);
    }

    /// Gets the value of FaultDomains
    pub fn get_fault_domains(&self) -> Option<&u32> {
        self.fault_domains.as_ref()
    }

    /// Sets the value of FaultDomainType
    pub fn set_fault_domain_type(&mut self, value: u32) {
        self.fault_domain_type = Some(value);
    }

    /// Gets the value of FaultDomainType
    pub fn get_fault_domain_type(&self) -> Option<&u32> {
        self.fault_domain_type.as_ref()
    }

    /// Sets the value of Id
    pub fn set_id(&mut self, value: u64) {
        self.id = Some(value);
    }

    /// Gets the value of Id
    pub fn get_id(&self) -> Option<&u64> {
        self.id.as_ref()
    }

    /// Sets the value of ParticipantIds
    pub fn set_participant_ids(&mut self, value: Vec<u64>) {
        self.participant_ids = value;
    }

    /// Gets the value of ParticipantIds
    pub fn get_participant_ids(&self) -> &Vec<u64> {
        &self.participant_ids
    }

    /// Sets the value of SoftEnforcement
    pub fn set_soft_enforcement(&mut self, value: bool) {
        self.soft_enforcement = Some(value);
    }

    /// Gets the value of SoftEnforcement
    pub fn get_soft_enforcement(&self) -> Option<&bool> {
        self.soft_enforcement.as_ref()
    }

    /// Sets the value of UpdateDomains
    pub fn set_update_domains(&mut self, value: u32) {
        self.update_domains = Some(value);
    }

    /// Gets the value of UpdateDomains
    pub fn get_update_domains(&self) -> Option<&u32> {
        self.update_domains.as_ref()
    }

    /// Sets the value of Workloads
    pub fn set_workloads(&mut self, value: Vec<u64>) {
        self.workloads = value;
    }

    /// Gets the value of Workloads
    pub fn get_workloads(&self) -> &Vec<u64> {
        &self.workloads
    }

/// 

    /// * `availability_set_name` -  (String)
    /// * `fault_domains` -  (u32)
    /// * `fdtype` -  (u32)
    /// * `flags` -  (u32)
    /// * `participant_name` -  (String[])
    /// * `soft_enforcement` -  (bool)
    /// * `update_domains` -  (u32)

    /// * `created_availability_set` -  (MSFT_ClusterSetAvailabilitySet)
    /// * `return_value` -  (u32)
    pub fn create_availability_set(&self, availability_set_name: &String, fault_domains: u32, update_domains: u32, fdtype: u32, participant_name: &Vec<String>, soft_enforcement: bool, flags: u32, created_availability_set: &mut MSFT_ClusterSetAvailabilitySet) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "AvailabilitySetName".to_string(), value: availability_set_name.into() });
        args.push(MethodParameter { name: "FaultDomains".to_string(), value: fault_domains.into() });
        args.push(MethodParameter { name: "UpdateDomains".to_string(), value: update_domains.into() });
        args.push(MethodParameter { name: "FDType".to_string(), value: fdtype.into() });
        args.push(MethodParameter { name: "ParticipantName".to_string(), value: participant_name.into() });
        args.push(MethodParameter { name: "SoftEnforcement".to_string(), value: soft_enforcement.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });

        let result = self.invoke_method("CreateAvailabilitySet", &args)?;
        let created_availability_set = result.get_value("CreatedAvailabilitySet")?;
        Ok(result.return_value)

    }


/// 

    /// * `flags` -  (u32)

    /// * `return_value` -  (u32)
    pub fn remove_availability_set(&self, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("RemoveAvailabilitySet", &args)

    }


/// 

    /// * `flags` -  (u32)
    /// * `participant_name` -  (String[])

    /// * `return_value` -  (u32)
    pub fn add_participant(&self, participant_name: &Vec<String>, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ParticipantName".to_string(), value: participant_name.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("AddParticipant", &args)

    }


/// 

    /// * `flags` -  (u32)
    /// * `participant_name` -  (String[])

    /// * `return_value` -  (u32)
    pub fn remove_participant(&self, participant_name: &Vec<String>, flags: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ParticipantName".to_string(), value: participant_name.into() });
        args.push(MethodParameter { name: "Flags".to_string(), value: flags.into() });
        self.invoke_method("RemoveParticipant", &args)

    }

}

