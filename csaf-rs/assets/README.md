# csaf-rs Asset Sources & Licenses

This library is meant to be published as rust crate(s).
For the sake of successful publishing and reproducibility, we have to bundle/vendor relevant (external) assets.
These assets are described within this `README.md`.

## git-based Assets

### CSAF Schemas

- `csaf_2.0_json_schema.json`
- `csaf_2.1_json_schema.json`

See https://github.com/oasis-tcs/csaf for information regarding these files and their respective licenses.
They are 1:1 copies from the respective repository (i.e., git submodule), cloned solely for successful crate publishing.

### SSVC Resources

- `ssvc_decision_points/**`
- `decision_point_json_schema.json`

See https://github.com/CERTCC/SSVC/tree/main/data/json/decision_points for information regarding these files and
https://github.com/CERTCC/SSVC/blob/main/data/LICENSE for information about licenses.
They are 1:1 copies from the respective repository (i.e., git submodule), cloned solely for successful crate publishing.

- `decision_point_value_selection_list_json_schema.json`

See https://github.com/CERTCC/SSVC/blob/main/data/schema/v2/SelectionList_2_0_0.schema.json for this schema file and
https://github.com/CERTCC/SSVC/blob/main/data/LICENSE for information about licenses.
This is a 1:1 copy from the respective repository (i.e., git submodule), cloned solely for successful crate publishing.

## External Assets

- `language-subtag-registry.txt`

Contains IANA's language subtag registry found at 
https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry.
Assumed to be public domain material, according to https://www.iana.org/help/licensing-terms.

### Metric resources

Schema definitions for metric content

- `cvss-v2.0.json` see https://www.first.org/cvss/cvss-v2.0.json

License as provided in the document
> Copyright (c) 2017, FIRST.ORG, INC.
> All rights reserved.
>
> Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
> following conditions are met:
> 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
>    disclaimer.
> 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
>    following disclaimer in the documentation and/or other materials provided with the distribution.
> 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
>    products derived from this software without specific prior written permission.
> 
> THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS 'AS IS' AND ANY EXPRESS OR IMPLIED WARRANTIES,
> INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
> DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
> SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
> SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
> WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
> OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
- `cvss-v3.0.json` see https://www.first.org/cvss/cvss-v3.0.json

License as provided in the document
> Copyright (c) 2017, FIRST.ORG, INC.
> All rights reserved.
> 
> Redistribution and use in source and binary forms, with or without modification, are permitted provided that the 
> following conditions are met:
> 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following 
>    disclaimer.
> 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the 
>    following disclaimer in the documentation and/or other materials provided with the distribution.
> 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote 
>    products derived from this software without specific prior written permission.
> 
> THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS 'AS IS' AND ANY EXPRESS OR IMPLIED WARRANTIES, 
> INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE 
> DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, 
> SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR 
> SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, 
> WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE 
> OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

- `cvss-v3.1.json` see https://www.first.org/cvss/cvss-v3.1.json

License as provided in the document
> Copyright (c) 2021, FIRST.ORG, INC.
> All rights reserved.
> 
> Redistribution and use in source and binary forms, with or without modification, are permitted provided that the 
> following conditions are met:
> 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following 
>    disclaimer.
> 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the 
>    following disclaimer in the documentation and/or other materials provided with the distribution.
> 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote 
>    products derived from this software without specific prior written permission.
> 
> THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS 'AS IS' AND ANY EXPRESS OR IMPLIED WARRANTIES, 
> INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE 
> DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, 
> SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR 
> SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, 
> WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE 
> OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

- `cvss-v4.0.json` see https://www.first.org/cvss/cvss-v4.0.json

License as provided in the document
> Copyright (c) 2023, FIRST.ORG, INC.
> All rights reserved.
> 
> Redistribution and use in source and binary forms, with or without modification, are permitted provided that the 
> following conditions are met:
> 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following 
>    disclaimer.
> 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the 
>    following disclaimer in the documentation and/or other materials provided with the distribution.
> 3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote 
>    products derived from this software without specific prior written permission.
> 
> THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS 'AS IS' AND ANY EXPRESS OR IMPLIED WARRANTIES, 
> INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE 
> DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, 
> SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR 
> SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, 
> WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE 
> OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.