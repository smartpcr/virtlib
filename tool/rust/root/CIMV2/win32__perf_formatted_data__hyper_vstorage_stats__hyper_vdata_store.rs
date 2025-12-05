// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_HyperVStorageStats_HyperVDataStore struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_HyperVStorageStats_HyperVDataStore {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "Cacheupdateoperationcount")]
    pub cacheupdateoperationcount: Option<u64>,

/// 
    #[serde(rename = "Cacheupdateoperationlatencymicroseconds")]
    pub cacheupdateoperationlatencymicroseconds: Option<u64>,

/// 
    #[serde(rename = "Commitbytecount")]
    pub commitbytecount: Option<u64>,

/// 
    #[serde(rename = "Commitbytelatencymicroseconds")]
    pub commitbytelatencymicroseconds: Option<u64>,

/// 
    #[serde(rename = "Commitcount")]
    pub commitcount: Option<u64>,

/// 
    #[serde(rename = "Commitoperationcount")]
    pub commitoperationcount: Option<u64>,

/// 
    #[serde(rename = "Commitoperationlatencymicroseconds")]
    pub commitoperationlatencymicroseconds: Option<u64>,

/// 
    #[serde(rename = "Compactoperationcount")]
    pub compactoperationcount: Option<u64>,

/// 
    #[serde(rename = "Compactoperationlatencymicroseconds")]
    pub compactoperationlatencymicroseconds: Option<u64>,

/// 
    #[serde(rename = "CurrentreplaylogSize")]
    pub currentreplaylog_size: Option<u64>,

/// 
    #[serde(rename = "Dataalignment")]
    pub dataalignment: Option<u64>,

/// 
    #[serde(rename = "Dataend")]
    pub dataend: Option<u64>,

/// 
    #[serde(rename = "Disconnectcount")]
    pub disconnectcount: Option<u64>,

/// 
    #[serde(rename = "Filedatasizeinbytes")]
    pub filedatasizeinbytes: Option<u64>,

/// 
    #[serde(rename = "Filelockacquirelatencymicroseconds")]
    pub filelockacquirelatencymicroseconds: Option<u64>,

/// 
    #[serde(rename = "Filelockcount")]
    pub filelockcount: Option<u64>,

/// 
    #[serde(rename = "Filelockreleaselatencymicroseconds")]
    pub filelockreleaselatencymicroseconds: Option<u64>,

/// 
    #[serde(rename = "Fragmentationratio")]
    pub fragmentationratio: Option<u64>,

/// 
    #[serde(rename = "Getoperationcount")]
    pub getoperationcount: Option<u64>,

/// 
    #[serde(rename = "Getoperationlatencymicroseconds")]
    pub getoperationlatencymicroseconds: Option<u64>,

/// 
    #[serde(rename = "Loadfileoperationcount")]
    pub loadfileoperationcount: Option<u64>,

/// 
    #[serde(rename = "Loadfileoperationlatencymicroseconds")]
    pub loadfileoperationlatencymicroseconds: Option<u64>,

/// 
    #[serde(rename = "Namessizeinbytes")]
    pub namessizeinbytes: Option<u64>,

/// 
    #[serde(rename = "Numberofavailableentriesinsideobjecttables")]
    pub numberofavailableentriesinsideobjecttables: Option<u64>,

/// 
    #[serde(rename = "Numberofemptyentriesinsideobjecttables")]
    pub numberofemptyentriesinsideobjecttables: Option<u64>,

/// 
    #[serde(rename = "Numberoffileobjects")]
    pub numberoffileobjects: Option<u64>,

/// 
    #[serde(rename = "Numberoffreebytesinsidekeytables")]
    pub numberoffreebytesinsidekeytables: Option<u64>,

/// 
    #[serde(rename = "Numberofkeys")]
    pub numberofkeys: Option<u64>,

/// 
    #[serde(rename = "Numberofkeytables")]
    pub numberofkeytables: Option<u64>,

/// 
    #[serde(rename = "Numberofobjecttables")]
    pub numberofobjecttables: Option<u64>,

/// 
    #[serde(rename = "Querysizeoperationcount")]
    pub querysizeoperationcount: Option<u64>,

/// 
    #[serde(rename = "Querysizeoperationlatencymicroseconds")]
    pub querysizeoperationlatencymicroseconds: Option<u64>,

/// 
    #[serde(rename = "Readfromfilebytecount")]
    pub readfromfilebytecount: Option<u64>,

/// 
    #[serde(rename = "Readfromfilebytelatencymicroseconds")]
    pub readfromfilebytelatencymicroseconds: Option<u64>,

/// 
    #[serde(rename = "Readfromfilecount")]
    pub readfromfilecount: Option<u64>,

/// 
    #[serde(rename = "Readfromstoragebytecount")]
    pub readfromstoragebytecount: Option<u64>,

/// 
    #[serde(rename = "Readfromstoragebytelatencymicroseconds")]
    pub readfromstoragebytelatencymicroseconds: Option<u64>,

/// 
    #[serde(rename = "Readfromstoragecount")]
    pub readfromstoragecount: Option<u64>,

/// 
    #[serde(rename = "Reconnectlatencymicroseconds")]
    pub reconnectlatencymicroseconds: Option<u64>,

/// 
    #[serde(rename = "Removeoperationcount")]
    pub removeoperationcount: Option<u64>,

/// 
    #[serde(rename = "Removeoperationlatencymicroseconds")]
    pub removeoperationlatencymicroseconds: Option<u64>,

/// 
    #[serde(rename = "Sectorsize")]
    pub sectorsize: Option<u64>,

/// 
    #[serde(rename = "Setoperationcount")]
    pub setoperationcount: Option<u64>,

/// 
    #[serde(rename = "Setoperationlatencymicroseconds")]
    pub setoperationlatencymicroseconds: Option<u64>,

/// 
    #[serde(rename = "Storagelockacquirelatencymicroseconds")]
    pub storagelockacquirelatencymicroseconds: Option<u64>,

/// 
    #[serde(rename = "Storagelockcount")]
    pub storagelockcount: Option<u64>,

/// 
    #[serde(rename = "Storagelockreleaselatencymicroseconds")]
    pub storagelockreleaselatencymicroseconds: Option<u64>,

/// 
    #[serde(rename = "Tabledatasizeinbytes")]
    pub tabledatasizeinbytes: Option<u64>,

/// 
    #[serde(rename = "Writetofilebytecount")]
    pub writetofilebytecount: Option<u64>,

/// 
    #[serde(rename = "Writetofilebytelatencymicroseconds")]
    pub writetofilebytelatencymicroseconds: Option<u64>,

/// 
    #[serde(rename = "Writetofilecount")]
    pub writetofilecount: Option<u64>,

/// 
    #[serde(rename = "Writetostoragebytecount")]
    pub writetostoragebytecount: Option<u64>,

/// 
    #[serde(rename = "Writetostoragebytelatencymicroseconds")]
    pub writetostoragebytelatencymicroseconds: Option<u64>,

/// 
    #[serde(rename = "Writetostoragecount")]
    pub writetostoragecount: Option<u64>,
}

impl Win32_PerfFormattedData_HyperVStorageStats_HyperVDataStore {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            cacheupdateoperationcount: None,
            cacheupdateoperationlatencymicroseconds: None,
            commitbytecount: None,
            commitbytelatencymicroseconds: None,
            commitcount: None,
            commitoperationcount: None,
            commitoperationlatencymicroseconds: None,
            compactoperationcount: None,
            compactoperationlatencymicroseconds: None,
            currentreplaylog_size: None,
            dataalignment: None,
            dataend: None,
            disconnectcount: None,
            filedatasizeinbytes: None,
            filelockacquirelatencymicroseconds: None,
            filelockcount: None,
            filelockreleaselatencymicroseconds: None,
            fragmentationratio: None,
            getoperationcount: None,
            getoperationlatencymicroseconds: None,
            loadfileoperationcount: None,
            loadfileoperationlatencymicroseconds: None,
            namessizeinbytes: None,
            numberofavailableentriesinsideobjecttables: None,
            numberofemptyentriesinsideobjecttables: None,
            numberoffileobjects: None,
            numberoffreebytesinsidekeytables: None,
            numberofkeys: None,
            numberofkeytables: None,
            numberofobjecttables: None,
            querysizeoperationcount: None,
            querysizeoperationlatencymicroseconds: None,
            readfromfilebytecount: None,
            readfromfilebytelatencymicroseconds: None,
            readfromfilecount: None,
            readfromstoragebytecount: None,
            readfromstoragebytelatencymicroseconds: None,
            readfromstoragecount: None,
            reconnectlatencymicroseconds: None,
            removeoperationcount: None,
            removeoperationlatencymicroseconds: None,
            sectorsize: None,
            setoperationcount: None,
            setoperationlatencymicroseconds: None,
            storagelockacquirelatencymicroseconds: None,
            storagelockcount: None,
            storagelockreleaselatencymicroseconds: None,
            tabledatasizeinbytes: None,
            writetofilebytecount: None,
            writetofilebytelatencymicroseconds: None,
            writetofilecount: None,
            writetostoragebytecount: None,
            writetostoragebytelatencymicroseconds: None,
            writetostoragecount: None,
        }
    }


    /// Sets the value of Cacheupdateoperationcount
    pub fn set_cacheupdateoperationcount(&mut self, value: u64) {
        self.cacheupdateoperationcount = Some(value);
    }

    /// Gets the value of Cacheupdateoperationcount
    pub fn get_cacheupdateoperationcount(&self) -> Option<&u64> {
        self.cacheupdateoperationcount.as_ref()
    }

    /// Sets the value of Cacheupdateoperationlatencymicroseconds
    pub fn set_cacheupdateoperationlatencymicroseconds(&mut self, value: u64) {
        self.cacheupdateoperationlatencymicroseconds = Some(value);
    }

    /// Gets the value of Cacheupdateoperationlatencymicroseconds
    pub fn get_cacheupdateoperationlatencymicroseconds(&self) -> Option<&u64> {
        self.cacheupdateoperationlatencymicroseconds.as_ref()
    }

    /// Sets the value of Commitbytecount
    pub fn set_commitbytecount(&mut self, value: u64) {
        self.commitbytecount = Some(value);
    }

    /// Gets the value of Commitbytecount
    pub fn get_commitbytecount(&self) -> Option<&u64> {
        self.commitbytecount.as_ref()
    }

    /// Sets the value of Commitbytelatencymicroseconds
    pub fn set_commitbytelatencymicroseconds(&mut self, value: u64) {
        self.commitbytelatencymicroseconds = Some(value);
    }

    /// Gets the value of Commitbytelatencymicroseconds
    pub fn get_commitbytelatencymicroseconds(&self) -> Option<&u64> {
        self.commitbytelatencymicroseconds.as_ref()
    }

    /// Sets the value of Commitcount
    pub fn set_commitcount(&mut self, value: u64) {
        self.commitcount = Some(value);
    }

    /// Gets the value of Commitcount
    pub fn get_commitcount(&self) -> Option<&u64> {
        self.commitcount.as_ref()
    }

    /// Sets the value of Commitoperationcount
    pub fn set_commitoperationcount(&mut self, value: u64) {
        self.commitoperationcount = Some(value);
    }

    /// Gets the value of Commitoperationcount
    pub fn get_commitoperationcount(&self) -> Option<&u64> {
        self.commitoperationcount.as_ref()
    }

    /// Sets the value of Commitoperationlatencymicroseconds
    pub fn set_commitoperationlatencymicroseconds(&mut self, value: u64) {
        self.commitoperationlatencymicroseconds = Some(value);
    }

    /// Gets the value of Commitoperationlatencymicroseconds
    pub fn get_commitoperationlatencymicroseconds(&self) -> Option<&u64> {
        self.commitoperationlatencymicroseconds.as_ref()
    }

    /// Sets the value of Compactoperationcount
    pub fn set_compactoperationcount(&mut self, value: u64) {
        self.compactoperationcount = Some(value);
    }

    /// Gets the value of Compactoperationcount
    pub fn get_compactoperationcount(&self) -> Option<&u64> {
        self.compactoperationcount.as_ref()
    }

    /// Sets the value of Compactoperationlatencymicroseconds
    pub fn set_compactoperationlatencymicroseconds(&mut self, value: u64) {
        self.compactoperationlatencymicroseconds = Some(value);
    }

    /// Gets the value of Compactoperationlatencymicroseconds
    pub fn get_compactoperationlatencymicroseconds(&self) -> Option<&u64> {
        self.compactoperationlatencymicroseconds.as_ref()
    }

    /// Sets the value of CurrentreplaylogSize
    pub fn set_currentreplaylog_size(&mut self, value: u64) {
        self.currentreplaylog_size = Some(value);
    }

    /// Gets the value of CurrentreplaylogSize
    pub fn get_currentreplaylog_size(&self) -> Option<&u64> {
        self.currentreplaylog_size.as_ref()
    }

    /// Sets the value of Dataalignment
    pub fn set_dataalignment(&mut self, value: u64) {
        self.dataalignment = Some(value);
    }

    /// Gets the value of Dataalignment
    pub fn get_dataalignment(&self) -> Option<&u64> {
        self.dataalignment.as_ref()
    }

    /// Sets the value of Dataend
    pub fn set_dataend(&mut self, value: u64) {
        self.dataend = Some(value);
    }

    /// Gets the value of Dataend
    pub fn get_dataend(&self) -> Option<&u64> {
        self.dataend.as_ref()
    }

    /// Sets the value of Disconnectcount
    pub fn set_disconnectcount(&mut self, value: u64) {
        self.disconnectcount = Some(value);
    }

    /// Gets the value of Disconnectcount
    pub fn get_disconnectcount(&self) -> Option<&u64> {
        self.disconnectcount.as_ref()
    }

    /// Sets the value of Filedatasizeinbytes
    pub fn set_filedatasizeinbytes(&mut self, value: u64) {
        self.filedatasizeinbytes = Some(value);
    }

    /// Gets the value of Filedatasizeinbytes
    pub fn get_filedatasizeinbytes(&self) -> Option<&u64> {
        self.filedatasizeinbytes.as_ref()
    }

    /// Sets the value of Filelockacquirelatencymicroseconds
    pub fn set_filelockacquirelatencymicroseconds(&mut self, value: u64) {
        self.filelockacquirelatencymicroseconds = Some(value);
    }

    /// Gets the value of Filelockacquirelatencymicroseconds
    pub fn get_filelockacquirelatencymicroseconds(&self) -> Option<&u64> {
        self.filelockacquirelatencymicroseconds.as_ref()
    }

    /// Sets the value of Filelockcount
    pub fn set_filelockcount(&mut self, value: u64) {
        self.filelockcount = Some(value);
    }

    /// Gets the value of Filelockcount
    pub fn get_filelockcount(&self) -> Option<&u64> {
        self.filelockcount.as_ref()
    }

    /// Sets the value of Filelockreleaselatencymicroseconds
    pub fn set_filelockreleaselatencymicroseconds(&mut self, value: u64) {
        self.filelockreleaselatencymicroseconds = Some(value);
    }

    /// Gets the value of Filelockreleaselatencymicroseconds
    pub fn get_filelockreleaselatencymicroseconds(&self) -> Option<&u64> {
        self.filelockreleaselatencymicroseconds.as_ref()
    }

    /// Sets the value of Fragmentationratio
    pub fn set_fragmentationratio(&mut self, value: u64) {
        self.fragmentationratio = Some(value);
    }

    /// Gets the value of Fragmentationratio
    pub fn get_fragmentationratio(&self) -> Option<&u64> {
        self.fragmentationratio.as_ref()
    }

    /// Sets the value of Getoperationcount
    pub fn set_getoperationcount(&mut self, value: u64) {
        self.getoperationcount = Some(value);
    }

    /// Gets the value of Getoperationcount
    pub fn get_getoperationcount(&self) -> Option<&u64> {
        self.getoperationcount.as_ref()
    }

    /// Sets the value of Getoperationlatencymicroseconds
    pub fn set_getoperationlatencymicroseconds(&mut self, value: u64) {
        self.getoperationlatencymicroseconds = Some(value);
    }

    /// Gets the value of Getoperationlatencymicroseconds
    pub fn get_getoperationlatencymicroseconds(&self) -> Option<&u64> {
        self.getoperationlatencymicroseconds.as_ref()
    }

    /// Sets the value of Loadfileoperationcount
    pub fn set_loadfileoperationcount(&mut self, value: u64) {
        self.loadfileoperationcount = Some(value);
    }

    /// Gets the value of Loadfileoperationcount
    pub fn get_loadfileoperationcount(&self) -> Option<&u64> {
        self.loadfileoperationcount.as_ref()
    }

    /// Sets the value of Loadfileoperationlatencymicroseconds
    pub fn set_loadfileoperationlatencymicroseconds(&mut self, value: u64) {
        self.loadfileoperationlatencymicroseconds = Some(value);
    }

    /// Gets the value of Loadfileoperationlatencymicroseconds
    pub fn get_loadfileoperationlatencymicroseconds(&self) -> Option<&u64> {
        self.loadfileoperationlatencymicroseconds.as_ref()
    }

    /// Sets the value of Namessizeinbytes
    pub fn set_namessizeinbytes(&mut self, value: u64) {
        self.namessizeinbytes = Some(value);
    }

    /// Gets the value of Namessizeinbytes
    pub fn get_namessizeinbytes(&self) -> Option<&u64> {
        self.namessizeinbytes.as_ref()
    }

    /// Sets the value of Numberofavailableentriesinsideobjecttables
    pub fn set_numberofavailableentriesinsideobjecttables(&mut self, value: u64) {
        self.numberofavailableentriesinsideobjecttables = Some(value);
    }

    /// Gets the value of Numberofavailableentriesinsideobjecttables
    pub fn get_numberofavailableentriesinsideobjecttables(&self) -> Option<&u64> {
        self.numberofavailableentriesinsideobjecttables.as_ref()
    }

    /// Sets the value of Numberofemptyentriesinsideobjecttables
    pub fn set_numberofemptyentriesinsideobjecttables(&mut self, value: u64) {
        self.numberofemptyentriesinsideobjecttables = Some(value);
    }

    /// Gets the value of Numberofemptyentriesinsideobjecttables
    pub fn get_numberofemptyentriesinsideobjecttables(&self) -> Option<&u64> {
        self.numberofemptyentriesinsideobjecttables.as_ref()
    }

    /// Sets the value of Numberoffileobjects
    pub fn set_numberoffileobjects(&mut self, value: u64) {
        self.numberoffileobjects = Some(value);
    }

    /// Gets the value of Numberoffileobjects
    pub fn get_numberoffileobjects(&self) -> Option<&u64> {
        self.numberoffileobjects.as_ref()
    }

    /// Sets the value of Numberoffreebytesinsidekeytables
    pub fn set_numberoffreebytesinsidekeytables(&mut self, value: u64) {
        self.numberoffreebytesinsidekeytables = Some(value);
    }

    /// Gets the value of Numberoffreebytesinsidekeytables
    pub fn get_numberoffreebytesinsidekeytables(&self) -> Option<&u64> {
        self.numberoffreebytesinsidekeytables.as_ref()
    }

    /// Sets the value of Numberofkeys
    pub fn set_numberofkeys(&mut self, value: u64) {
        self.numberofkeys = Some(value);
    }

    /// Gets the value of Numberofkeys
    pub fn get_numberofkeys(&self) -> Option<&u64> {
        self.numberofkeys.as_ref()
    }

    /// Sets the value of Numberofkeytables
    pub fn set_numberofkeytables(&mut self, value: u64) {
        self.numberofkeytables = Some(value);
    }

    /// Gets the value of Numberofkeytables
    pub fn get_numberofkeytables(&self) -> Option<&u64> {
        self.numberofkeytables.as_ref()
    }

    /// Sets the value of Numberofobjecttables
    pub fn set_numberofobjecttables(&mut self, value: u64) {
        self.numberofobjecttables = Some(value);
    }

    /// Gets the value of Numberofobjecttables
    pub fn get_numberofobjecttables(&self) -> Option<&u64> {
        self.numberofobjecttables.as_ref()
    }

    /// Sets the value of Querysizeoperationcount
    pub fn set_querysizeoperationcount(&mut self, value: u64) {
        self.querysizeoperationcount = Some(value);
    }

    /// Gets the value of Querysizeoperationcount
    pub fn get_querysizeoperationcount(&self) -> Option<&u64> {
        self.querysizeoperationcount.as_ref()
    }

    /// Sets the value of Querysizeoperationlatencymicroseconds
    pub fn set_querysizeoperationlatencymicroseconds(&mut self, value: u64) {
        self.querysizeoperationlatencymicroseconds = Some(value);
    }

    /// Gets the value of Querysizeoperationlatencymicroseconds
    pub fn get_querysizeoperationlatencymicroseconds(&self) -> Option<&u64> {
        self.querysizeoperationlatencymicroseconds.as_ref()
    }

    /// Sets the value of Readfromfilebytecount
    pub fn set_readfromfilebytecount(&mut self, value: u64) {
        self.readfromfilebytecount = Some(value);
    }

    /// Gets the value of Readfromfilebytecount
    pub fn get_readfromfilebytecount(&self) -> Option<&u64> {
        self.readfromfilebytecount.as_ref()
    }

    /// Sets the value of Readfromfilebytelatencymicroseconds
    pub fn set_readfromfilebytelatencymicroseconds(&mut self, value: u64) {
        self.readfromfilebytelatencymicroseconds = Some(value);
    }

    /// Gets the value of Readfromfilebytelatencymicroseconds
    pub fn get_readfromfilebytelatencymicroseconds(&self) -> Option<&u64> {
        self.readfromfilebytelatencymicroseconds.as_ref()
    }

    /// Sets the value of Readfromfilecount
    pub fn set_readfromfilecount(&mut self, value: u64) {
        self.readfromfilecount = Some(value);
    }

    /// Gets the value of Readfromfilecount
    pub fn get_readfromfilecount(&self) -> Option<&u64> {
        self.readfromfilecount.as_ref()
    }

    /// Sets the value of Readfromstoragebytecount
    pub fn set_readfromstoragebytecount(&mut self, value: u64) {
        self.readfromstoragebytecount = Some(value);
    }

    /// Gets the value of Readfromstoragebytecount
    pub fn get_readfromstoragebytecount(&self) -> Option<&u64> {
        self.readfromstoragebytecount.as_ref()
    }

    /// Sets the value of Readfromstoragebytelatencymicroseconds
    pub fn set_readfromstoragebytelatencymicroseconds(&mut self, value: u64) {
        self.readfromstoragebytelatencymicroseconds = Some(value);
    }

    /// Gets the value of Readfromstoragebytelatencymicroseconds
    pub fn get_readfromstoragebytelatencymicroseconds(&self) -> Option<&u64> {
        self.readfromstoragebytelatencymicroseconds.as_ref()
    }

    /// Sets the value of Readfromstoragecount
    pub fn set_readfromstoragecount(&mut self, value: u64) {
        self.readfromstoragecount = Some(value);
    }

    /// Gets the value of Readfromstoragecount
    pub fn get_readfromstoragecount(&self) -> Option<&u64> {
        self.readfromstoragecount.as_ref()
    }

    /// Sets the value of Reconnectlatencymicroseconds
    pub fn set_reconnectlatencymicroseconds(&mut self, value: u64) {
        self.reconnectlatencymicroseconds = Some(value);
    }

    /// Gets the value of Reconnectlatencymicroseconds
    pub fn get_reconnectlatencymicroseconds(&self) -> Option<&u64> {
        self.reconnectlatencymicroseconds.as_ref()
    }

    /// Sets the value of Removeoperationcount
    pub fn set_removeoperationcount(&mut self, value: u64) {
        self.removeoperationcount = Some(value);
    }

    /// Gets the value of Removeoperationcount
    pub fn get_removeoperationcount(&self) -> Option<&u64> {
        self.removeoperationcount.as_ref()
    }

    /// Sets the value of Removeoperationlatencymicroseconds
    pub fn set_removeoperationlatencymicroseconds(&mut self, value: u64) {
        self.removeoperationlatencymicroseconds = Some(value);
    }

    /// Gets the value of Removeoperationlatencymicroseconds
    pub fn get_removeoperationlatencymicroseconds(&self) -> Option<&u64> {
        self.removeoperationlatencymicroseconds.as_ref()
    }

    /// Sets the value of Sectorsize
    pub fn set_sectorsize(&mut self, value: u64) {
        self.sectorsize = Some(value);
    }

    /// Gets the value of Sectorsize
    pub fn get_sectorsize(&self) -> Option<&u64> {
        self.sectorsize.as_ref()
    }

    /// Sets the value of Setoperationcount
    pub fn set_setoperationcount(&mut self, value: u64) {
        self.setoperationcount = Some(value);
    }

    /// Gets the value of Setoperationcount
    pub fn get_setoperationcount(&self) -> Option<&u64> {
        self.setoperationcount.as_ref()
    }

    /// Sets the value of Setoperationlatencymicroseconds
    pub fn set_setoperationlatencymicroseconds(&mut self, value: u64) {
        self.setoperationlatencymicroseconds = Some(value);
    }

    /// Gets the value of Setoperationlatencymicroseconds
    pub fn get_setoperationlatencymicroseconds(&self) -> Option<&u64> {
        self.setoperationlatencymicroseconds.as_ref()
    }

    /// Sets the value of Storagelockacquirelatencymicroseconds
    pub fn set_storagelockacquirelatencymicroseconds(&mut self, value: u64) {
        self.storagelockacquirelatencymicroseconds = Some(value);
    }

    /// Gets the value of Storagelockacquirelatencymicroseconds
    pub fn get_storagelockacquirelatencymicroseconds(&self) -> Option<&u64> {
        self.storagelockacquirelatencymicroseconds.as_ref()
    }

    /// Sets the value of Storagelockcount
    pub fn set_storagelockcount(&mut self, value: u64) {
        self.storagelockcount = Some(value);
    }

    /// Gets the value of Storagelockcount
    pub fn get_storagelockcount(&self) -> Option<&u64> {
        self.storagelockcount.as_ref()
    }

    /// Sets the value of Storagelockreleaselatencymicroseconds
    pub fn set_storagelockreleaselatencymicroseconds(&mut self, value: u64) {
        self.storagelockreleaselatencymicroseconds = Some(value);
    }

    /// Gets the value of Storagelockreleaselatencymicroseconds
    pub fn get_storagelockreleaselatencymicroseconds(&self) -> Option<&u64> {
        self.storagelockreleaselatencymicroseconds.as_ref()
    }

    /// Sets the value of Tabledatasizeinbytes
    pub fn set_tabledatasizeinbytes(&mut self, value: u64) {
        self.tabledatasizeinbytes = Some(value);
    }

    /// Gets the value of Tabledatasizeinbytes
    pub fn get_tabledatasizeinbytes(&self) -> Option<&u64> {
        self.tabledatasizeinbytes.as_ref()
    }

    /// Sets the value of Writetofilebytecount
    pub fn set_writetofilebytecount(&mut self, value: u64) {
        self.writetofilebytecount = Some(value);
    }

    /// Gets the value of Writetofilebytecount
    pub fn get_writetofilebytecount(&self) -> Option<&u64> {
        self.writetofilebytecount.as_ref()
    }

    /// Sets the value of Writetofilebytelatencymicroseconds
    pub fn set_writetofilebytelatencymicroseconds(&mut self, value: u64) {
        self.writetofilebytelatencymicroseconds = Some(value);
    }

    /// Gets the value of Writetofilebytelatencymicroseconds
    pub fn get_writetofilebytelatencymicroseconds(&self) -> Option<&u64> {
        self.writetofilebytelatencymicroseconds.as_ref()
    }

    /// Sets the value of Writetofilecount
    pub fn set_writetofilecount(&mut self, value: u64) {
        self.writetofilecount = Some(value);
    }

    /// Gets the value of Writetofilecount
    pub fn get_writetofilecount(&self) -> Option<&u64> {
        self.writetofilecount.as_ref()
    }

    /// Sets the value of Writetostoragebytecount
    pub fn set_writetostoragebytecount(&mut self, value: u64) {
        self.writetostoragebytecount = Some(value);
    }

    /// Gets the value of Writetostoragebytecount
    pub fn get_writetostoragebytecount(&self) -> Option<&u64> {
        self.writetostoragebytecount.as_ref()
    }

    /// Sets the value of Writetostoragebytelatencymicroseconds
    pub fn set_writetostoragebytelatencymicroseconds(&mut self, value: u64) {
        self.writetostoragebytelatencymicroseconds = Some(value);
    }

    /// Gets the value of Writetostoragebytelatencymicroseconds
    pub fn get_writetostoragebytelatencymicroseconds(&self) -> Option<&u64> {
        self.writetostoragebytelatencymicroseconds.as_ref()
    }

    /// Sets the value of Writetostoragecount
    pub fn set_writetostoragecount(&mut self, value: u64) {
        self.writetostoragecount = Some(value);
    }

    /// Gets the value of Writetostoragecount
    pub fn get_writetostoragecount(&self) -> Option<&u64> {
        self.writetostoragecount.as_ref()
    }
}

