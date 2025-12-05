// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

using System;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using System.Management;
using System.Text;
using Microsoft.wmi.Common;

namespace Microsoft.wmi.Rust
{
    public class RustWmiClass : WmiClass
    {
        public RustWmiClass(RustWmiSource wSource)
            : base(wSource, "WmiInstance")
        {
        }

        public RustWmiClass(ManagementClass wmiClass, RustWmiSource wSource)
            : base(wmiClass, wSource, "WmiInstance")
        {
        }

        protected override WmiSource GetWmiSource(string sourceName, WmiModule wModule)
        {
            return new RustWmiSource(sourceName, wModule);
        }

        public override string GetSourceCode()
        {
            StringBuilder sb = new StringBuilder();

            // Generate struct with derive macros
            sb.AppendFormat(CultureInfo.InvariantCulture,
                "\n/// {0} struct\n", Name);
            sb.AppendLine("#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]");
            sb.AppendFormat(CultureInfo.InvariantCulture,
                "pub struct {0} {{\n", Name);

            // Add base type field if there's derivation
            if (!string.IsNullOrEmpty(Derivation) && !Derivation.Equals("WmiInstance", StringComparison.OrdinalIgnoreCase))
            {
                sb.AppendFormat("    #[serde(flatten)]\n");
                sb.AppendFormat("    pub base: {0},\n", Derivation);
            }

            // Add properties as struct fields
            foreach (var item in Properties)
            {
                sb.AppendLine(item.GetSourceCode());
            }

            sb.AppendLine("}");
            sb.AppendLine();

            // Create impl block for the struct
            sb.AppendFormat(CultureInfo.InvariantCulture, "impl {0} {{\n", Name);

            // Create constructor
            sb.AppendLine("    /// Creates a new instance of the struct");
            sb.AppendLine("    pub fn new() -> Self {");
            sb.AppendLine("        Self {");
            if (!string.IsNullOrEmpty(Derivation) && !Derivation.Equals("WmiInstance", StringComparison.OrdinalIgnoreCase))
            {
                sb.AppendFormat("            base: {0}::new(),\n", Derivation);
            }
            foreach (var item in Properties)
            {
                var rustProp = item as RustWmiProperty;
                sb.AppendFormat("            {0}: {1},\n",
                    ToSnakeCase(item.Name),
                    rustProp.GetDefaultValue());
            }
            sb.AppendLine("        }");
            sb.AppendLine("    }");
            sb.AppendLine();

            // Create setter and getter methods for properties
            foreach (var item in Properties)
            {
                sb.AppendLine((item as RustWmiProperty).Setter);
                sb.AppendLine((item as RustWmiProperty).Getter);
            }

            // Create the methods for the instance
            foreach (var item in Methods)
            {
                sb.AppendLine(item.GetSourceCode());
            }

            sb.AppendLine("}");

            // Create related object methods in separate impl block
            if (Related.Count > 0)
            {
                sb.AppendLine();
                sb.AppendFormat(CultureInfo.InvariantCulture, "impl {0} {{\n", Name);
                foreach (var item in Related.GroupBy(r => r.Name))
                {
                    bool multiple = item.Count() > 1;
                    sb.AppendLine((item.ElementAt(0) as RustWmiRelated).GetSourceCode(multiple));
                }
                sb.AppendLine("}");
            }

            return sb.ToString();
        }

        protected override WmiConstructor GetWmiConstructor(List<string> paramText, string derivation, string body)
        {
            RustWmiConstructor method = new RustWmiConstructor(this, derivation);
            method.Name = Name;
            foreach (var item in paramText)
            {
                RustWmiMethodParam mParam = new RustWmiMethodParam(method, item);
                method.Params.Add(mParam);
            }
            method.BodyText = body;
            return method;
        }

        protected override WmiMethod GetWmiMethod(MethodData mData, WmiClass wClass)
        {
            return new RustWmiMethod(mData, wClass as RustWmiClass);
        }

        protected override WmiProperty GetWmiProperty(PropertyData pData, WmiClass wClass)
        {
            return new RustWmiProperty(pData, wClass as RustWmiClass);
        }

        protected override WmiRelated GetWmiRelated(string name, WmiClass parent)
        {
            return new RustWmiRelated(name, parent as RustWmiClass);
        }

        protected override void AddReference(string className)
        {
            // Do nothing - Rust uses modules differently
        }

        protected override List<WmiConstructor> GetWmiConstructors()
        {
            // Rust doesn't use traditional constructors like C#/Go
            // We use new() associated function instead
            return new List<WmiConstructor>();
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
