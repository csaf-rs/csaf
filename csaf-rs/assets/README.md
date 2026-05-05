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

## External Assets

- `scancode-licensedb.json`

Contains the index of ScanCode LicenseDB licenses found at
https://scancode-licensedb.aboutcode.org/index.json
Licensed under the Creative Commons Attribution License 4.0 (CC-BY-4.0).
Copyright (c) nexB Inc. and others. ScanCode is a trademark of nexB Inc.


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

- `cvss-v4.0.1.json` see https://www.first.org/cvss/cvss-v4.0.1.json

License as provided in the document
> Copyright (c) 2025, FIRST.ORG, INC.
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

### CWE resources

- `cwe/*.csv` 

CWE numbers and names for different versions, extracted from https://cwe.mitre.org/data/archive.html

Terms of use according to https://cwe.mitre.org/about/termsofuse.html:
> CWE™ is free to use by any organization or individual for any research, development, and/or commercial purposes, 
> per these CWE Terms > of Use. Accordingly, The MITRE Corporation hereby grants you a non-exclusive, royalty-free 
> license to use CWE for research, > development, and commercial purposes. Any copy you make for such purposes is 
> authorized on the condition that you reproduce MITRE’s > copyright designation and this license in any such copy. 
> CWE is a trademark of The MITRE Corporation. Please contact cwe@mitre.org if > you require further clarification 
> on this issue.
> 
> DISCLAIMERS
> 
> By accessing information through this site you (as “the user”) hereby agrees the site and the information is 
> provided on an “as is” > basis only without warranty of any kind, express or implied, including but not limited
> to implied warranties of merchantability, > availability, accuracy, noninfringement, or fitness for a particular 
> purpose. Use of this site and the information is at the user’s > own risk. The user shall comply with all 
> applicable laws, rules, and regulations, and the data source’s restrictions, when using the > site.
> 
> By contributing information to this site you (as “the contributor”) hereby represents and warrants the contributor 
> has obtained all > necessary permissions from copyright holders and other third parties to allow the contributor to 
> contribute, and this site to host > and display, the information and any such contribution, hosting, and displaying 
> will not violate any law, rule, or regulation. > Additionally, the contributor hereby grants all users of such 
> information a perpetual, worldwide, non-exclusive, no-charge, > royalty-free, irrevocable license to reproduce, 
> prepare derivative works of, publicly display, publicly perform, sublicense, and > distribute such information 
> and all derivative works.
> 
> The MITRE Corporation expressly disclaims any liability for any damages arising from the contributor’s 
> contribution of such > information, the user’s use of the site or such information, and The MITRE Corporation’s 
> hosting the tool and displaying the > information. The foregoing disclaimer specifically includes but is not 
> limited to general, consequential, indirect, incidental, > exemplary, or special or punitive damages (including 
> but not limited to loss of income, program interruption, loss of information, or > other pecuniary loss) arising 
> out of use of this information, no matter the cause of action, even if The MITRE Corporation has been > advised 
> of the possibility of such damages.