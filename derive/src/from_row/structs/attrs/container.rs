use darling::{util::SpannedValue, FromMeta};

use crate::from_value::{
    enums::attrs::container::{Crate, RenameAll},
    structs::attrs::container::Bound,
};

#[derive(Default, FromMeta)]
pub struct Mysql {
    #[darling(default)]
    pub crate_name: Crate,
    #[darling(default)]
    pub rename_all: Option<RenameAll>,
    #[darling(default)]
    pub table_name: Option<SpannedValue<String>>,
    #[darling(default)]
    pub bound: Option<Bound>,
}
