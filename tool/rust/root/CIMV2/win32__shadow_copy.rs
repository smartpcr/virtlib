// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ShadowCopy struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ShadowCopy {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// 
    #[serde(rename = "ClientAccessible")]
    pub client_accessible: Option<bool>,

/// 
    #[serde(rename = "Count")]
    pub count: Option<u32>,

/// 
    #[serde(rename = "DeviceObject")]
    pub device_object: Option<String>,

/// 
    #[serde(rename = "Differential")]
    pub differential: Option<bool>,

/// 
    #[serde(rename = "ExposedLocally")]
    pub exposed_locally: Option<bool>,

/// 
    #[serde(rename = "ExposedName")]
    pub exposed_name: Option<String>,

/// 
    #[serde(rename = "ExposedPath")]
    pub exposed_path: Option<String>,

/// 
    #[serde(rename = "ExposedRemotely")]
    pub exposed_remotely: Option<bool>,

/// 
    #[serde(rename = "HardwareAssisted")]
    pub hardware_assisted: Option<bool>,

/// 
    #[serde(rename = "ID")]
    pub id: Option<String>,

/// 
    #[serde(rename = "Imported")]
    pub imported: Option<bool>,

/// 
    #[serde(rename = "NoAutoRelease")]
    pub no_auto_release: Option<bool>,

/// 
    #[serde(rename = "NotSurfaced")]
    pub not_surfaced: Option<bool>,

/// 
    #[serde(rename = "NoWriters")]
    pub no_writers: Option<bool>,

/// 
    #[serde(rename = "OriginatingMachine")]
    pub originating_machine: Option<String>,

/// 
    #[serde(rename = "Persistent")]
    pub persistent: Option<bool>,

/// 
    #[serde(rename = "Plex")]
    pub plex: Option<bool>,

/// 
    #[serde(rename = "ProviderID")]
    pub provider_id: Option<String>,

/// 
    #[serde(rename = "ServiceMachine")]
    pub service_machine: Option<String>,

/// 
    #[serde(rename = "SetID")]
    pub set_id: Option<String>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u32>,

/// 
    #[serde(rename = "Transportable")]
    pub transportable: Option<bool>,

/// 
    #[serde(rename = "VolumeName")]
    pub volume_name: Option<String>,
}

impl Win32_ShadowCopy {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            client_accessible: None,
            count: None,
            device_object: None,
            differential: None,
            exposed_locally: None,
            exposed_name: None,
            exposed_path: None,
            exposed_remotely: None,
            hardware_assisted: None,
            id: None,
            imported: None,
            no_auto_release: None,
            not_surfaced: None,
            no_writers: None,
            originating_machine: None,
            persistent: None,
            plex: None,
            provider_id: None,
            service_machine: None,
            set_id: None,
            state: None,
            transportable: None,
            volume_name: None,
        }
    }


    /// Sets the value of ClientAccessible
    pub fn set_client_accessible(&mut self, value: bool) {
        self.client_accessible = Some(value);
    }

    /// Gets the value of ClientAccessible
    pub fn get_client_accessible(&self) -> Option<&bool> {
        self.client_accessible.as_ref()
    }

    /// Sets the value of Count
    pub fn set_count(&mut self, value: u32) {
        self.count = Some(value);
    }

    /// Gets the value of Count
    pub fn get_count(&self) -> Option<&u32> {
        self.count.as_ref()
    }

    /// Sets the value of DeviceObject
    pub fn set_device_object(&mut self, value: String) {
        self.device_object = Some(value);
    }

    /// Gets the value of DeviceObject
    pub fn get_device_object(&self) -> Option<&String> {
        self.device_object.as_ref()
    }

    /// Sets the value of Differential
    pub fn set_differential(&mut self, value: bool) {
        self.differential = Some(value);
    }

    /// Gets the value of Differential
    pub fn get_differential(&self) -> Option<&bool> {
        self.differential.as_ref()
    }

    /// Sets the value of ExposedLocally
    pub fn set_exposed_locally(&mut self, value: bool) {
        self.exposed_locally = Some(value);
    }

    /// Gets the value of ExposedLocally
    pub fn get_exposed_locally(&self) -> Option<&bool> {
        self.exposed_locally.as_ref()
    }

    /// Sets the value of ExposedName
    pub fn set_exposed_name(&mut self, value: String) {
        self.exposed_name = Some(value);
    }

    /// Gets the value of ExposedName
    pub fn get_exposed_name(&self) -> Option<&String> {
        self.exposed_name.as_ref()
    }

    /// Sets the value of ExposedPath
    pub fn set_exposed_path(&mut self, value: String) {
        self.exposed_path = Some(value);
    }

    /// Gets the value of ExposedPath
    pub fn get_exposed_path(&self) -> Option<&String> {
        self.exposed_path.as_ref()
    }

    /// Sets the value of ExposedRemotely
    pub fn set_exposed_remotely(&mut self, value: bool) {
        self.exposed_remotely = Some(value);
    }

    /// Gets the value of ExposedRemotely
    pub fn get_exposed_remotely(&self) -> Option<&bool> {
        self.exposed_remotely.as_ref()
    }

    /// Sets the value of HardwareAssisted
    pub fn set_hardware_assisted(&mut self, value: bool) {
        self.hardware_assisted = Some(value);
    }

    /// Gets the value of HardwareAssisted
    pub fn get_hardware_assisted(&self) -> Option<&bool> {
        self.hardware_assisted.as_ref()
    }

    /// Sets the value of ID
    pub fn set_id(&mut self, value: String) {
        self.id = Some(value);
    }

    /// Gets the value of ID
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    /// Sets the value of Imported
    pub fn set_imported(&mut self, value: bool) {
        self.imported = Some(value);
    }

    /// Gets the value of Imported
    pub fn get_imported(&self) -> Option<&bool> {
        self.imported.as_ref()
    }

    /// Sets the value of NoAutoRelease
    pub fn set_no_auto_release(&mut self, value: bool) {
        self.no_auto_release = Some(value);
    }

    /// Gets the value of NoAutoRelease
    pub fn get_no_auto_release(&self) -> Option<&bool> {
        self.no_auto_release.as_ref()
    }

    /// Sets the value of NotSurfaced
    pub fn set_not_surfaced(&mut self, value: bool) {
        self.not_surfaced = Some(value);
    }

    /// Gets the value of NotSurfaced
    pub fn get_not_surfaced(&self) -> Option<&bool> {
        self.not_surfaced.as_ref()
    }

    /// Sets the value of NoWriters
    pub fn set_no_writers(&mut self, value: bool) {
        self.no_writers = Some(value);
    }

    /// Gets the value of NoWriters
    pub fn get_no_writers(&self) -> Option<&bool> {
        self.no_writers.as_ref()
    }

    /// Sets the value of OriginatingMachine
    pub fn set_originating_machine(&mut self, value: String) {
        self.originating_machine = Some(value);
    }

    /// Gets the value of OriginatingMachine
    pub fn get_originating_machine(&self) -> Option<&String> {
        self.originating_machine.as_ref()
    }

    /// Sets the value of Persistent
    pub fn set_persistent(&mut self, value: bool) {
        self.persistent = Some(value);
    }

    /// Gets the value of Persistent
    pub fn get_persistent(&self) -> Option<&bool> {
        self.persistent.as_ref()
    }

    /// Sets the value of Plex
    pub fn set_plex(&mut self, value: bool) {
        self.plex = Some(value);
    }

    /// Gets the value of Plex
    pub fn get_plex(&self) -> Option<&bool> {
        self.plex.as_ref()
    }

    /// Sets the value of ProviderID
    pub fn set_provider_id(&mut self, value: String) {
        self.provider_id = Some(value);
    }

    /// Gets the value of ProviderID
    pub fn get_provider_id(&self) -> Option<&String> {
        self.provider_id.as_ref()
    }

    /// Sets the value of ServiceMachine
    pub fn set_service_machine(&mut self, value: String) {
        self.service_machine = Some(value);
    }

    /// Gets the value of ServiceMachine
    pub fn get_service_machine(&self) -> Option<&String> {
        self.service_machine.as_ref()
    }

    /// Sets the value of SetID
    pub fn set_set_id(&mut self, value: String) {
        self.set_id = Some(value);
    }

    /// Gets the value of SetID
    pub fn get_set_id(&self) -> Option<&String> {
        self.set_id.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: u32) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&u32> {
        self.state.as_ref()
    }

    /// Sets the value of Transportable
    pub fn set_transportable(&mut self, value: bool) {
        self.transportable = Some(value);
    }

    /// Gets the value of Transportable
    pub fn get_transportable(&self) -> Option<&bool> {
        self.transportable.as_ref()
    }

    /// Sets the value of VolumeName
    pub fn set_volume_name(&mut self, value: String) {
        self.volume_name = Some(value);
    }

    /// Gets the value of VolumeName
    pub fn get_volume_name(&self) -> Option<&String> {
        self.volume_name.as_ref()
    }

/// 

    /// * `context` -  (String)
    /// * `volume` -  (String)

    /// * `return_value` -  (u32)
    /// * `shadow_id` -  (String)
    pub fn create(&self, volume: &String, context: &String, shadow_id: &mut String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Volume".to_string(), value: volume.into() });
        args.push(MethodParameter { name: "Context".to_string(), value: context.into() });

        let result = self.invoke_method("Create", &args)?;
        let shadow_id = result.get_value("ShadowID")?;
        Ok(result.return_value)

    }


/// 

    /// * `force_dismount` -  (bool)

    /// * `return_value` -  (u32)
    pub fn revert(&self, force_dismount: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ForceDismount".to_string(), value: force_dismount.into() });
        self.invoke_method("Revert", &args)

    }

}

