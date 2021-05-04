mod typescript;

use typescript::{{TypescriptHTTP}};
use openapiv3::{OpenAPI};

enum ExistingExports {
    TypescriptHttp
}

pub struct ExportOption {
    path: String
}

impl ExportOption {
    fn new(path: String) -> ExportOption {
        ExportOption { path }
    }

    fn as_partial(&self) -> PartialExportOption {
        PartialExportOption::new(self.path.to_owned())
    }
}

pub struct PartialExportOption {
    path: String
}

impl PartialExportOption {
    fn new(path: String) -> PartialExportOption {
        PartialExportOption { path }
    }

    fn as_full(&self) -> ExportOption {
        ExportOption::new(self.path.to_owned())
    }
}

pub struct Exporter {}

impl Exporter {
    fn export(options: PartialExportOption) {}
}