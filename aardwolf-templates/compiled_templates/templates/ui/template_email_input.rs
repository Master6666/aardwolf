use std::io::{self, Write};
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use ::templates::{Html,ToHtml};
use crate::{EmailInput, templates::ui::input};

pub fn email_input(out: &mut Write, email_input: &EmailInput)
-> io::Result<()> {
input(out, &email_input.into())?;
write!(out, "\n")?;
Ok(())
}
