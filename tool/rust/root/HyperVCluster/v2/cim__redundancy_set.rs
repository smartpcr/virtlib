// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_RedundancySet struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_RedundancySet {
    #[serde(flatten)]
    pub base: CIM_SystemSpecificCollection,

/// The current load balance algorithm. 
/// Least Blocks, Least IO, and Address Region are used in storage device path redundancy drivers to optimize load balancing by routing requests to a path with the least queued blocks or IO requests, or based on locality of reference. 
/// 'Product Specific' indicates that the algorithm is optimized for a particular type of product. Information about that product SHOULD be provided in an associated CIM_Product instance.
    #[serde(rename = "LoadBalanceAlgorithm")]
    pub load_balance_algorithm: Option<RedundancySet_LoadBalanceAlgorithm>,

/// MaxNumberSupported indicates the largest number of elements that can participate in the RedundancySet. A value of 0 indicates there is no limit on the number of elements.
    #[serde(rename = "MaxNumberSupported")]
    pub max_number_supported: Option<u32>,

/// MinNumberNeeded indicates the smallest number of elements that MUST be operational in order to function. For example, in an N+1 redundancy relationship, the MinNumberNeeded property is set equal to N. In a 'LimitedSparing' environment, this property is meaningless and SHOULD be set to zero.
    #[serde(rename = "MinNumberNeeded")]
    pub min_number_needed: Option<u32>,

/// When LoadBalanceAlgorithm is Other, this property describes the algorithm.
    #[serde(rename = "OtherLoadBalanceAlgorithm")]
    pub other_load_balance_algorithm: Option<String>,

/// When the corresponding array entry in TypeOfSet[] is 'Other', this entry provides a string describing the type of set.
    #[serde(rename = "OtherTypeOfSet")]
    pub other_type_of_set: Vec<String>,

/// RedundancyStatus provides information on the state of the RedundancyGroup. 'Fully Redundant' (value=2) means that all of the configured redundancy is still available; 'Degraded Redundancy' (3) means that some configured elements are degraded, missing or failed but that the number of elements in the set is still greater than the minimum required ('MinNumberNeeded'); 'Redundancy Lost' (4) means that sufficient configured elements are missing or failed that no redundancy is available and the next failure experienced will cause overall failure. 'Overall Failure' (5) means that there has been an overall failure of the RedundancySet.
    #[serde(rename = "RedundancyStatus")]
    pub redundancy_status: Option<RedundancySet_RedundancyStatus>,

/// TypeOfSet provides information on the type of redundancy. N+1 (=2) indicates all members are active, are unaware and function independent of one another. However, there exist at least one extra member to achieve functionality. 'Sparing' is implied (i.e. each member can be a spare for the other(s). An example of N+1 is a system that has 2 power supplies, but needs only 1 power supply to functioning properly. N+1 is a special case of N+M redundancy where M=1. A value of N+1 (=2) shall be used for N+M redundancy. - Load Balanced (=3) indicates all members are active. However, there functionality is not independent of each other. Their functioning is determined by some sort of load balancing algrothim (implemented in hardware and/or software). 'Sparing' is implied (i.e. each member can be a spare for the other(s). 
/// - Sparing (=4) indicates that all members are active and are aware of each others. However, their functionality is independent until failover. Each member can be a spare for the other(s). 
/// - Limited Sparing (=5) indicates that all members are active, and they may or may not be aware of each and they are not spares for each other. Instead, their redundancy is indicated by the IsSpare relationship.
    #[serde(rename = "TypeOfSet")]
    pub type_of_set: Vec<RedundancySet_TypeOfSet>,

/// VendorIdentifyingInfo captures the vendor identifying data for the RedundancySet. One example is the product name for a cluster.
    #[serde(rename = "VendorIdentifyingInfo")]
    pub vendor_identifying_info: Option<String>,
}

impl CIM_RedundancySet {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SystemSpecificCollection::new(),
            load_balance_algorithm: None,
            max_number_supported: None,
            min_number_needed: None,
            other_load_balance_algorithm: None,
            other_type_of_set: Vec::new(),
            redundancy_status: None,
            type_of_set: Vec::new(),
            vendor_identifying_info: None,
        }
    }


    /// Sets the value of LoadBalanceAlgorithm
    pub fn set_load_balance_algorithm(&mut self, value: RedundancySet_LoadBalanceAlgorithm) {
        self.load_balance_algorithm = Some(value);
    }

    /// Gets the value of LoadBalanceAlgorithm
    pub fn get_load_balance_algorithm(&self) -> Option<&RedundancySet_LoadBalanceAlgorithm> {
        self.load_balance_algorithm.as_ref()
    }

    /// Sets the value of MaxNumberSupported
    pub fn set_max_number_supported(&mut self, value: u32) {
        self.max_number_supported = Some(value);
    }

    /// Gets the value of MaxNumberSupported
    pub fn get_max_number_supported(&self) -> Option<&u32> {
        self.max_number_supported.as_ref()
    }

    /// Sets the value of MinNumberNeeded
    pub fn set_min_number_needed(&mut self, value: u32) {
        self.min_number_needed = Some(value);
    }

    /// Gets the value of MinNumberNeeded
    pub fn get_min_number_needed(&self) -> Option<&u32> {
        self.min_number_needed.as_ref()
    }

    /// Sets the value of OtherLoadBalanceAlgorithm
    pub fn set_other_load_balance_algorithm(&mut self, value: String) {
        self.other_load_balance_algorithm = Some(value);
    }

    /// Gets the value of OtherLoadBalanceAlgorithm
    pub fn get_other_load_balance_algorithm(&self) -> Option<&String> {
        self.other_load_balance_algorithm.as_ref()
    }

    /// Sets the value of OtherTypeOfSet
    pub fn set_other_type_of_set(&mut self, value: Vec<String>) {
        self.other_type_of_set = value;
    }

    /// Gets the value of OtherTypeOfSet
    pub fn get_other_type_of_set(&self) -> &Vec<String> {
        &self.other_type_of_set
    }

    /// Sets the value of RedundancyStatus
    pub fn set_redundancy_status(&mut self, value: RedundancySet_RedundancyStatus) {
        self.redundancy_status = Some(value);
    }

    /// Gets the value of RedundancyStatus
    pub fn get_redundancy_status(&self) -> Option<&RedundancySet_RedundancyStatus> {
        self.redundancy_status.as_ref()
    }

    /// Sets the value of TypeOfSet
    pub fn set_type_of_set(&mut self, value: Vec<RedundancySet_TypeOfSet>) {
        self.type_of_set = value;
    }

    /// Gets the value of TypeOfSet
    pub fn get_type_of_set(&self) -> &Vec<RedundancySet_TypeOfSet> {
        &self.type_of_set
    }

    /// Sets the value of VendorIdentifyingInfo
    pub fn set_vendor_identifying_info(&mut self, value: String) {
        self.vendor_identifying_info = Some(value);
    }

    /// Gets the value of VendorIdentifyingInfo
    pub fn get_vendor_identifying_info(&self) -> Option<&String> {
        self.vendor_identifying_info.as_ref()
    }

/// This method forces a failover from one ManagedElement to another. There are two parameters to the Failover method. 
/// - FailoverFrom is a reference to an 'active' ManagedElement that will become inactive after the method. This element SHOULD be part of the RedundancySet via a MemberOfCollection relationship. 
/// - FailoverTo is a reference to the ManagedElement that will take over for the FailoverFrom element. This element SHOULD either be a member of the RedundancySet or be associated with the RedundancySet via an IsSpare relationship. 
/// 
/// Upon sucessful completion: 
/// - the FailoverTo element SHOULD be associated to the RedundancySet via MemberOfCollection. 
/// - the FailFrom element SHOULD either still be associated to the RedundandySet via MemberOfCollection with a OperationalStatus or EnableState that indicates it not active, or it SHOULD be associated to the 'Spared' collection via the MemberOfCollection association.

    /// * `failover_from` - The primary ManagedSystemElement that will become inactive after the method. (CIM_ManagedElement)
    /// * `failover_to` - The ManagedSystemElement that will take over from the primary MSE. (CIM_ManagedElement)

    /// * `return_value` -  (u32)
    pub fn failover(&self, failover_from: CIM_ManagedElement, failover_to: CIM_ManagedElement) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FailoverFrom".to_string(), value: failover_from.into() });
        args.push(MethodParameter { name: "FailoverTo".to_string(), value: failover_to.into() });
        self.invoke_method("Failover", &args)

    }

}

