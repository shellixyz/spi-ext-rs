#![no_std]

use frequency::Hertz;

pub trait SpiExt {
	type SetFrequencyError;
	fn frequency(&self) -> Hertz;
	fn set_frequency(&mut self, freq: Hertz) -> Result<(), Self::SetFrequencyError>;
}
