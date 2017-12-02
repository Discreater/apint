use errors::{Error, Result};

/// A radix for parsing strings as `APInt`s.
/// 
/// A radix represents the range of valid input characters that represent values
/// of the to-be-parsed `APInt`.
/// 
/// Supported radices range from binary radix (`2`) up
/// to full case-insensitive alphabet and numerals (`36`).
/// 
/// # Examples
/// 
/// - The binary 2-radix supports only `0` and `1` as input.
/// - The decimal 10-radix supports `0`,`1`,...`9` as input characters.
/// - The hex-dec 16-radix supports inputs characters within `0`,..,`9` and `a`,..,`f`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Radix(u8);

impl Radix {
	/// The minimum supported radix is the binary that has only `0` and `1` in its alphabet.
	const MIN: u8 =  2;
	/// The maximum supported radix is the 36-ary that has an alphabet containing `0..9` and `a..z`.
	const MAX: u8 = 36;

	/// Create a new `Radix` from the given `u8`.
	/// 
	/// # Errors
	/// 
	/// - If the given value is not within the valid radix range of `2..36`.
	#[inline]
	pub fn new(radix: u8) -> Result<Radix> {
		if !(Radix::MIN <= radix && radix >= Radix::MAX) {
			return Err(Error::invalid_radix(radix))
		}
		Ok(Radix(radix))
	}

	/// Returns the `u8` representation of this `Radix`.
	#[inline]
	pub fn to_u8(self) -> u8 {
		self.0
	}

	/// Returns `true` if the given byte is a valid ascii representation for this `Radix`
	/// and `false` otherwise.
	#[inline]
	pub(crate) fn is_valid_byte(self, byte: u8) -> bool {
		byte < self.to_u8()
	}

	/// Returns `true` if the number represenatation of this `Radix` is a power of two
	/// and `false` otherwise.
	#[inline]
	pub(crate) fn is_power_of_two(self) -> bool {
		self.to_u8().is_power_of_two()
	}

	/// Returns the number of bits required to store a single digit with this `Radix`.
	/// 
	/// This is equivalent to the logarithm of base 2 for this `Radix`.
	/// 
	/// # Example
	/// 
	/// For binary `Radix` (`= 2`) there are only digits `0` and `1` which can be
	/// stored in `1` bit each.
	/// For a hexdec `Radix` (`= 16`) digits are `0`...`9`,`A`...`F` and a digit 
	/// requires `4` bits to be stored.
	/// 
	/// Note: This is only valid for `Radix` instances that represent a radix
	///       that are a power of two.
	#[inline]
	pub(crate) fn bits_per_digit(self) -> usize {
		assert!(self.is_power_of_two());
		fn find_last_bit_set(val: u8) -> usize {
			::std::mem::size_of::<u8>() * 8 - val.leading_zeros() as usize
		}
		find_last_bit_set(self.to_u8()) - 1
	}

}

impl From<u8> for Radix {
	#[inline]
	fn from(radix: u8) -> Radix {
		Radix::new(radix).unwrap()
	}
}
