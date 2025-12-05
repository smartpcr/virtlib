// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PrinterConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PrinterConfiguration {
    #[serde(flatten)]
    pub base: CIM_Setting,

/// The BitsPerPel property contains the number of bits per pixel for the output device Win32 printer.  This member is used by display drivers and not by printer drivers.
/// Example: 8.  
///  This property has been deprecated because it is not applicable to printers.  There is no replacement value.
    #[serde(rename = "BitsPerPel")]
    pub bits_per_pel: Option<u32>,

/// The Collate property specifies whether to collate the pages that are printed. To collate is to print out the entire document before printing the next copy, as opposed to printing out each page of the document the required number times. This property is ignored unless the printer driver indicates support for collation.
/// Values: TRUE or FALSE. If TRUE, the printer collates all documents.
    #[serde(rename = "Collate")]
    pub collate: Option<bool>,

/// The Color property indicates whether the document is to be printed in color or monochrome.  Some color printers have the capability to print using true black instead of a combination of Yellow, Cyan, and Magenta.  This usually creates darker and sharper text for documents.  This option is only useful for color printers that support true black printing.
    #[serde(rename = "Color")]
    pub color: Option<PrinterConfiguration_Color>,

/// The Copies property indicates the number of copies to be printed. The printer driver must support printing multi-page copies.
/// Example: 2
    #[serde(rename = "Copies")]
    pub copies: Option<u32>,

/// The DeviceName property specifies the friendly name of the printer.  This name is unique to the type of printer and may be truncated because of the limitations of the string from which it is derived.
/// Example PCL/HP LaserJet
    #[serde(rename = "DeviceName")]
    pub device_name: Option<String>,

/// The DisplayFlags property contains two bits of information about the display. This member communicates whether the display device is monochrome or colored, and interlaced or non-interlaced, by masking its value with the DM_GRAYSCALE and DM_INTERLACED masks respectively. 
/// This property has been deprecated because it is not applicable to printers.  There is no replacement value.
    #[serde(rename = "DisplayFlags")]
    pub display_flags: Option<u32>,

/// The DisplayFrequency property indicates the refresh frequency of the display The refresh frequency for a monitor is the number of times the screen is redrawn per second. 
/// This property has been deprecated because it is not applicable to printers.  There is no replacement value.
    #[serde(rename = "DisplayFrequency")]
    pub display_frequency: Option<u32>,

/// The DitherType property indicates the dither type of the printer.  This member can assume predefined values of 1 to 5, or driver-defined values from 6 to 256.  Line art dithering is a special dithering method that produces well defined borders between black, white, and gray scalings.  It is not suitable for images that include continuous graduations in intensity and hue such as scanned photographs.
    #[serde(rename = "DitherType")]
    pub dither_type: Option<PrinterConfiguration_DitherType>,

/// The DriverVersion property indicates the version number of the Win32 printer driver.  The version numbers are created and maintained by the driver manufacturer.
    #[serde(rename = "DriverVersion")]
    pub driver_version: Option<u32>,

/// The Duplex property indicates whether printing is done on one or both sides.
/// Values: TRUE or FALSE. If TRUE, printing is done on both sides.
    #[serde(rename = "Duplex")]
    pub duplex: Option<bool>,

/// The FormName property indicates the name of the form used for the print job.  This property is used only on Windows NT/Windows 2000 systems.
/// Example: Legal
    #[serde(rename = "FormName")]
    pub form_name: Option<String>,

/// The HorizontalResolution property indicates the print resolution along the X axis (width) of the print job. This value is only set when the PrintQuality property of this class is positive and is similar to the XResolution property.
    #[serde(rename = "HorizontalResolution")]
    pub horizontal_resolution: Option<u32>,

/// The ICMIntent (Image Color Matching Intent) property indicates the specific value of one of the three possible color matching methods (called intents) that should be used by default.  ICM applications establish intents by using the ICM functions.  This property can assume predefined values of 1 to 3, or driver-defined values from 4 to 256.  Non-ICM applications can use this value to determine how the printer handles color printing jobs.
    #[serde(rename = "ICMIntent")]
    pub icmintent: Option<PrinterConfiguration_ICMIntent>,

/// The ICMMethod (Image Color Matching Method) property specifies how ICM is handled.  For a non-ICM application, this property determines if ICM is enabled or disabled.  For ICM applications, the system examines this property to determine which part of the computer system handles ICM support. 
    #[serde(rename = "ICMMethod")]
    pub icmmethod: Option<PrinterConfiguration_ICMMethod>,

/// The LogPixels property contains the number of pixels per logical inch.  This member is valid only with devices that work with pixels (this excludes devices such as printers).
/// This property has been deprecated because it is not applicable to printers.  There is no replacement value.
    #[serde(rename = "LogPixels")]
    pub log_pixels: Option<u32>,

/// The MediaType property specifies the type of media being printed on. The property can be set to a predefined value or a driver-defined value greater than or equal to 256. For Windows 95 and later; Windows 2000.
    #[serde(rename = "MediaType")]
    pub media_type: Option<PrinterConfiguration_MediaType>,

/// The Name property indicates the name of the printer with which this configuration is associated.
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// The Orientation property indicates the printing orientation of the paper.
    #[serde(rename = "Orientation")]
    pub orientation: Option<PrinterConfiguration_Orientation>,

/// The PaperLength property indicates the length of the paper.
/// Example: 2794
    #[serde(rename = "PaperLength")]
    pub paper_length: Option<u32>,

/// The PaperSize property indicates the size of the paper.
/// Example: A4 or Letter
    #[serde(rename = "PaperSize")]
    pub paper_size: Option<String>,

/// The PaperWidth property indicates the width of the paper.
/// Example: 2159
    #[serde(rename = "PaperWidth")]
    pub paper_width: Option<u32>,

/// The PelsHeight property indicates the height of the displayable surface. 
/// This property has been deprecated because it is not applicable to printers.  There is no replacement value.
    #[serde(rename = "PelsHeight")]
    pub pels_height: Option<u32>,

/// The PelsWidth property indicates the width of the displayable surface.  
/// This property has been deprecated because it is not applicable to printers.  There is no replacement value.
    #[serde(rename = "PelsWidth")]
    pub pels_width: Option<u32>,

/// The PrintQuality property indicates one of four quality levels of the print job.  If a positive value is specified, the quality is measured in dots per inch.
/// Example: Draft
    #[serde(rename = "PrintQuality")]
    pub print_quality: Option<PrinterConfiguration_PrintQuality>,

/// The Scale property specifies the factor by which the printed output is to be scaled.  For example a scale of 75 reduces the print output to 3/4 its original height and width.
    #[serde(rename = "Scale")]
    pub scale: Option<u32>,

/// The SpecificationVersion property indicates the version number of the initialization data for the device associated with the Win32 printer.
    #[serde(rename = "SpecificationVersion")]
    pub specification_version: Option<u32>,

/// The TTOption property specifies how TrueType(r) fonts should be printed.  There are 3 possible values:
/// Bitmap -  Prints TrueType fonts as graphics. This is the default action for dot-matrix printers.
/// Download -  Downloads TrueType fonts as soft fonts. This is the default action for printers that use the Printer Control Language (PCL).
/// Substitute -  Substitutes device fonts for TrueType fonts. This is the default action for PostScript(r) printers.
    #[serde(rename = "TTOption")]
    pub ttoption: Option<PrinterConfiguration_TTOption>,

/// The VerticalResolution property indicates the print resolution along the Y axis (height) of the print job. This value is only set when the PrintQuality property of this class is positive, and is similar to the YResolution property.
    #[serde(rename = "VerticalResolution")]
    pub vertical_resolution: Option<u32>,

/// The XResolution property has been deprecated to theHorizontalResolution property.  Please refer to the description of that property.
    #[serde(rename = "XResolution")]
    pub xresolution: Option<u32>,

/// The YResolution property has been deprecated to theVerticalResolution property.  Please refer to the description of that property.
    #[serde(rename = "YResolution")]
    pub yresolution: Option<u32>,
}

impl Win32_PrinterConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Setting::new(),
            bits_per_pel: None,
            collate: None,
            color: None,
            copies: None,
            device_name: None,
            display_flags: None,
            display_frequency: None,
            dither_type: None,
            driver_version: None,
            duplex: None,
            form_name: None,
            horizontal_resolution: None,
            icmintent: None,
            icmmethod: None,
            log_pixels: None,
            media_type: None,
            name: None,
            orientation: None,
            paper_length: None,
            paper_size: None,
            paper_width: None,
            pels_height: None,
            pels_width: None,
            print_quality: None,
            scale: None,
            specification_version: None,
            ttoption: None,
            vertical_resolution: None,
            xresolution: None,
            yresolution: None,
        }
    }


    /// Sets the value of BitsPerPel
    pub fn set_bits_per_pel(&mut self, value: u32) {
        self.bits_per_pel = Some(value);
    }

    /// Gets the value of BitsPerPel
    pub fn get_bits_per_pel(&self) -> Option<&u32> {
        self.bits_per_pel.as_ref()
    }

    /// Sets the value of Collate
    pub fn set_collate(&mut self, value: bool) {
        self.collate = Some(value);
    }

    /// Gets the value of Collate
    pub fn get_collate(&self) -> Option<&bool> {
        self.collate.as_ref()
    }

    /// Sets the value of Color
    pub fn set_color(&mut self, value: PrinterConfiguration_Color) {
        self.color = Some(value);
    }

    /// Gets the value of Color
    pub fn get_color(&self) -> Option<&PrinterConfiguration_Color> {
        self.color.as_ref()
    }

    /// Sets the value of Copies
    pub fn set_copies(&mut self, value: u32) {
        self.copies = Some(value);
    }

    /// Gets the value of Copies
    pub fn get_copies(&self) -> Option<&u32> {
        self.copies.as_ref()
    }

    /// Sets the value of DeviceName
    pub fn set_device_name(&mut self, value: String) {
        self.device_name = Some(value);
    }

    /// Gets the value of DeviceName
    pub fn get_device_name(&self) -> Option<&String> {
        self.device_name.as_ref()
    }

    /// Sets the value of DisplayFlags
    pub fn set_display_flags(&mut self, value: u32) {
        self.display_flags = Some(value);
    }

    /// Gets the value of DisplayFlags
    pub fn get_display_flags(&self) -> Option<&u32> {
        self.display_flags.as_ref()
    }

    /// Sets the value of DisplayFrequency
    pub fn set_display_frequency(&mut self, value: u32) {
        self.display_frequency = Some(value);
    }

    /// Gets the value of DisplayFrequency
    pub fn get_display_frequency(&self) -> Option<&u32> {
        self.display_frequency.as_ref()
    }

    /// Sets the value of DitherType
    pub fn set_dither_type(&mut self, value: PrinterConfiguration_DitherType) {
        self.dither_type = Some(value);
    }

    /// Gets the value of DitherType
    pub fn get_dither_type(&self) -> Option<&PrinterConfiguration_DitherType> {
        self.dither_type.as_ref()
    }

    /// Sets the value of DriverVersion
    pub fn set_driver_version(&mut self, value: u32) {
        self.driver_version = Some(value);
    }

    /// Gets the value of DriverVersion
    pub fn get_driver_version(&self) -> Option<&u32> {
        self.driver_version.as_ref()
    }

    /// Sets the value of Duplex
    pub fn set_duplex(&mut self, value: bool) {
        self.duplex = Some(value);
    }

    /// Gets the value of Duplex
    pub fn get_duplex(&self) -> Option<&bool> {
        self.duplex.as_ref()
    }

    /// Sets the value of FormName
    pub fn set_form_name(&mut self, value: String) {
        self.form_name = Some(value);
    }

    /// Gets the value of FormName
    pub fn get_form_name(&self) -> Option<&String> {
        self.form_name.as_ref()
    }

    /// Sets the value of HorizontalResolution
    pub fn set_horizontal_resolution(&mut self, value: u32) {
        self.horizontal_resolution = Some(value);
    }

    /// Gets the value of HorizontalResolution
    pub fn get_horizontal_resolution(&self) -> Option<&u32> {
        self.horizontal_resolution.as_ref()
    }

    /// Sets the value of ICMIntent
    pub fn set_icmintent(&mut self, value: PrinterConfiguration_ICMIntent) {
        self.icmintent = Some(value);
    }

    /// Gets the value of ICMIntent
    pub fn get_icmintent(&self) -> Option<&PrinterConfiguration_ICMIntent> {
        self.icmintent.as_ref()
    }

    /// Sets the value of ICMMethod
    pub fn set_icmmethod(&mut self, value: PrinterConfiguration_ICMMethod) {
        self.icmmethod = Some(value);
    }

    /// Gets the value of ICMMethod
    pub fn get_icmmethod(&self) -> Option<&PrinterConfiguration_ICMMethod> {
        self.icmmethod.as_ref()
    }

    /// Sets the value of LogPixels
    pub fn set_log_pixels(&mut self, value: u32) {
        self.log_pixels = Some(value);
    }

    /// Gets the value of LogPixels
    pub fn get_log_pixels(&self) -> Option<&u32> {
        self.log_pixels.as_ref()
    }

    /// Sets the value of MediaType
    pub fn set_media_type(&mut self, value: PrinterConfiguration_MediaType) {
        self.media_type = Some(value);
    }

    /// Gets the value of MediaType
    pub fn get_media_type(&self) -> Option<&PrinterConfiguration_MediaType> {
        self.media_type.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of Orientation
    pub fn set_orientation(&mut self, value: PrinterConfiguration_Orientation) {
        self.orientation = Some(value);
    }

    /// Gets the value of Orientation
    pub fn get_orientation(&self) -> Option<&PrinterConfiguration_Orientation> {
        self.orientation.as_ref()
    }

    /// Sets the value of PaperLength
    pub fn set_paper_length(&mut self, value: u32) {
        self.paper_length = Some(value);
    }

    /// Gets the value of PaperLength
    pub fn get_paper_length(&self) -> Option<&u32> {
        self.paper_length.as_ref()
    }

    /// Sets the value of PaperSize
    pub fn set_paper_size(&mut self, value: String) {
        self.paper_size = Some(value);
    }

    /// Gets the value of PaperSize
    pub fn get_paper_size(&self) -> Option<&String> {
        self.paper_size.as_ref()
    }

    /// Sets the value of PaperWidth
    pub fn set_paper_width(&mut self, value: u32) {
        self.paper_width = Some(value);
    }

    /// Gets the value of PaperWidth
    pub fn get_paper_width(&self) -> Option<&u32> {
        self.paper_width.as_ref()
    }

    /// Sets the value of PelsHeight
    pub fn set_pels_height(&mut self, value: u32) {
        self.pels_height = Some(value);
    }

    /// Gets the value of PelsHeight
    pub fn get_pels_height(&self) -> Option<&u32> {
        self.pels_height.as_ref()
    }

    /// Sets the value of PelsWidth
    pub fn set_pels_width(&mut self, value: u32) {
        self.pels_width = Some(value);
    }

    /// Gets the value of PelsWidth
    pub fn get_pels_width(&self) -> Option<&u32> {
        self.pels_width.as_ref()
    }

    /// Sets the value of PrintQuality
    pub fn set_print_quality(&mut self, value: PrinterConfiguration_PrintQuality) {
        self.print_quality = Some(value);
    }

    /// Gets the value of PrintQuality
    pub fn get_print_quality(&self) -> Option<&PrinterConfiguration_PrintQuality> {
        self.print_quality.as_ref()
    }

    /// Sets the value of Scale
    pub fn set_scale(&mut self, value: u32) {
        self.scale = Some(value);
    }

    /// Gets the value of Scale
    pub fn get_scale(&self) -> Option<&u32> {
        self.scale.as_ref()
    }

    /// Sets the value of SpecificationVersion
    pub fn set_specification_version(&mut self, value: u32) {
        self.specification_version = Some(value);
    }

    /// Gets the value of SpecificationVersion
    pub fn get_specification_version(&self) -> Option<&u32> {
        self.specification_version.as_ref()
    }

    /// Sets the value of TTOption
    pub fn set_ttoption(&mut self, value: PrinterConfiguration_TTOption) {
        self.ttoption = Some(value);
    }

    /// Gets the value of TTOption
    pub fn get_ttoption(&self) -> Option<&PrinterConfiguration_TTOption> {
        self.ttoption.as_ref()
    }

    /// Sets the value of VerticalResolution
    pub fn set_vertical_resolution(&mut self, value: u32) {
        self.vertical_resolution = Some(value);
    }

    /// Gets the value of VerticalResolution
    pub fn get_vertical_resolution(&self) -> Option<&u32> {
        self.vertical_resolution.as_ref()
    }

    /// Sets the value of XResolution
    pub fn set_xresolution(&mut self, value: u32) {
        self.xresolution = Some(value);
    }

    /// Gets the value of XResolution
    pub fn get_xresolution(&self) -> Option<&u32> {
        self.xresolution.as_ref()
    }

    /// Sets the value of YResolution
    pub fn set_yresolution(&mut self, value: u32) {
        self.yresolution = Some(value);
    }

    /// Gets the value of YResolution
    pub fn get_yresolution(&self) -> Option<&u32> {
        self.yresolution.as_ref()
    }
}

