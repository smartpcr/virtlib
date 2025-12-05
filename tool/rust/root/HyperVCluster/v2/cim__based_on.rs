// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_BasedOn struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_BasedOn {
    #[serde(flatten)]
    pub base: CIM_Dependency,

/// EndingAddress indicates where in lower level storage, the higher level Extent ends. This property is useful when mapping non-contiguous Extents into a higher level grouping.
    #[serde(rename = "EndingAddress")]
    pub ending_address: Option<u64>,

/// If there is an order to the BasedOn associations that describe how a higher level StorageExtent is assembled, the OrderIndex property indicates this. When an order exists, the instances of BasedOn with the same Dependent value (i.e., the same higher level Extent) should place unique values in the OrderIndex property. The lowest value implies the first member of the collection of lower level Extents, and increasing values imply successive members of the collection. If there is no ordered relationship, a value of zero should be specified. An example of the use of this property is to define a RAID-0 striped array of 3 disks. The resultant RAID array is a StorageExtent that is dependent on (BasedOn) the StorageExtents that describe each of the 3 disks. The OrderIndex of each BasedOn association from the disk Extents to the RAID array could be specified as 1, 2 and 3 to indicate the order in which the disk Extents are used to access the RAID data.
    #[serde(rename = "OrderIndex")]
    pub order_index: Option<u16>,

/// StartingAddress indicates where in lower level storage, the higher level Extent begins.
    #[serde(rename = "StartingAddress")]
    pub starting_address: Option<u64>,
}

impl CIM_BasedOn {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Dependency::new(),
            ending_address: None,
            order_index: None,
            starting_address: None,
        }
    }


    /// Sets the value of EndingAddress
    pub fn set_ending_address(&mut self, value: u64) {
        self.ending_address = Some(value);
    }

    /// Gets the value of EndingAddress
    pub fn get_ending_address(&self) -> Option<&u64> {
        self.ending_address.as_ref()
    }

    /// Sets the value of OrderIndex
    pub fn set_order_index(&mut self, value: u16) {
        self.order_index = Some(value);
    }

    /// Gets the value of OrderIndex
    pub fn get_order_index(&self) -> Option<&u16> {
        self.order_index.as_ref()
    }

    /// Sets the value of StartingAddress
    pub fn set_starting_address(&mut self, value: u64) {
        self.starting_address = Some(value);
    }

    /// Gets the value of StartingAddress
    pub fn get_starting_address(&self) -> Option<&u64> {
        self.starting_address.as_ref()
    }
}

