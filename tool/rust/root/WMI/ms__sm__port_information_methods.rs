// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MS_SM_PortInformationMethods struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MS_SM_PortInformationMethods {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl MS_SM_PortInformationMethods {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active: None,
            instance_name: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

/// 

    /// * `port_index` -  (u32)

    /// * `hbastatus` -  (u32)
    /// * `port_type` -  (u32)
    pub fn sm__get_port_type(&self, port_index: u32, hbastatus: &mut u32, port_type: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PortIndex".to_string(), value: port_index.into() });

        let result = self.invoke_method("SM_GetPortType", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        let port_type = result.get_value("PortType")?;
        Ok(result.return_value)

    }


/// 

    /// * `port_index` -  (u32)
    /// * `port_specific_attributes_max_size` -  (u32)

    /// * `hbastatus` -  (u32)
    /// * `port_attributes` -  (MS_SMHBA_PORTATTRIBUTES)
    pub fn sm__get_adapter_port_attributes(&self, port_index: u32, port_specific_attributes_max_size: u32, hbastatus: &mut u32, port_attributes: &mut MS_SMHBA_PORTATTRIBUTES) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PortIndex".to_string(), value: port_index.into() });
        args.push(MethodParameter { name: "PortSpecificAttributesMaxSize".to_string(), value: port_specific_attributes_max_size.into() });

        let result = self.invoke_method("SM_GetAdapterPortAttributes", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        let port_attributes = result.get_value("PortAttributes")?;
        Ok(result.return_value)

    }


/// 

    /// * `discovered_port_index` -  (u32)
    /// * `port_index` -  (u32)
    /// * `port_specific_attributes_max_size` -  (u32)

    /// * `hbastatus` -  (u32)
    /// * `port_attributes` -  (MS_SMHBA_PORTATTRIBUTES)
    pub fn sm__get_discovered_port_attributes(&self, port_index: u32, discovered_port_index: u32, port_specific_attributes_max_size: u32, hbastatus: &mut u32, port_attributes: &mut MS_SMHBA_PORTATTRIBUTES) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PortIndex".to_string(), value: port_index.into() });
        args.push(MethodParameter { name: "DiscoveredPortIndex".to_string(), value: discovered_port_index.into() });
        args.push(MethodParameter { name: "PortSpecificAttributesMaxSize".to_string(), value: port_specific_attributes_max_size.into() });

        let result = self.invoke_method("SM_GetDiscoveredPortAttributes", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        let port_attributes = result.get_value("PortAttributes")?;
        Ok(result.return_value)

    }


/// 

    /// * `domain_port_wwn` -  (u8[])
    /// * `port_specific_attributes_max_size` -  (u32)
    /// * `port_wwn` -  (u8[])

    /// * `hbastatus` -  (u32)
    /// * `port_attributes` -  (MS_SMHBA_PORTATTRIBUTES)
    pub fn sm__get_port_attributes_by_wwn(&self, port_wwn: &Vec<u8>, domain_port_wwn: &Vec<u8>, port_specific_attributes_max_size: u32, hbastatus: &mut u32, port_attributes: &mut MS_SMHBA_PORTATTRIBUTES) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PortWWN".to_string(), value: port_wwn.into() });
        args.push(MethodParameter { name: "DomainPortWWN".to_string(), value: domain_port_wwn.into() });
        args.push(MethodParameter { name: "PortSpecificAttributesMaxSize".to_string(), value: port_specific_attributes_max_size.into() });

        let result = self.invoke_method("SM_GetPortAttributesByWWN", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        let port_attributes = result.get_value("PortAttributes")?;
        Ok(result.return_value)

    }


/// 

    /// * `port_index` -  (u32)
    /// * `protocol_type` -  (u32)

    /// * `hbastatus` -  (u32)
    /// * `protocol_statistics` -  (MS_SMHBA_PROTOCOLSTATISTICS)
    pub fn sm__get_protocol_statistics(&self, port_index: u32, protocol_type: u32, hbastatus: &mut u32, protocol_statistics: &mut MS_SMHBA_PROTOCOLSTATISTICS) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PortIndex".to_string(), value: port_index.into() });
        args.push(MethodParameter { name: "ProtocolType".to_string(), value: protocol_type.into() });

        let result = self.invoke_method("SM_GetProtocolStatistics", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        let protocol_statistics = result.get_value("ProtocolStatistics")?;
        Ok(result.return_value)

    }


/// 

    /// * `in_num_of_phy_counters` -  (u32)
    /// * `phy_index` -  (u32)
    /// * `port_index` -  (u32)

    /// * `hbastatus` -  (u32)
    /// * `out_num_of_phy_counters` -  (u32)
    /// * `phy_counter` -  (i64[])
    /// * `total_num_of_phy_counters` -  (u32)
    pub fn sm__get_phy_statistics(&self, port_index: u32, phy_index: u32, in_num_of_phy_counters: u32, hbastatus: &mut u32, total_num_of_phy_counters: &mut u32, out_num_of_phy_counters: &mut u32, phy_counter: &mut Vec<i64>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PortIndex".to_string(), value: port_index.into() });
        args.push(MethodParameter { name: "PhyIndex".to_string(), value: phy_index.into() });
        args.push(MethodParameter { name: "InNumOfPhyCounters".to_string(), value: in_num_of_phy_counters.into() });

        let result = self.invoke_method("SM_GetPhyStatistics", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        let out_num_of_phy_counters = result.get_value("OutNumOfPhyCounters")?;
        let phy_counter = result.get_value("PhyCounter")?;
        let total_num_of_phy_counters = result.get_value("TotalNumOfPhyCounters")?;
        Ok(result.return_value)

    }


/// 

    /// * `phy_index` -  (u32)
    /// * `port_index` -  (u32)

    /// * `hbastatus` -  (u32)
    /// * `phy_type` -  (MS_SMHBA_FC_PHY)
    pub fn sm__get_fcphy_attributes(&self, port_index: u32, phy_index: u32, hbastatus: &mut u32, phy_type: &mut MS_SMHBA_FC_PHY) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PortIndex".to_string(), value: port_index.into() });
        args.push(MethodParameter { name: "PhyIndex".to_string(), value: phy_index.into() });

        let result = self.invoke_method("SM_GetFCPhyAttributes", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        let phy_type = result.get_value("PhyType")?;
        Ok(result.return_value)

    }


/// 

    /// * `phy_index` -  (u32)
    /// * `port_index` -  (u32)

    /// * `hbastatus` -  (u32)
    /// * `phy_type` -  (MS_SMHBA_SAS_PHY)
    pub fn sm__get_sasphy_attributes(&self, port_index: u32, phy_index: u32, hbastatus: &mut u32, phy_type: &mut MS_SMHBA_SAS_PHY) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PortIndex".to_string(), value: port_index.into() });
        args.push(MethodParameter { name: "PhyIndex".to_string(), value: phy_index.into() });

        let result = self.invoke_method("SM_GetSASPhyAttributes", &args)?;
        let hbastatus = result.get_value("HBAStatus")?;
        let phy_type = result.get_value("PhyType")?;
        Ok(result.return_value)

    }


/// 
    pub fn sm__refresh_information(&self) -> Result<(), WmiError> {
        self.invoke_method("SM_RefreshInformation", &[])

    }

}

