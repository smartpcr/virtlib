// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

using System;
using System.Globalization;
using System.IO;
using System.Management;
using System.Text;
using Microsoft.wmi.Common;

namespace Microsoft.wmi.Rust
{
    public class RustWmiSource : WmiSource
    {
        public RustWmiSource(string sourceName, WmiModule wModule) : base(sourceName, wModule)
        {
            HeaderComment = string.Format(CultureInfo.InvariantCulture,
@"
// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.
//
// Author:
//      Auto Generated on {2} using wmigen
//      Source {3}
//////////////////////////////////////////////",
                    Name,
                    "This class doesn't exist. This is just a placeholder",
                    DateTime.Now.ToShortDateString(),
                    Parent.Parent.CSNamespaceName
                    );
            AddReference("crate::wmi");
            AddReference("crate::cim");
            Class = new RustWmiClass(this);
        }

        public RustWmiSource(ManagementClass wmiClass, WmiModule wModule) : base(wmiClass, wModule)
        {
            CopyrightText = string.Format(CultureInfo.InvariantCulture,
@"// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.
");

            HeaderComment = string.Format(CultureInfo.InvariantCulture,
@"//
// Author:
//      Auto Generated on {2} using wmigen
//      Source {3}
//////////////////////////////////////////////",
                    Name,
                    IFormat.GetDescriptionText(wmiClass.Qualifiers),
                    DateTime.Now.ToShortDateString(),
                    Parent.Parent.CSNamespaceName
                    );

            AddReference("crate::wmi");
            AddReference("crate::cim");
            Class = new RustWmiClass(wmiClass, this);
        }


        public override string GetSourceCode()
        {
            StringBuilder sb = new StringBuilder();

            sb.AppendLine(CopyrightText);
            sb.AppendLine(HeaderComment);

            // Use declarations
            foreach (var item in References)
            {
                sb.AppendLine(item.GetSourceCode());
            }
            sb.AppendLine();

            // Types
            sb.AppendLine(Class.GetSourceCode());

            return sb.ToString();
        }

        protected override WmiReference GetWmiReference(string reference)
        {
            return new RustWmiReference(reference);
        }

        public override void GenerateSource(string outdir)
        {
            string fileName = ToSnakeCase(Name) + ".rs";
            string path = Path.Combine(outdir, fileName);

            File.WriteAllText(path, GetSourceCode());
            Logger.Info("Source {0}", path);
        }

        public static string GetHeaderCommentText(QualifierDataCollection qCollection)
        {
            object description = IFormat.GetQualifierValue(qCollection, "description");
            return String.Format(CultureInfo.InvariantCulture,
@"
/// {0}",
               description != null ? description.ToString().Replace("\n", "\n/// ") : string.Empty);
        }

        protected override WmiEnum GetWmiEnum(string enumName, WmiSource wSource)
        {
            return new RustWmiEnum(enumName, wSource as RustWmiSource);
        }

        protected WmiSource GetWmiSource(string sourceName, WmiModule wModule)
        {
            return new RustWmiSource(sourceName, wModule);
        }

        //
        // Get the Rust type of the property
        //
        public override string GetType(PropertyData pData, out Type type)
        {
            type = ConvertCimTypeToSystemType(pData.Type);
            string typeName = ConvertToRustType(type);
            string moduleName = Parent.Parent.Name;

            if (pData.Type == CimType.Object || pData.Type == CimType.Reference)
            {
                object typeValue;
                if (IFormat.TryGetQualifierValue(pData.Qualifiers, "CimType", out typeValue))
                {
                    string typeValueString = typeValue.ToString();
                    if (typeValueString.Contains(":"))
                    {
                        string[] typeValues = typeValueString.Split(new char[] { ':' });
                        if (typeValues.Length > 1)
                        {
                            // Add reference to this type
                            typeName = WmiClass.FixClassName(typeValues[1]);

                            if (!typeName.Equals("unint32", StringComparison.OrdinalIgnoreCase))
                            {
                                if (!Parent.Parent.HasSource(typeName) &&
                                    !CheckClass(typeName, Parent.Parent.Name))
                                {
                                    Logger.Debug("Class not found in the current Namespace." +
                                    " Start searching from root namespace - {0}",
                                        Namespace);
                                    string reference = GetReference(typeName, "root");
                                    if (!string.IsNullOrEmpty(reference))
                                    {
                                        AddReference(reference);
                                        Parent.Parent.AddReference(reference);
                                    }
                                    else
                                    {
                                        Parent.Parent.AddSource(GetWmiSource(typeName, Parent.Parent.AddModule(moduleName)));
                                    }
                                }
                            }
                            else
                            {
                                type = typeof(UInt32);
                                typeName = "u32";
                            }
                        }
                    }
                }
            }
            else if (pData.Type != CimType.Boolean && IFormat.HasQualifier(pData.Qualifiers, "values"))
            {
                WmiEnum wEnum = GetEnum(pData);
                if (wEnum != null)
                {
                    typeName = wEnum.Name;
                }
                else typeName = ConvertToRustType(type);
            }
            else
            {
                typeName = ConvertToRustType(type);
            }
            return FixTypeName(typeName);
        }

        private string ConvertToRustType(Type type)
        {
            if (type == typeof(Boolean)) return "bool";
            if (type == typeof(String)) return "String";
            if (type == typeof(SByte)) return "i8";
            if (type == typeof(Byte)) return "u8";
            if (type == typeof(Int16)) return "i16";
            if (type == typeof(UInt16)) return "u16";
            if (type == typeof(Int32)) return "i32";
            if (type == typeof(UInt32)) return "u32";
            if (type == typeof(Int64)) return "i64";
            if (type == typeof(UInt64)) return "u64";
            if (type == typeof(Single)) return "f32";
            if (type == typeof(Double)) return "f64";
            if (type == typeof(Char)) return "char";
            if (type == typeof(DateTime)) return "String"; // WMI DateTime as string
            if (type == typeof(Object)) return "serde_json::Value";
            return "serde_json::Value";
        }

        private string FixTypeName(string name)
        {
            if (name.Equals("unint32", StringComparison.OrdinalIgnoreCase)) return "u32";
            return name;
        }

        private static string ToSnakeCase(string name)
        {
            if (string.IsNullOrEmpty(name))
                return name;

            var result = new StringBuilder();
            for (int i = 0; i < name.Length; i++)
            {
                char c = name[i];
                if (char.IsUpper(c))
                {
                    if (i > 0 && !char.IsUpper(name[i - 1]))
                        result.Append('_');
                    result.Append(char.ToLowerInvariant(c));
                }
                else
                {
                    result.Append(c);
                }
            }
            return result.ToString().ToLowerInvariant();
        }
    }
}
