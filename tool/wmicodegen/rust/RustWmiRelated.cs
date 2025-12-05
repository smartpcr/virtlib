// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

using System;
using System.Globalization;
using System.Text;
using Microsoft.wmi.Common;

namespace Microsoft.wmi.Rust
{
    public class RustWmiRelated : WmiRelated
    {
        public RustWmiRelated(string name, RustWmiClass parent) : base(name, parent)
        {
        }

        public string GetSourceCode(bool multipleElement)
        {
            StringBuilder sb = new StringBuilder();

            string propertyName = Name.Contains('_') ? Name.Split('_')[1] : Name;
            string methodName = ToSnakeCase($"get_related_{propertyName}");
            string returnType = multipleElement ? $"Vec<{Name}>" : Name;

            sb.AppendFormat(CultureInfo.InvariantCulture,
                "    /// Gets the related {0} object(s)\n", Name);
            sb.AppendFormat(CultureInfo.InvariantCulture,
                "    pub fn {0}(&self) -> Result<{1}, WmiError> {{\n",
                methodName, returnType);

            if (multipleElement)
            {
                sb.AppendFormat(CultureInfo.InvariantCulture,
                    "        self.get_all_related(\"{0}\")\n", Name);
            }
            else
            {
                sb.AppendFormat(CultureInfo.InvariantCulture,
                    "        self.get_related(\"{0}\")\n", Name);
            }

            sb.AppendLine("    }");
            return sb.ToString();
        }

        public override string GetSourceCode()
        {
            throw new NotImplementedException();
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
