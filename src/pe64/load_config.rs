/*!
Load Config Directory.
*/

use std::fmt;

use error::{Error, Result};

use super::image::*;
use super::Pe;

/// Load Config Directory.
#[derive(Copy, Clone)]
pub struct LoadConfig<'a, P> {
	pe: P,
	image: &'a IMAGE_LOAD_CONFIG_DIRECTORY,
}
impl<'a, P: Pe<'a> + Copy> LoadConfig<'a, P> {
	pub(crate) fn new(pe: P) -> Result<LoadConfig<'a, P>> {
		let datadir = pe.data_directory().get(IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG).ok_or(Error::OOB)?;
		let image = pe.derva(datadir.VirtualAddress)?;
		Ok(LoadConfig { pe, image })
	}
	/// Gets the PE instance.
	pub fn pe(&self) -> P {
		self.pe
	}
	/// Returns the underlying load config directory image.
	pub fn image(&self) -> &'a IMAGE_LOAD_CONFIG_DIRECTORY {
		self.image
	}
	/// Gets the default security cookie for the image.
	pub fn security_cookie(&self) -> Result<&'a u32> {
		self.pe.deref(self.image.SecurityCookie)
	}
	/// Gets the structured exception handler table.
	pub fn se_handler_table(&self) -> Result<&'a [Va]> {
		self.pe.deref_slice(self.image.SEHandlerTable, self.image.SEHandlerCount as usize)
	}
}
impl<'a, P: Pe<'a> + Copy> fmt::Debug for LoadConfig<'a, P> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("LoadConfig")
			.field("security_cookie", &format_args!("{:x?}", self.security_cookie()))
			.field("se_handler_table.len", &format_args!("{:?}", self.se_handler_table().map(|seh| seh.len())))
			.finish()
	}
}