use itertools::Itertools;
use serde::{Deserialize, Serialize};

use super::common::*;
use crate::ir::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteDuplicateEnumFields {
    pub from: String,
}

impl DeleteDuplicateEnumFields {
    pub fn run(&self, ir: &mut IR) -> anyhow::Result<()> {
        let re = make_regex(&self.from)?;

        for (_path, val) in ir.enums.iter_mut().filter(|v| re.is_match(v.0)) {
            let new_variants = val
                .variants
                .iter()
                .unique_by(|x| &x.name)
                .cloned()
                .collect::<Vec<_>>();

            val.variants = new_variants;
        }
        Ok(())
    }
}
