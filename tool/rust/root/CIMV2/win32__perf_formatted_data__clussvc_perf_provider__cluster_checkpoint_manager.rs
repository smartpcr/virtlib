// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_ClussvcPerfProvider_ClusterCheckpointManager struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_ClussvcPerfProvider_ClusterCheckpointManager {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "CryptoCheckpointsRestored")]
    pub crypto_checkpoints_restored: Option<u64>,

/// 
    #[serde(rename = "CryptoCheckpointsRestoredPersec")]
    pub crypto_checkpoints_restored_persec: Option<u64>,

/// 
    #[serde(rename = "CryptoCheckpointsSaved")]
    pub crypto_checkpoints_saved: Option<u64>,

/// 
    #[serde(rename = "CryptoCheckpointsSavedPersec")]
    pub crypto_checkpoints_saved_persec: Option<u64>,

/// 
    #[serde(rename = "RegistryCheckpointsRestored")]
    pub registry_checkpoints_restored: Option<u64>,

/// 
    #[serde(rename = "RegistryCheckpointsRestoredPersec")]
    pub registry_checkpoints_restored_persec: Option<u64>,

/// 
    #[serde(rename = "RegistryCheckpointsSaved")]
    pub registry_checkpoints_saved: Option<u64>,

/// 
    #[serde(rename = "RegistryCheckpointsSavedPersec")]
    pub registry_checkpoints_saved_persec: Option<u64>,
}

impl Win32_PerfFormattedData_ClussvcPerfProvider_ClusterCheckpointManager {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            crypto_checkpoints_restored: None,
            crypto_checkpoints_restored_persec: None,
            crypto_checkpoints_saved: None,
            crypto_checkpoints_saved_persec: None,
            registry_checkpoints_restored: None,
            registry_checkpoints_restored_persec: None,
            registry_checkpoints_saved: None,
            registry_checkpoints_saved_persec: None,
        }
    }


    /// Sets the value of CryptoCheckpointsRestored
    pub fn set_crypto_checkpoints_restored(&mut self, value: u64) {
        self.crypto_checkpoints_restored = Some(value);
    }

    /// Gets the value of CryptoCheckpointsRestored
    pub fn get_crypto_checkpoints_restored(&self) -> Option<&u64> {
        self.crypto_checkpoints_restored.as_ref()
    }

    /// Sets the value of CryptoCheckpointsRestoredPersec
    pub fn set_crypto_checkpoints_restored_persec(&mut self, value: u64) {
        self.crypto_checkpoints_restored_persec = Some(value);
    }

    /// Gets the value of CryptoCheckpointsRestoredPersec
    pub fn get_crypto_checkpoints_restored_persec(&self) -> Option<&u64> {
        self.crypto_checkpoints_restored_persec.as_ref()
    }

    /// Sets the value of CryptoCheckpointsSaved
    pub fn set_crypto_checkpoints_saved(&mut self, value: u64) {
        self.crypto_checkpoints_saved = Some(value);
    }

    /// Gets the value of CryptoCheckpointsSaved
    pub fn get_crypto_checkpoints_saved(&self) -> Option<&u64> {
        self.crypto_checkpoints_saved.as_ref()
    }

    /// Sets the value of CryptoCheckpointsSavedPersec
    pub fn set_crypto_checkpoints_saved_persec(&mut self, value: u64) {
        self.crypto_checkpoints_saved_persec = Some(value);
    }

    /// Gets the value of CryptoCheckpointsSavedPersec
    pub fn get_crypto_checkpoints_saved_persec(&self) -> Option<&u64> {
        self.crypto_checkpoints_saved_persec.as_ref()
    }

    /// Sets the value of RegistryCheckpointsRestored
    pub fn set_registry_checkpoints_restored(&mut self, value: u64) {
        self.registry_checkpoints_restored = Some(value);
    }

    /// Gets the value of RegistryCheckpointsRestored
    pub fn get_registry_checkpoints_restored(&self) -> Option<&u64> {
        self.registry_checkpoints_restored.as_ref()
    }

    /// Sets the value of RegistryCheckpointsRestoredPersec
    pub fn set_registry_checkpoints_restored_persec(&mut self, value: u64) {
        self.registry_checkpoints_restored_persec = Some(value);
    }

    /// Gets the value of RegistryCheckpointsRestoredPersec
    pub fn get_registry_checkpoints_restored_persec(&self) -> Option<&u64> {
        self.registry_checkpoints_restored_persec.as_ref()
    }

    /// Sets the value of RegistryCheckpointsSaved
    pub fn set_registry_checkpoints_saved(&mut self, value: u64) {
        self.registry_checkpoints_saved = Some(value);
    }

    /// Gets the value of RegistryCheckpointsSaved
    pub fn get_registry_checkpoints_saved(&self) -> Option<&u64> {
        self.registry_checkpoints_saved.as_ref()
    }

    /// Sets the value of RegistryCheckpointsSavedPersec
    pub fn set_registry_checkpoints_saved_persec(&mut self, value: u64) {
        self.registry_checkpoints_saved_persec = Some(value);
    }

    /// Gets the value of RegistryCheckpointsSavedPersec
    pub fn get_registry_checkpoints_saved_persec(&self) -> Option<&u64> {
        self.registry_checkpoints_saved_persec.as_ref()
    }
}

