// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ComputerSystem struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ComputerSystem {
    #[serde(flatten)]
    pub base: CIM_UnitaryComputerSystem,

/// 
    #[serde(rename = "AdminPasswordStatus")]
    pub admin_password_status: Option<u16>,

/// 
    #[serde(rename = "AutomaticManagedPagefile")]
    pub automatic_managed_pagefile: Option<bool>,

/// 
    #[serde(rename = "AutomaticResetBootOption")]
    pub automatic_reset_boot_option: Option<bool>,

/// 
    #[serde(rename = "AutomaticResetCapability")]
    pub automatic_reset_capability: Option<bool>,

/// 
    #[serde(rename = "BootOptionOnLimit")]
    pub boot_option_on_limit: Option<u16>,

/// 
    #[serde(rename = "BootOptionOnWatchDog")]
    pub boot_option_on_watch_dog: Option<u16>,

/// 
    #[serde(rename = "BootROMSupported")]
    pub boot_romsupported: Option<bool>,

/// 
    #[serde(rename = "BootStatus")]
    pub boot_status: Vec<u16>,

/// 
    #[serde(rename = "BootupState")]
    pub bootup_state: Option<String>,

/// 
    #[serde(rename = "ChassisBootupState")]
    pub chassis_bootup_state: Option<u16>,

/// 
    #[serde(rename = "ChassisSKUNumber")]
    pub chassis_skunumber: Option<String>,

/// 
    #[serde(rename = "CurrentTimeZone")]
    pub current_time_zone: Option<i16>,

/// 
    #[serde(rename = "DaylightInEffect")]
    pub daylight_in_effect: Option<bool>,

/// 
    #[serde(rename = "DNSHostName")]
    pub dnshost_name: Option<String>,

/// 
    #[serde(rename = "Domain")]
    pub domain: Option<String>,

/// 
    #[serde(rename = "DomainRole")]
    pub domain_role: Option<u16>,

/// 
    #[serde(rename = "EnableDaylightSavingsTime")]
    pub enable_daylight_savings_time: Option<bool>,

/// 
    #[serde(rename = "FrontPanelResetStatus")]
    pub front_panel_reset_status: Option<u16>,

/// 
    #[serde(rename = "HypervisorPresent")]
    pub hypervisor_present: Option<bool>,

/// 
    #[serde(rename = "InfraredSupported")]
    pub infrared_supported: Option<bool>,

/// 
    #[serde(rename = "KeyboardPasswordStatus")]
    pub keyboard_password_status: Option<u16>,

/// 
    #[serde(rename = "Manufacturer")]
    pub manufacturer: Option<String>,

/// 
    #[serde(rename = "Model")]
    pub model: Option<String>,

/// 
    #[serde(rename = "NetworkServerModeEnabled")]
    pub network_server_mode_enabled: Option<bool>,

/// 
    #[serde(rename = "NumberOfLogicalProcessors")]
    pub number_of_logical_processors: Option<u32>,

/// 
    #[serde(rename = "NumberOfProcessors")]
    pub number_of_processors: Option<u32>,

/// 
    #[serde(rename = "OEMLogoBitmap")]
    pub oemlogo_bitmap: Vec<u8>,

/// 
    #[serde(rename = "OEMStringArray")]
    pub oemstring_array: Vec<String>,

/// 
    #[serde(rename = "PartOfDomain")]
    pub part_of_domain: Option<bool>,

/// 
    #[serde(rename = "PauseAfterReset")]
    pub pause_after_reset: Option<i64>,

/// 
    #[serde(rename = "PCSystemType")]
    pub pcsystem_type: Option<u16>,

/// 
    #[serde(rename = "PCSystemTypeEx")]
    pub pcsystem_type_ex: Option<u16>,

/// 
    #[serde(rename = "PowerOnPasswordStatus")]
    pub power_on_password_status: Option<u16>,

/// 
    #[serde(rename = "PowerSupplyState")]
    pub power_supply_state: Option<u16>,

/// 
    #[serde(rename = "ResetCount")]
    pub reset_count: Option<i16>,

/// 
    #[serde(rename = "ResetLimit")]
    pub reset_limit: Option<i16>,

/// 
    #[serde(rename = "SupportContactDescription")]
    pub support_contact_description: Vec<String>,

/// 
    #[serde(rename = "SystemFamily")]
    pub system_family: Option<String>,

/// 
    #[serde(rename = "SystemSKUNumber")]
    pub system_skunumber: Option<String>,

/// 
    #[serde(rename = "SystemStartupDelay")]
    pub system_startup_delay: Option<u16>,

/// 
    #[serde(rename = "SystemStartupOptions")]
    pub system_startup_options: Vec<String>,

/// 
    #[serde(rename = "SystemStartupSetting")]
    pub system_startup_setting: Option<u8>,

/// 
    #[serde(rename = "SystemType")]
    pub system_type: Option<String>,

/// 
    #[serde(rename = "ThermalState")]
    pub thermal_state: Option<u16>,

/// 
    #[serde(rename = "TotalPhysicalMemory")]
    pub total_physical_memory: Option<u64>,

/// 
    #[serde(rename = "UserName")]
    pub user_name: Option<String>,

/// 
    #[serde(rename = "WakeUpType")]
    pub wake_up_type: Option<u16>,

/// 
    #[serde(rename = "Workgroup")]
    pub workgroup: Option<String>,
}

impl Win32_ComputerSystem {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_UnitaryComputerSystem::new(),
            admin_password_status: None,
            automatic_managed_pagefile: None,
            automatic_reset_boot_option: None,
            automatic_reset_capability: None,
            boot_option_on_limit: None,
            boot_option_on_watch_dog: None,
            boot_romsupported: None,
            boot_status: Vec::new(),
            bootup_state: None,
            chassis_bootup_state: None,
            chassis_skunumber: None,
            current_time_zone: None,
            daylight_in_effect: None,
            dnshost_name: None,
            domain: None,
            domain_role: None,
            enable_daylight_savings_time: None,
            front_panel_reset_status: None,
            hypervisor_present: None,
            infrared_supported: None,
            keyboard_password_status: None,
            manufacturer: None,
            model: None,
            network_server_mode_enabled: None,
            number_of_logical_processors: None,
            number_of_processors: None,
            oemlogo_bitmap: Vec::new(),
            oemstring_array: Vec::new(),
            part_of_domain: None,
            pause_after_reset: None,
            pcsystem_type: None,
            pcsystem_type_ex: None,
            power_on_password_status: None,
            power_supply_state: None,
            reset_count: None,
            reset_limit: None,
            support_contact_description: Vec::new(),
            system_family: None,
            system_skunumber: None,
            system_startup_delay: None,
            system_startup_options: Vec::new(),
            system_startup_setting: None,
            system_type: None,
            thermal_state: None,
            total_physical_memory: None,
            user_name: None,
            wake_up_type: None,
            workgroup: None,
        }
    }


    /// Sets the value of AdminPasswordStatus
    pub fn set_admin_password_status(&mut self, value: u16) {
        self.admin_password_status = Some(value);
    }

    /// Gets the value of AdminPasswordStatus
    pub fn get_admin_password_status(&self) -> Option<&u16> {
        self.admin_password_status.as_ref()
    }

    /// Sets the value of AutomaticManagedPagefile
    pub fn set_automatic_managed_pagefile(&mut self, value: bool) {
        self.automatic_managed_pagefile = Some(value);
    }

    /// Gets the value of AutomaticManagedPagefile
    pub fn get_automatic_managed_pagefile(&self) -> Option<&bool> {
        self.automatic_managed_pagefile.as_ref()
    }

    /// Sets the value of AutomaticResetBootOption
    pub fn set_automatic_reset_boot_option(&mut self, value: bool) {
        self.automatic_reset_boot_option = Some(value);
    }

    /// Gets the value of AutomaticResetBootOption
    pub fn get_automatic_reset_boot_option(&self) -> Option<&bool> {
        self.automatic_reset_boot_option.as_ref()
    }

    /// Sets the value of AutomaticResetCapability
    pub fn set_automatic_reset_capability(&mut self, value: bool) {
        self.automatic_reset_capability = Some(value);
    }

    /// Gets the value of AutomaticResetCapability
    pub fn get_automatic_reset_capability(&self) -> Option<&bool> {
        self.automatic_reset_capability.as_ref()
    }

    /// Sets the value of BootOptionOnLimit
    pub fn set_boot_option_on_limit(&mut self, value: u16) {
        self.boot_option_on_limit = Some(value);
    }

    /// Gets the value of BootOptionOnLimit
    pub fn get_boot_option_on_limit(&self) -> Option<&u16> {
        self.boot_option_on_limit.as_ref()
    }

    /// Sets the value of BootOptionOnWatchDog
    pub fn set_boot_option_on_watch_dog(&mut self, value: u16) {
        self.boot_option_on_watch_dog = Some(value);
    }

    /// Gets the value of BootOptionOnWatchDog
    pub fn get_boot_option_on_watch_dog(&self) -> Option<&u16> {
        self.boot_option_on_watch_dog.as_ref()
    }

    /// Sets the value of BootROMSupported
    pub fn set_boot_romsupported(&mut self, value: bool) {
        self.boot_romsupported = Some(value);
    }

    /// Gets the value of BootROMSupported
    pub fn get_boot_romsupported(&self) -> Option<&bool> {
        self.boot_romsupported.as_ref()
    }

    /// Sets the value of BootStatus
    pub fn set_boot_status(&mut self, value: Vec<u16>) {
        self.boot_status = value;
    }

    /// Gets the value of BootStatus
    pub fn get_boot_status(&self) -> &Vec<u16> {
        &self.boot_status
    }

    /// Sets the value of BootupState
    pub fn set_bootup_state(&mut self, value: String) {
        self.bootup_state = Some(value);
    }

    /// Gets the value of BootupState
    pub fn get_bootup_state(&self) -> Option<&String> {
        self.bootup_state.as_ref()
    }

    /// Sets the value of ChassisBootupState
    pub fn set_chassis_bootup_state(&mut self, value: u16) {
        self.chassis_bootup_state = Some(value);
    }

    /// Gets the value of ChassisBootupState
    pub fn get_chassis_bootup_state(&self) -> Option<&u16> {
        self.chassis_bootup_state.as_ref()
    }

    /// Sets the value of ChassisSKUNumber
    pub fn set_chassis_skunumber(&mut self, value: String) {
        self.chassis_skunumber = Some(value);
    }

    /// Gets the value of ChassisSKUNumber
    pub fn get_chassis_skunumber(&self) -> Option<&String> {
        self.chassis_skunumber.as_ref()
    }

    /// Sets the value of CurrentTimeZone
    pub fn set_current_time_zone(&mut self, value: i16) {
        self.current_time_zone = Some(value);
    }

    /// Gets the value of CurrentTimeZone
    pub fn get_current_time_zone(&self) -> Option<&i16> {
        self.current_time_zone.as_ref()
    }

    /// Sets the value of DaylightInEffect
    pub fn set_daylight_in_effect(&mut self, value: bool) {
        self.daylight_in_effect = Some(value);
    }

    /// Gets the value of DaylightInEffect
    pub fn get_daylight_in_effect(&self) -> Option<&bool> {
        self.daylight_in_effect.as_ref()
    }

    /// Sets the value of DNSHostName
    pub fn set_dnshost_name(&mut self, value: String) {
        self.dnshost_name = Some(value);
    }

    /// Gets the value of DNSHostName
    pub fn get_dnshost_name(&self) -> Option<&String> {
        self.dnshost_name.as_ref()
    }

    /// Sets the value of Domain
    pub fn set_domain(&mut self, value: String) {
        self.domain = Some(value);
    }

    /// Gets the value of Domain
    pub fn get_domain(&self) -> Option<&String> {
        self.domain.as_ref()
    }

    /// Sets the value of DomainRole
    pub fn set_domain_role(&mut self, value: u16) {
        self.domain_role = Some(value);
    }

    /// Gets the value of DomainRole
    pub fn get_domain_role(&self) -> Option<&u16> {
        self.domain_role.as_ref()
    }

    /// Sets the value of EnableDaylightSavingsTime
    pub fn set_enable_daylight_savings_time(&mut self, value: bool) {
        self.enable_daylight_savings_time = Some(value);
    }

    /// Gets the value of EnableDaylightSavingsTime
    pub fn get_enable_daylight_savings_time(&self) -> Option<&bool> {
        self.enable_daylight_savings_time.as_ref()
    }

    /// Sets the value of FrontPanelResetStatus
    pub fn set_front_panel_reset_status(&mut self, value: u16) {
        self.front_panel_reset_status = Some(value);
    }

    /// Gets the value of FrontPanelResetStatus
    pub fn get_front_panel_reset_status(&self) -> Option<&u16> {
        self.front_panel_reset_status.as_ref()
    }

    /// Sets the value of HypervisorPresent
    pub fn set_hypervisor_present(&mut self, value: bool) {
        self.hypervisor_present = Some(value);
    }

    /// Gets the value of HypervisorPresent
    pub fn get_hypervisor_present(&self) -> Option<&bool> {
        self.hypervisor_present.as_ref()
    }

    /// Sets the value of InfraredSupported
    pub fn set_infrared_supported(&mut self, value: bool) {
        self.infrared_supported = Some(value);
    }

    /// Gets the value of InfraredSupported
    pub fn get_infrared_supported(&self) -> Option<&bool> {
        self.infrared_supported.as_ref()
    }

    /// Sets the value of KeyboardPasswordStatus
    pub fn set_keyboard_password_status(&mut self, value: u16) {
        self.keyboard_password_status = Some(value);
    }

    /// Gets the value of KeyboardPasswordStatus
    pub fn get_keyboard_password_status(&self) -> Option<&u16> {
        self.keyboard_password_status.as_ref()
    }

    /// Sets the value of Manufacturer
    pub fn set_manufacturer(&mut self, value: String) {
        self.manufacturer = Some(value);
    }

    /// Gets the value of Manufacturer
    pub fn get_manufacturer(&self) -> Option<&String> {
        self.manufacturer.as_ref()
    }

    /// Sets the value of Model
    pub fn set_model(&mut self, value: String) {
        self.model = Some(value);
    }

    /// Gets the value of Model
    pub fn get_model(&self) -> Option<&String> {
        self.model.as_ref()
    }

    /// Sets the value of NetworkServerModeEnabled
    pub fn set_network_server_mode_enabled(&mut self, value: bool) {
        self.network_server_mode_enabled = Some(value);
    }

    /// Gets the value of NetworkServerModeEnabled
    pub fn get_network_server_mode_enabled(&self) -> Option<&bool> {
        self.network_server_mode_enabled.as_ref()
    }

    /// Sets the value of NumberOfLogicalProcessors
    pub fn set_number_of_logical_processors(&mut self, value: u32) {
        self.number_of_logical_processors = Some(value);
    }

    /// Gets the value of NumberOfLogicalProcessors
    pub fn get_number_of_logical_processors(&self) -> Option<&u32> {
        self.number_of_logical_processors.as_ref()
    }

    /// Sets the value of NumberOfProcessors
    pub fn set_number_of_processors(&mut self, value: u32) {
        self.number_of_processors = Some(value);
    }

    /// Gets the value of NumberOfProcessors
    pub fn get_number_of_processors(&self) -> Option<&u32> {
        self.number_of_processors.as_ref()
    }

    /// Sets the value of OEMLogoBitmap
    pub fn set_oemlogo_bitmap(&mut self, value: Vec<u8>) {
        self.oemlogo_bitmap = value;
    }

    /// Gets the value of OEMLogoBitmap
    pub fn get_oemlogo_bitmap(&self) -> &Vec<u8> {
        &self.oemlogo_bitmap
    }

    /// Sets the value of OEMStringArray
    pub fn set_oemstring_array(&mut self, value: Vec<String>) {
        self.oemstring_array = value;
    }

    /// Gets the value of OEMStringArray
    pub fn get_oemstring_array(&self) -> &Vec<String> {
        &self.oemstring_array
    }

    /// Sets the value of PartOfDomain
    pub fn set_part_of_domain(&mut self, value: bool) {
        self.part_of_domain = Some(value);
    }

    /// Gets the value of PartOfDomain
    pub fn get_part_of_domain(&self) -> Option<&bool> {
        self.part_of_domain.as_ref()
    }

    /// Sets the value of PauseAfterReset
    pub fn set_pause_after_reset(&mut self, value: i64) {
        self.pause_after_reset = Some(value);
    }

    /// Gets the value of PauseAfterReset
    pub fn get_pause_after_reset(&self) -> Option<&i64> {
        self.pause_after_reset.as_ref()
    }

    /// Sets the value of PCSystemType
    pub fn set_pcsystem_type(&mut self, value: u16) {
        self.pcsystem_type = Some(value);
    }

    /// Gets the value of PCSystemType
    pub fn get_pcsystem_type(&self) -> Option<&u16> {
        self.pcsystem_type.as_ref()
    }

    /// Sets the value of PCSystemTypeEx
    pub fn set_pcsystem_type_ex(&mut self, value: u16) {
        self.pcsystem_type_ex = Some(value);
    }

    /// Gets the value of PCSystemTypeEx
    pub fn get_pcsystem_type_ex(&self) -> Option<&u16> {
        self.pcsystem_type_ex.as_ref()
    }

    /// Sets the value of PowerOnPasswordStatus
    pub fn set_power_on_password_status(&mut self, value: u16) {
        self.power_on_password_status = Some(value);
    }

    /// Gets the value of PowerOnPasswordStatus
    pub fn get_power_on_password_status(&self) -> Option<&u16> {
        self.power_on_password_status.as_ref()
    }

    /// Sets the value of PowerSupplyState
    pub fn set_power_supply_state(&mut self, value: u16) {
        self.power_supply_state = Some(value);
    }

    /// Gets the value of PowerSupplyState
    pub fn get_power_supply_state(&self) -> Option<&u16> {
        self.power_supply_state.as_ref()
    }

    /// Sets the value of ResetCount
    pub fn set_reset_count(&mut self, value: i16) {
        self.reset_count = Some(value);
    }

    /// Gets the value of ResetCount
    pub fn get_reset_count(&self) -> Option<&i16> {
        self.reset_count.as_ref()
    }

    /// Sets the value of ResetLimit
    pub fn set_reset_limit(&mut self, value: i16) {
        self.reset_limit = Some(value);
    }

    /// Gets the value of ResetLimit
    pub fn get_reset_limit(&self) -> Option<&i16> {
        self.reset_limit.as_ref()
    }

    /// Sets the value of SupportContactDescription
    pub fn set_support_contact_description(&mut self, value: Vec<String>) {
        self.support_contact_description = value;
    }

    /// Gets the value of SupportContactDescription
    pub fn get_support_contact_description(&self) -> &Vec<String> {
        &self.support_contact_description
    }

    /// Sets the value of SystemFamily
    pub fn set_system_family(&mut self, value: String) {
        self.system_family = Some(value);
    }

    /// Gets the value of SystemFamily
    pub fn get_system_family(&self) -> Option<&String> {
        self.system_family.as_ref()
    }

    /// Sets the value of SystemSKUNumber
    pub fn set_system_skunumber(&mut self, value: String) {
        self.system_skunumber = Some(value);
    }

    /// Gets the value of SystemSKUNumber
    pub fn get_system_skunumber(&self) -> Option<&String> {
        self.system_skunumber.as_ref()
    }

    /// Sets the value of SystemStartupDelay
    pub fn set_system_startup_delay(&mut self, value: u16) {
        self.system_startup_delay = Some(value);
    }

    /// Gets the value of SystemStartupDelay
    pub fn get_system_startup_delay(&self) -> Option<&u16> {
        self.system_startup_delay.as_ref()
    }

    /// Sets the value of SystemStartupOptions
    pub fn set_system_startup_options(&mut self, value: Vec<String>) {
        self.system_startup_options = value;
    }

    /// Gets the value of SystemStartupOptions
    pub fn get_system_startup_options(&self) -> &Vec<String> {
        &self.system_startup_options
    }

    /// Sets the value of SystemStartupSetting
    pub fn set_system_startup_setting(&mut self, value: u8) {
        self.system_startup_setting = Some(value);
    }

    /// Gets the value of SystemStartupSetting
    pub fn get_system_startup_setting(&self) -> Option<&u8> {
        self.system_startup_setting.as_ref()
    }

    /// Sets the value of SystemType
    pub fn set_system_type(&mut self, value: String) {
        self.system_type = Some(value);
    }

    /// Gets the value of SystemType
    pub fn get_system_type(&self) -> Option<&String> {
        self.system_type.as_ref()
    }

    /// Sets the value of ThermalState
    pub fn set_thermal_state(&mut self, value: u16) {
        self.thermal_state = Some(value);
    }

    /// Gets the value of ThermalState
    pub fn get_thermal_state(&self) -> Option<&u16> {
        self.thermal_state.as_ref()
    }

    /// Sets the value of TotalPhysicalMemory
    pub fn set_total_physical_memory(&mut self, value: u64) {
        self.total_physical_memory = Some(value);
    }

    /// Gets the value of TotalPhysicalMemory
    pub fn get_total_physical_memory(&self) -> Option<&u64> {
        self.total_physical_memory.as_ref()
    }

    /// Sets the value of UserName
    pub fn set_user_name(&mut self, value: String) {
        self.user_name = Some(value);
    }

    /// Gets the value of UserName
    pub fn get_user_name(&self) -> Option<&String> {
        self.user_name.as_ref()
    }

    /// Sets the value of WakeUpType
    pub fn set_wake_up_type(&mut self, value: u16) {
        self.wake_up_type = Some(value);
    }

    /// Gets the value of WakeUpType
    pub fn get_wake_up_type(&self) -> Option<&u16> {
        self.wake_up_type.as_ref()
    }

    /// Sets the value of Workgroup
    pub fn set_workgroup(&mut self, value: String) {
        self.workgroup = Some(value);
    }

    /// Gets the value of Workgroup
    pub fn get_workgroup(&self) -> Option<&String> {
        self.workgroup.as_ref()
    }

/// 

    /// * `name` -  (String)
    /// * `password` -  (String)
    /// * `user_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn rename(&self, name: &String, password: &String, user_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "Password".to_string(), value: password.into() });
        args.push(MethodParameter { name: "UserName".to_string(), value: user_name.into() });
        self.invoke_method("Rename", &args)

    }


/// 

    /// * `account_ou` -  (String)
    /// * `fjoin_options` -  (u32)
    /// * `name` -  (String)
    /// * `password` -  (String)
    /// * `user_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn join_domain_or_workgroup(&self, name: &String, password: &String, user_name: &String, account_ou: &String, fjoin_options: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "Password".to_string(), value: password.into() });
        args.push(MethodParameter { name: "UserName".to_string(), value: user_name.into() });
        args.push(MethodParameter { name: "AccountOU".to_string(), value: account_ou.into() });
        args.push(MethodParameter { name: "FJoinOptions".to_string(), value: fjoin_options.into() });
        self.invoke_method("JoinDomainOrWorkgroup", &args)

    }


/// 

    /// * `funjoin_options` -  (u32)
    /// * `password` -  (String)
    /// * `user_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn unjoin_domain_or_workgroup(&self, password: &String, user_name: &String, funjoin_options: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Password".to_string(), value: password.into() });
        args.push(MethodParameter { name: "UserName".to_string(), value: user_name.into() });
        args.push(MethodParameter { name: "FUnjoinOptions".to_string(), value: funjoin_options.into() });
        self.invoke_method("UnjoinDomainOrWorkgroup", &args)

    }

}

