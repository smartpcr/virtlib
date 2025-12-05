// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.TerminalServices
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_TSGetIcon struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_TSGetIcon {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,
}

impl Win32_TSGetIcon {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
        }
    }


/// Returns the contents of the Icon in the filepath using the icon index

    /// * `file_path` - String that contains the path to the file that contains the icon (String)
    /// * `index` - Index for the Icon wanted (i32)

    /// * `icon_contents` - Contents of the Icon (u8[])
    /// * `return_value` -  (u32)
    pub fn get_icon(&self, file_path: &String, index: i32, icon_contents: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "FilePath".to_string(), value: file_path.into() });
        args.push(MethodParameter { name: "Index".to_string(), value: index.into() });

        let result = self.invoke_method("GetIcon", &args)?;
        let icon_contents = result.get_value("IconContents")?;
        Ok(result.return_value)

    }

}

