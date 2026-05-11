# CSAF-RS

This library is a reference implementation for the CSAF standard. It is generated from the base RUST implementation found at https://github.com/csaf-rs/csaf via UniFFi bindings.

For the license, see https://github.com/csaf-rs/csaf/blob/main/LICENSE.

## Usage

Install this library via npm
```bash
npm install @csaf-rs/csaf-rs
```

Reference the main validation function in your typescript files and run the validation against a string
```ts
import { CsafFfi } from "@csaf-rs/csaf-wasm";

const file_content = readFileSync(filePath, "utf-8");
const result = CsafFfi.validateCsaf(jsonStr, "basic");
```

## Coverage
For an overview which tests from the standard are currently covered, see https://github.com/csaf-rs/csaf#implementation-status-in-regards-to-the-standard