mod args;

pub use args::ConverterArgs;

use std::borrow::Cow;

use csaf_rs::{
    converter::{
        Diagnostics,
        Error::{self},
        Warning::{self},
    },
    json::JsonSource,
};

pub struct VerboseDiagnostics {
    error_count: u32,
}

impl Diagnostics for VerboseDiagnostics {
    fn error(&mut self, path: Cow<'static, str>, error: Error) {
        self.error_count += 1;
        eprintln!("Error: {error} ({path})");
    }

    fn has_errors(&self) -> bool {
        self.error_count > 0
    }

    fn warning(&mut self, path: Cow<'static, str>, warning: Warning) {
        eprintln!("Warning: {warning} ({path})");
    }
}

pub fn run_converter(args: ConverterArgs) -> Result<(), anyhow::Error> {
    let path = args.path.as_path();
    if !path.exists() {
        anyhow::bail!("Input file does not exist: {}", path.display());
    }

    let mut diag = VerboseDiagnostics { error_count: 0 };

    let raw: serde_json::Value = path.parse()?;

    let settings = csaf_rs::converter::Settings {
        publisher_category_other: csaf_rs::converter::PublisherCategoryOtherSettings {
            suppress_warning: args.publisher_category_other.suppress_warning,
            convert_to_multiplier: args.publisher_category_other.convert_to_multiplier,
        },
        publisher_contact: csaf_rs::converter::PublisherContactSettings {
            details: args.publisher_contact.details,
            email: args.publisher_contact.email,
            public_openpgp_key_url: args.publisher_contact.public_openpgp_key_url,
            overwrite_extracted: args.publisher_contact.overwrite_extracted,
        },
        model_numbers: csaf_rs::converter::ModelNumberSettings {
            interpret_as_complete: args.model_numbers.interpret_as_complete,
            question_mark_is_literal: args.model_numbers.question_mark_is_literal,
            asterisk_is_literal: args.model_numbers.asterisk_is_literal,
        },
        serial_numbers: csaf_rs::converter::SerialNumberSettings {
            interpret_as_complete: args.serial_numbers.interpret_as_complete,
            question_mark_is_literal: args.serial_numbers.question_mark_is_literal,
            asterisk_is_literal: args.serial_numbers.asterisk_is_literal,
        },
        skus: csaf_rs::converter::StockKeepingUnitSettings {
            interpret_as_complete: args.skus.interpret_as_complete,
            question_mark_is_literal: args.skus.question_mark_is_literal,
            asterisk_is_literal: args.skus.asterisk_is_literal,
        },
    };
    let converted = super::convert(raw, &settings, &mut diag, args.allow_invalid)?;
    let result = serde_json::to_string_pretty(&converted)?;

    if let Some(output) = args.output {
        std::fs::write(output, result)?;
    } else {
        println!("{result}");
    }

    Ok(())
}
