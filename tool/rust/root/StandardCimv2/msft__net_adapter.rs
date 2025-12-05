// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetAdapter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetAdapter {
    #[serde(flatten)]
    pub base: CIM_NetworkPort,

/// 
    #[serde(rename = "AdminLocked")]
    pub admin_locked: Option<bool>,

/// 
    #[serde(rename = "ComponentID")]
    pub component_id: Option<String>,

/// 
    #[serde(rename = "ConnectorPresent")]
    pub connector_present: Option<bool>,

/// 
    #[serde(rename = "DeviceName")]
    pub device_name: Option<String>,

/// 
    #[serde(rename = "DeviceWakeUpEnable")]
    pub device_wake_up_enable: Option<bool>,

/// 
    #[serde(rename = "DriverDate")]
    pub driver_date: Option<String>,

/// 
    #[serde(rename = "DriverDateData")]
    pub driver_date_data: Option<u64>,

/// 
    #[serde(rename = "DriverDescription")]
    pub driver_description: Option<String>,

/// 
    #[serde(rename = "DriverMajorNdisVersion")]
    pub driver_major_ndis_version: Option<u8>,

/// 
    #[serde(rename = "DriverMinorNdisVersion")]
    pub driver_minor_ndis_version: Option<u8>,

/// 
    #[serde(rename = "DriverName")]
    pub driver_name: Option<String>,

/// 
    #[serde(rename = "DriverProvider")]
    pub driver_provider: Option<String>,

/// 
    #[serde(rename = "DriverVersionString")]
    pub driver_version_string: Option<String>,

/// 
    #[serde(rename = "EndPointInterface")]
    pub end_point_interface: Option<bool>,

/// 
    #[serde(rename = "HardwareInterface")]
    pub hardware_interface: Option<bool>,

/// 
    #[serde(rename = "Hidden")]
    pub hidden: Option<bool>,

/// 
    #[serde(rename = "HigherLayerInterfaceIndices")]
    pub higher_layer_interface_indices: Vec<u32>,

/// 
    #[serde(rename = "IMFilter")]
    pub imfilter: Option<bool>,

/// 
    #[serde(rename = "InterfaceAdminStatus")]
    pub interface_admin_status: Option<u32>,

/// 
    #[serde(rename = "InterfaceDescription")]
    pub interface_description: Option<String>,

/// 
    #[serde(rename = "InterfaceGuid")]
    pub interface_guid: Option<String>,

/// 
    #[serde(rename = "InterfaceIndex")]
    pub interface_index: Option<u32>,

/// 
    #[serde(rename = "InterfaceName")]
    pub interface_name: Option<String>,

/// 
    #[serde(rename = "InterfaceOperationalStatus")]
    pub interface_operational_status: Option<u32>,

/// 
    #[serde(rename = "InterfaceType")]
    pub interface_type: Option<u32>,

/// 
    #[serde(rename = "iSCSIInterface")]
    pub i_scsiinterface: Option<bool>,

/// 
    #[serde(rename = "LowerLayerInterfaceIndices")]
    pub lower_layer_interface_indices: Vec<u32>,

/// 
    #[serde(rename = "MajorDriverVersion")]
    pub major_driver_version: Option<u16>,

/// 
    #[serde(rename = "MediaConnectState")]
    pub media_connect_state: Option<u32>,

/// 
    #[serde(rename = "MediaDuplexState")]
    pub media_duplex_state: Option<u32>,

/// 
    #[serde(rename = "MinorDriverVersion")]
    pub minor_driver_version: Option<u16>,

/// 
    #[serde(rename = "MtuSize")]
    pub mtu_size: Option<u32>,

/// 
    #[serde(rename = "NdisMedium")]
    pub ndis_medium: Option<u32>,

/// 
    #[serde(rename = "NdisPhysicalMedium")]
    pub ndis_physical_medium: Option<u32>,

/// 
    #[serde(rename = "NetLuid")]
    pub net_luid: Option<u64>,

/// 
    #[serde(rename = "NetLuidIndex")]
    pub net_luid_index: Option<u32>,

/// 
    #[serde(rename = "NotUserRemovable")]
    pub not_user_removable: Option<bool>,

/// 
    #[serde(rename = "OperationalStatusDownDefaultPortNotAuthenticated")]
    pub operational_status_down_default_port_not_authenticated: Option<bool>,

/// 
    #[serde(rename = "OperationalStatusDownInterfacePaused")]
    pub operational_status_down_interface_paused: Option<bool>,

/// 
    #[serde(rename = "OperationalStatusDownLowPowerState")]
    pub operational_status_down_low_power_state: Option<bool>,

/// 
    #[serde(rename = "OperationalStatusDownMediaDisconnected")]
    pub operational_status_down_media_disconnected: Option<bool>,

/// 
    #[serde(rename = "PnPDeviceID")]
    pub pn_pdevice_id: Option<String>,

/// 
    #[serde(rename = "PromiscuousMode")]
    pub promiscuous_mode: Option<bool>,

/// 
    #[serde(rename = "ReceiveLinkSpeed")]
    pub receive_link_speed: Option<u64>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u32>,

/// 
    #[serde(rename = "TransmitLinkSpeed")]
    pub transmit_link_speed: Option<u64>,

/// 
    #[serde(rename = "Virtual")]
    pub virtual: Option<bool>,

/// 
    #[serde(rename = "VlanID")]
    pub vlan_id: Option<u16>,

/// 
    #[serde(rename = "WdmInterface")]
    pub wdm_interface: Option<bool>,
}

impl MSFT_NetAdapter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_NetworkPort::new(),
            admin_locked: None,
            component_id: None,
            connector_present: None,
            device_name: None,
            device_wake_up_enable: None,
            driver_date: None,
            driver_date_data: None,
            driver_description: None,
            driver_major_ndis_version: None,
            driver_minor_ndis_version: None,
            driver_name: None,
            driver_provider: None,
            driver_version_string: None,
            end_point_interface: None,
            hardware_interface: None,
            hidden: None,
            higher_layer_interface_indices: Vec::new(),
            imfilter: None,
            interface_admin_status: None,
            interface_description: None,
            interface_guid: None,
            interface_index: None,
            interface_name: None,
            interface_operational_status: None,
            interface_type: None,
            i_scsiinterface: None,
            lower_layer_interface_indices: Vec::new(),
            major_driver_version: None,
            media_connect_state: None,
            media_duplex_state: None,
            minor_driver_version: None,
            mtu_size: None,
            ndis_medium: None,
            ndis_physical_medium: None,
            net_luid: None,
            net_luid_index: None,
            not_user_removable: None,
            operational_status_down_default_port_not_authenticated: None,
            operational_status_down_interface_paused: None,
            operational_status_down_low_power_state: None,
            operational_status_down_media_disconnected: None,
            pn_pdevice_id: None,
            promiscuous_mode: None,
            receive_link_speed: None,
            state: None,
            transmit_link_speed: None,
            virtual: None,
            vlan_id: None,
            wdm_interface: None,
        }
    }


    /// Sets the value of AdminLocked
    pub fn set_admin_locked(&mut self, value: bool) {
        self.admin_locked = Some(value);
    }

    /// Gets the value of AdminLocked
    pub fn get_admin_locked(&self) -> Option<&bool> {
        self.admin_locked.as_ref()
    }

    /// Sets the value of ComponentID
    pub fn set_component_id(&mut self, value: String) {
        self.component_id = Some(value);
    }

    /// Gets the value of ComponentID
    pub fn get_component_id(&self) -> Option<&String> {
        self.component_id.as_ref()
    }

    /// Sets the value of ConnectorPresent
    pub fn set_connector_present(&mut self, value: bool) {
        self.connector_present = Some(value);
    }

    /// Gets the value of ConnectorPresent
    pub fn get_connector_present(&self) -> Option<&bool> {
        self.connector_present.as_ref()
    }

    /// Sets the value of DeviceName
    pub fn set_device_name(&mut self, value: String) {
        self.device_name = Some(value);
    }

    /// Gets the value of DeviceName
    pub fn get_device_name(&self) -> Option<&String> {
        self.device_name.as_ref()
    }

    /// Sets the value of DeviceWakeUpEnable
    pub fn set_device_wake_up_enable(&mut self, value: bool) {
        self.device_wake_up_enable = Some(value);
    }

    /// Gets the value of DeviceWakeUpEnable
    pub fn get_device_wake_up_enable(&self) -> Option<&bool> {
        self.device_wake_up_enable.as_ref()
    }

    /// Sets the value of DriverDate
    pub fn set_driver_date(&mut self, value: String) {
        self.driver_date = Some(value);
    }

    /// Gets the value of DriverDate
    pub fn get_driver_date(&self) -> Option<&String> {
        self.driver_date.as_ref()
    }

    /// Sets the value of DriverDateData
    pub fn set_driver_date_data(&mut self, value: u64) {
        self.driver_date_data = Some(value);
    }

    /// Gets the value of DriverDateData
    pub fn get_driver_date_data(&self) -> Option<&u64> {
        self.driver_date_data.as_ref()
    }

    /// Sets the value of DriverDescription
    pub fn set_driver_description(&mut self, value: String) {
        self.driver_description = Some(value);
    }

    /// Gets the value of DriverDescription
    pub fn get_driver_description(&self) -> Option<&String> {
        self.driver_description.as_ref()
    }

    /// Sets the value of DriverMajorNdisVersion
    pub fn set_driver_major_ndis_version(&mut self, value: u8) {
        self.driver_major_ndis_version = Some(value);
    }

    /// Gets the value of DriverMajorNdisVersion
    pub fn get_driver_major_ndis_version(&self) -> Option<&u8> {
        self.driver_major_ndis_version.as_ref()
    }

    /// Sets the value of DriverMinorNdisVersion
    pub fn set_driver_minor_ndis_version(&mut self, value: u8) {
        self.driver_minor_ndis_version = Some(value);
    }

    /// Gets the value of DriverMinorNdisVersion
    pub fn get_driver_minor_ndis_version(&self) -> Option<&u8> {
        self.driver_minor_ndis_version.as_ref()
    }

    /// Sets the value of DriverName
    pub fn set_driver_name(&mut self, value: String) {
        self.driver_name = Some(value);
    }

    /// Gets the value of DriverName
    pub fn get_driver_name(&self) -> Option<&String> {
        self.driver_name.as_ref()
    }

    /// Sets the value of DriverProvider
    pub fn set_driver_provider(&mut self, value: String) {
        self.driver_provider = Some(value);
    }

    /// Gets the value of DriverProvider
    pub fn get_driver_provider(&self) -> Option<&String> {
        self.driver_provider.as_ref()
    }

    /// Sets the value of DriverVersionString
    pub fn set_driver_version_string(&mut self, value: String) {
        self.driver_version_string = Some(value);
    }

    /// Gets the value of DriverVersionString
    pub fn get_driver_version_string(&self) -> Option<&String> {
        self.driver_version_string.as_ref()
    }

    /// Sets the value of EndPointInterface
    pub fn set_end_point_interface(&mut self, value: bool) {
        self.end_point_interface = Some(value);
    }

    /// Gets the value of EndPointInterface
    pub fn get_end_point_interface(&self) -> Option<&bool> {
        self.end_point_interface.as_ref()
    }

    /// Sets the value of HardwareInterface
    pub fn set_hardware_interface(&mut self, value: bool) {
        self.hardware_interface = Some(value);
    }

    /// Gets the value of HardwareInterface
    pub fn get_hardware_interface(&self) -> Option<&bool> {
        self.hardware_interface.as_ref()
    }

    /// Sets the value of Hidden
    pub fn set_hidden(&mut self, value: bool) {
        self.hidden = Some(value);
    }

    /// Gets the value of Hidden
    pub fn get_hidden(&self) -> Option<&bool> {
        self.hidden.as_ref()
    }

    /// Sets the value of HigherLayerInterfaceIndices
    pub fn set_higher_layer_interface_indices(&mut self, value: Vec<u32>) {
        self.higher_layer_interface_indices = value;
    }

    /// Gets the value of HigherLayerInterfaceIndices
    pub fn get_higher_layer_interface_indices(&self) -> &Vec<u32> {
        &self.higher_layer_interface_indices
    }

    /// Sets the value of IMFilter
    pub fn set_imfilter(&mut self, value: bool) {
        self.imfilter = Some(value);
    }

    /// Gets the value of IMFilter
    pub fn get_imfilter(&self) -> Option<&bool> {
        self.imfilter.as_ref()
    }

    /// Sets the value of InterfaceAdminStatus
    pub fn set_interface_admin_status(&mut self, value: u32) {
        self.interface_admin_status = Some(value);
    }

    /// Gets the value of InterfaceAdminStatus
    pub fn get_interface_admin_status(&self) -> Option<&u32> {
        self.interface_admin_status.as_ref()
    }

    /// Sets the value of InterfaceDescription
    pub fn set_interface_description(&mut self, value: String) {
        self.interface_description = Some(value);
    }

    /// Gets the value of InterfaceDescription
    pub fn get_interface_description(&self) -> Option<&String> {
        self.interface_description.as_ref()
    }

    /// Sets the value of InterfaceGuid
    pub fn set_interface_guid(&mut self, value: String) {
        self.interface_guid = Some(value);
    }

    /// Gets the value of InterfaceGuid
    pub fn get_interface_guid(&self) -> Option<&String> {
        self.interface_guid.as_ref()
    }

    /// Sets the value of InterfaceIndex
    pub fn set_interface_index(&mut self, value: u32) {
        self.interface_index = Some(value);
    }

    /// Gets the value of InterfaceIndex
    pub fn get_interface_index(&self) -> Option<&u32> {
        self.interface_index.as_ref()
    }

    /// Sets the value of InterfaceName
    pub fn set_interface_name(&mut self, value: String) {
        self.interface_name = Some(value);
    }

    /// Gets the value of InterfaceName
    pub fn get_interface_name(&self) -> Option<&String> {
        self.interface_name.as_ref()
    }

    /// Sets the value of InterfaceOperationalStatus
    pub fn set_interface_operational_status(&mut self, value: u32) {
        self.interface_operational_status = Some(value);
    }

    /// Gets the value of InterfaceOperationalStatus
    pub fn get_interface_operational_status(&self) -> Option<&u32> {
        self.interface_operational_status.as_ref()
    }

    /// Sets the value of InterfaceType
    pub fn set_interface_type(&mut self, value: u32) {
        self.interface_type = Some(value);
    }

    /// Gets the value of InterfaceType
    pub fn get_interface_type(&self) -> Option<&u32> {
        self.interface_type.as_ref()
    }

    /// Sets the value of iSCSIInterface
    pub fn set_i_scsiinterface(&mut self, value: bool) {
        self.i_scsiinterface = Some(value);
    }

    /// Gets the value of iSCSIInterface
    pub fn get_i_scsiinterface(&self) -> Option<&bool> {
        self.i_scsiinterface.as_ref()
    }

    /// Sets the value of LowerLayerInterfaceIndices
    pub fn set_lower_layer_interface_indices(&mut self, value: Vec<u32>) {
        self.lower_layer_interface_indices = value;
    }

    /// Gets the value of LowerLayerInterfaceIndices
    pub fn get_lower_layer_interface_indices(&self) -> &Vec<u32> {
        &self.lower_layer_interface_indices
    }

    /// Sets the value of MajorDriverVersion
    pub fn set_major_driver_version(&mut self, value: u16) {
        self.major_driver_version = Some(value);
    }

    /// Gets the value of MajorDriverVersion
    pub fn get_major_driver_version(&self) -> Option<&u16> {
        self.major_driver_version.as_ref()
    }

    /// Sets the value of MediaConnectState
    pub fn set_media_connect_state(&mut self, value: u32) {
        self.media_connect_state = Some(value);
    }

    /// Gets the value of MediaConnectState
    pub fn get_media_connect_state(&self) -> Option<&u32> {
        self.media_connect_state.as_ref()
    }

    /// Sets the value of MediaDuplexState
    pub fn set_media_duplex_state(&mut self, value: u32) {
        self.media_duplex_state = Some(value);
    }

    /// Gets the value of MediaDuplexState
    pub fn get_media_duplex_state(&self) -> Option<&u32> {
        self.media_duplex_state.as_ref()
    }

    /// Sets the value of MinorDriverVersion
    pub fn set_minor_driver_version(&mut self, value: u16) {
        self.minor_driver_version = Some(value);
    }

    /// Gets the value of MinorDriverVersion
    pub fn get_minor_driver_version(&self) -> Option<&u16> {
        self.minor_driver_version.as_ref()
    }

    /// Sets the value of MtuSize
    pub fn set_mtu_size(&mut self, value: u32) {
        self.mtu_size = Some(value);
    }

    /// Gets the value of MtuSize
    pub fn get_mtu_size(&self) -> Option<&u32> {
        self.mtu_size.as_ref()
    }

    /// Sets the value of NdisMedium
    pub fn set_ndis_medium(&mut self, value: u32) {
        self.ndis_medium = Some(value);
    }

    /// Gets the value of NdisMedium
    pub fn get_ndis_medium(&self) -> Option<&u32> {
        self.ndis_medium.as_ref()
    }

    /// Sets the value of NdisPhysicalMedium
    pub fn set_ndis_physical_medium(&mut self, value: u32) {
        self.ndis_physical_medium = Some(value);
    }

    /// Gets the value of NdisPhysicalMedium
    pub fn get_ndis_physical_medium(&self) -> Option<&u32> {
        self.ndis_physical_medium.as_ref()
    }

    /// Sets the value of NetLuid
    pub fn set_net_luid(&mut self, value: u64) {
        self.net_luid = Some(value);
    }

    /// Gets the value of NetLuid
    pub fn get_net_luid(&self) -> Option<&u64> {
        self.net_luid.as_ref()
    }

    /// Sets the value of NetLuidIndex
    pub fn set_net_luid_index(&mut self, value: u32) {
        self.net_luid_index = Some(value);
    }

    /// Gets the value of NetLuidIndex
    pub fn get_net_luid_index(&self) -> Option<&u32> {
        self.net_luid_index.as_ref()
    }

    /// Sets the value of NotUserRemovable
    pub fn set_not_user_removable(&mut self, value: bool) {
        self.not_user_removable = Some(value);
    }

    /// Gets the value of NotUserRemovable
    pub fn get_not_user_removable(&self) -> Option<&bool> {
        self.not_user_removable.as_ref()
    }

    /// Sets the value of OperationalStatusDownDefaultPortNotAuthenticated
    pub fn set_operational_status_down_default_port_not_authenticated(&mut self, value: bool) {
        self.operational_status_down_default_port_not_authenticated = Some(value);
    }

    /// Gets the value of OperationalStatusDownDefaultPortNotAuthenticated
    pub fn get_operational_status_down_default_port_not_authenticated(&self) -> Option<&bool> {
        self.operational_status_down_default_port_not_authenticated.as_ref()
    }

    /// Sets the value of OperationalStatusDownInterfacePaused
    pub fn set_operational_status_down_interface_paused(&mut self, value: bool) {
        self.operational_status_down_interface_paused = Some(value);
    }

    /// Gets the value of OperationalStatusDownInterfacePaused
    pub fn get_operational_status_down_interface_paused(&self) -> Option<&bool> {
        self.operational_status_down_interface_paused.as_ref()
    }

    /// Sets the value of OperationalStatusDownLowPowerState
    pub fn set_operational_status_down_low_power_state(&mut self, value: bool) {
        self.operational_status_down_low_power_state = Some(value);
    }

    /// Gets the value of OperationalStatusDownLowPowerState
    pub fn get_operational_status_down_low_power_state(&self) -> Option<&bool> {
        self.operational_status_down_low_power_state.as_ref()
    }

    /// Sets the value of OperationalStatusDownMediaDisconnected
    pub fn set_operational_status_down_media_disconnected(&mut self, value: bool) {
        self.operational_status_down_media_disconnected = Some(value);
    }

    /// Gets the value of OperationalStatusDownMediaDisconnected
    pub fn get_operational_status_down_media_disconnected(&self) -> Option<&bool> {
        self.operational_status_down_media_disconnected.as_ref()
    }

    /// Sets the value of PnPDeviceID
    pub fn set_pn_pdevice_id(&mut self, value: String) {
        self.pn_pdevice_id = Some(value);
    }

    /// Gets the value of PnPDeviceID
    pub fn get_pn_pdevice_id(&self) -> Option<&String> {
        self.pn_pdevice_id.as_ref()
    }

    /// Sets the value of PromiscuousMode
    pub fn set_promiscuous_mode(&mut self, value: bool) {
        self.promiscuous_mode = Some(value);
    }

    /// Gets the value of PromiscuousMode
    pub fn get_promiscuous_mode(&self) -> Option<&bool> {
        self.promiscuous_mode.as_ref()
    }

    /// Sets the value of ReceiveLinkSpeed
    pub fn set_receive_link_speed(&mut self, value: u64) {
        self.receive_link_speed = Some(value);
    }

    /// Gets the value of ReceiveLinkSpeed
    pub fn get_receive_link_speed(&self) -> Option<&u64> {
        self.receive_link_speed.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: u32) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&u32> {
        self.state.as_ref()
    }

    /// Sets the value of TransmitLinkSpeed
    pub fn set_transmit_link_speed(&mut self, value: u64) {
        self.transmit_link_speed = Some(value);
    }

    /// Gets the value of TransmitLinkSpeed
    pub fn get_transmit_link_speed(&self) -> Option<&u64> {
        self.transmit_link_speed.as_ref()
    }

    /// Sets the value of Virtual
    pub fn set_virtual(&mut self, value: bool) {
        self.virtual = Some(value);
    }

    /// Gets the value of Virtual
    pub fn get_virtual(&self) -> Option<&bool> {
        self.virtual.as_ref()
    }

    /// Sets the value of VlanID
    pub fn set_vlan_id(&mut self, value: u16) {
        self.vlan_id = Some(value);
    }

    /// Gets the value of VlanID
    pub fn get_vlan_id(&self) -> Option<&u16> {
        self.vlan_id.as_ref()
    }

    /// Sets the value of WdmInterface
    pub fn set_wdm_interface(&mut self, value: bool) {
        self.wdm_interface = Some(value);
    }

    /// Gets the value of WdmInterface
    pub fn get_wdm_interface(&self) -> Option<&bool> {
        self.wdm_interface.as_ref()
    }

/// 

    /// * `cmdlet_output` -  (MSFT_NetAdapter)
    /// * `return_value` -  (u32)
    pub fn enable(&self, cmdlet_output: &mut MSFT_NetAdapter) -> Result<(), WmiError> {

        let result = self.invoke_method("Enable", &[])?;
        let cmdlet_output = result.get_value("CmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `cmdlet_output` -  (MSFT_NetAdapter)
    /// * `return_value` -  (u32)
    pub fn disable(&self, cmdlet_output: &mut MSFT_NetAdapter) -> Result<(), WmiError> {

        let result = self.invoke_method("Disable", &[])?;
        let cmdlet_output = result.get_value("CmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `cmdlet_output` -  (MSFT_NetAdapter)
    /// * `return_value` -  (u32)
    pub fn restart(&self, cmdlet_output: &mut MSFT_NetAdapter) -> Result<(), WmiError> {

        let result = self.invoke_method("Restart", &[])?;
        let cmdlet_output = result.get_value("CmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `cmdlet_output` -  (MSFT_NetAdapter)
    /// * `return_value` -  (u32)
    pub fn lock(&self, cmdlet_output: &mut MSFT_NetAdapter) -> Result<(), WmiError> {

        let result = self.invoke_method("Lock", &[])?;
        let cmdlet_output = result.get_value("CmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `cmdlet_output` -  (MSFT_NetAdapter)
    /// * `return_value` -  (u32)
    pub fn unlock(&self, cmdlet_output: &mut MSFT_NetAdapter) -> Result<(), WmiError> {

        let result = self.invoke_method("Unlock", &[])?;
        let cmdlet_output = result.get_value("CmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `new_name` -  (String)

    /// * `cmdlet_output` -  (MSFT_NetAdapter)
    /// * `return_value` -  (u32)
    pub fn rename(&self, new_name: &String, cmdlet_output: &mut MSFT_NetAdapter) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NewName".to_string(), value: new_name.into() });

        let result = self.invoke_method("Rename", &args)?;
        let cmdlet_output = result.get_value("CmdletOutput")?;
        Ok(result.return_value)

    }

}

