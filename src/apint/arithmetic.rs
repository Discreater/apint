use apint::{ApInt};
use apint::utils::ZipDataAccessMut::{Inl, Ext};
use traits::{Width};
use errors::{DivOp, Error, Result};
use digit::{Digit, DigitRepr};
use ll;
use utils::{try_forward_bin_mut_impl, forward_mut_impl};

use std::ops::{
	Neg,
	Add,
	Sub,
	Mul,
	AddAssign,
	SubAssign,
	MulAssign
};

/// # Arithmetic Operations
impl ApInt {

	/// Negates this `ApInt` inplace and returns the result.
	/// 
	/// **Note:** This will **not** allocate memory.
	pub fn into_negate(self) -> ApInt {
		forward_mut_impl(self, ApInt::negate)
	}

	/// Negates this `ApInt` inplace.
	/// 
	/// **Note:** This will **not** allocate memory.
	pub fn negate(&mut self) {
		let width = self.width();
		self.bitnot();
		// self.increment_by(1); // This is not implemented, yet.
		                         // Replace `self.checked_add_assign(..)` with this
		                         // as soon as possible for avoiding temporary
		                         // expensive copies of `self`.
		self.checked_add_assign(&ApInt::one(width))
			.expect("This operation cannot fail since the temporary `ApInt`\
						and `self` are ensured to always have the same bit width.");
		self.clear_unused_bits();
	}

	/// Adds `rhs` to `self` and returns the result.
	/// 
	/// **Note:** This will **not** allocate memory.
	/// 
	/// # Errors
	/// 
	/// - If `self` and `rhs` have unmatching bit widths.
	pub fn into_checked_add(self, rhs: &ApInt) -> Result<ApInt> {
		try_forward_bin_mut_impl(self, rhs, ApInt::checked_add_assign)
	}

	/// Add-assigns `rhs` to `self` inplace.
	/// 
	/// **Note:** This will **not** allocate memory.
	/// 
	/// # Errors
	/// 
	/// - If `self` and `rhs` have unmatching bit widths.
	pub fn checked_add_assign(&mut self, rhs: &ApInt) -> Result<()> {
		match self.zip_access_data_mut(rhs)? {
			Inl(lhs, rhs) => {
				let lval = lhs.repr();
				let rval = rhs.repr();
				let result = lval.wrapping_add(rval);
				*lhs = Digit(result);
			}
			Ext(lhs, rhs) => {
				let mut carry = Digit::zero();
				for (l, r) in lhs.into_iter().zip(rhs) {
					*l = ll::carry_add(*l, *r, &mut carry);
				}
			}
		}
		self.clear_unused_bits();
		// Maybe we should return a recoverable error upon carry != 0 at this point.
		Ok(())
	}

	/// Subtracts `rhs` from `self` and returns the result.
	/// 
	/// # Note
	/// 
	/// In the low-level bit-wise representation there is no difference between signed
	/// and unsigned subtraction of fixed bit-width integers. (Cite: LLVM)
	/// 
	/// # Errors
	/// 
	/// - If `self` and `rhs` have unmatching bit widths.
	pub fn into_checked_sub(self, rhs: &ApInt) -> Result<ApInt> {
		try_forward_bin_mut_impl(self, rhs, ApInt::checked_sub_assign)
	}

	/// Subtract-assigns `rhs` from `self` inplace.
	/// 
	/// # Note
	/// 
	/// In the low-level bit-wise representation there is no difference between signed
	/// and unsigned subtraction of fixed bit-width integers. (Cite: LLVM)
	/// 
	/// # Errors
	/// 
	/// - If `self` and `rhs` have unmatching bit widths.
	pub fn checked_sub_assign(&mut self, rhs: &ApInt) -> Result<()> {
		match self.zip_access_data_mut(rhs)? {
			Inl(lhs, rhs) => {
				let lval = lhs.repr();
				let rval = rhs.repr();
				let result = lval.wrapping_sub(rval);
				*lhs = Digit(result);
			}
			Ext(lhs, rhs) => {
				let mut borrow = Digit::zero();
				for (l, r) in lhs.into_iter().zip(rhs) {
					*l = ll::borrow_sub(*l, *r, &mut borrow);
				}
			}
		}
		self.clear_unused_bits();
		// Maybe we should return a recoverable error upon borrow != 0 at this point.
		Ok(())
	}

	/// Multiplies `rhs` with `self` and returns the result.
	/// 
	/// # Note
	/// 
	/// In the low-level bit-wise representation there is no difference between signed
	/// and unsigned multiplication of fixed bit-width integers. (Cite: LLVM)
	/// 
	/// # Errors
	/// 
	/// - If `self` and `rhs` have unmatching bit widths.
	pub fn into_checked_mul(self, rhs: &ApInt) -> Result<ApInt> {
		try_forward_bin_mut_impl(self, rhs, ApInt::checked_mul_assign)
	}

	/// Multiply-assigns `rhs` to `self` inplace.
	/// 
	/// # Note
	/// 
	/// In the low-level bit-wise representation there is no difference between signed
	/// and unsigned multiplication of fixed bit-width integers. (Cite: LLVM)
	/// 
	/// # Errors
	/// 
	/// - If `self` and `rhs` have unmatching bit widths.
	pub fn checked_mul_assign(&mut self, rhs: &ApInt) -> Result<()> {
		match self.zip_access_data_mut(rhs)? {
			Inl(lhs, rhs) => {
				let lval = lhs.repr();
				let rval = rhs.repr();
				let result = lval.wrapping_mul(rval);
				*lhs = Digit(result);
			}
			Ext(_lhs, _rhs) => {
				unimplemented!()
			}
		}
		self.clear_unused_bits();
		Ok(())
	}

	/// Divides `self` by `rhs` using **unsigned** interpretation and returns the result.
	/// 
	/// # Note
	/// 
	/// - This operation will **not** allocate memory and computes inplace of `self`.
	/// - In the low-level machine abstraction signed division and unsigned division
	///   are two different operations.
	/// 
	/// # Errors
	/// 
	/// - If `self` and `rhs` have unmatching bit widths.
	pub fn into_checked_udiv(self, rhs: &ApInt) -> Result<ApInt> {
		try_forward_bin_mut_impl(self, rhs, ApInt::checked_udiv_assign)
	}

	/// Assignes `self` to the division of `self` by `rhs` using **unsigned**
	/// interpretation of the values.
	/// 
	/// # Note
	/// 
	/// - This operation will **not** allocate memory and computes inplace of `self`.
	/// - In the low-level machine abstraction signed division and unsigned division
	///   are two different operations.
	/// 
	/// # Errors
	/// 
	/// - If `self` and `rhs` have unmatching bit widths.
	pub fn checked_udiv_assign(&mut self, rhs: &ApInt) -> Result<()> {
		if rhs.is_zero() {
			return Err(Error::division_by_zero(DivOp::UnsignedDiv, self.clone()))
		}
		match self.zip_access_data_mut(rhs)? {
			Inl(lhs, rhs) => {
				let lval = lhs.repr();
				let rval = rhs.repr();
				let result = lval.wrapping_div(rval);
				*lhs = Digit(result);
			}
			Ext(_lhs, _rhs) => {
				unimplemented!()
			}
		}
		Ok(())
	}

	/// Divides `self` by `rhs` using **signed** interpretation and returns the result.
	/// 
	/// # Note
	/// 
	/// - This operation will **not** allocate memory and computes inplace of `self`.
	/// - In the low-level machine abstraction signed division and unsigned division
	///   are two different operations.
	/// 
	/// # Errors
	/// 
	/// - If `self` and `rhs` have unmatching bit widths.
	pub fn into_checked_sdiv(self, rhs: &ApInt) -> Result<ApInt> {
		try_forward_bin_mut_impl(self, rhs, ApInt::checked_sdiv_assign)
	}

	/// Assignes `self` to the division of `self` by `rhs` using **signed**
	/// interpretation of the values.
	/// 
	/// # Note
	/// 
	/// - This operation will **not** allocate memory and computes inplace of `self`.
	/// - In the low-level machine abstraction signed division and unsigned division
	///   are two different operations.
	/// 
	/// # Errors
	/// 
	/// - If `self` and `rhs` have unmatching bit widths.
	pub fn checked_sdiv_assign(&mut self, rhs: &ApInt) -> Result<()> {
		if rhs.is_zero() {
			return Err(Error::division_by_zero(DivOp::SignedDiv, self.clone()))
		}
		let width = self.width();
		match self.zip_access_data_mut(rhs)? {
			Inl(lhs, rhs) => {
				let mut l = lhs.clone();
				let mut r = rhs.clone();
				l.sign_extend_from(width).unwrap();
				r.sign_extend_from(width).unwrap();
				let lval = l.repr() as i64;
				let rval = r.repr() as i64;
				let result = lval.wrapping_div(rval) as DigitRepr;
				*lhs = Digit(result);
			}
			Ext(_lhs, _rhs) => {
				unimplemented!()
			}
		}
		self.clear_unused_bits();
		Ok(())
	}

	/// Calculates the **unsigned** remainder of `self` by `rhs` and returns the result.
	/// 
	/// # Note
	/// 
	/// - This operation will **not** allocate memory and computes inplace of `self`.
	/// - In the low-level machine abstraction signed division and unsigned division
	///   are two different operations.
	/// 
	/// # Errors
	/// 
	/// - If `self` and `rhs` have unmatching bit widths.
	pub fn into_checked_urem(self, rhs: &ApInt) -> Result<ApInt> {
		try_forward_bin_mut_impl(self, rhs, ApInt::checked_urem_assign)
	}

	/// Assignes `self` to the **unsigned** remainder of `self` by `rhs`.
	/// 
	/// # Note
	/// 
	/// - This operation will **not** allocate memory and computes inplace of `self`.
	/// - In the low-level machine abstraction signed division and unsigned division
	///   are two different operations.
	/// 
	/// # Errors
	/// 
	/// - If `self` and `rhs` have unmatching bit widths.
	pub fn checked_urem_assign(&mut self, rhs: &ApInt) -> Result<()> {
		if rhs.is_zero() {
			return Err(Error::division_by_zero(DivOp::UnsignedRem, self.clone()))
		}
		match self.zip_access_data_mut(rhs)? {
			Inl(lhs, rhs) => {
				let lval = lhs.repr();
				let rval = rhs.repr();
				let result = lval.wrapping_rem(rval);
				*lhs = Digit(result);
			}
			Ext(_lhs, _rhs) => {
				unimplemented!()
			}
		}
		Ok(())
	}

	/// Calculates the **signed** remainder of `self` by `rhs` and returns the result.
	/// 
	/// # Note
	/// 
	/// - This operation will **not** allocate memory and computes inplace of `self`.
	/// - In the low-level machine abstraction signed division and unsigned division
	///   are two different operations.
	/// 
	/// # Errors
	/// 
	/// - If `self` and `rhs` have unmatching bit widths.
	pub fn into_checked_srem(self, rhs: &ApInt) -> Result<ApInt> {
		try_forward_bin_mut_impl(self, rhs, ApInt::checked_srem_assign)
	}

	/// Assignes `self` to the **signed** remainder of `self` by `rhs`.
	/// 
	/// # Note
	/// 
	/// - This operation will **not** allocate memory and computes inplace of `self`.
	/// - In the low-level machine abstraction signed division and unsigned division
	///   are two different operations.
	/// 
	/// # Errors
	/// 
	/// - If `self` and `rhs` have unmatching bit widths.
	pub fn checked_srem_assign(&mut self, rhs: &ApInt) -> Result<()> {
		if rhs.is_zero() {
			return Err(Error::division_by_zero(DivOp::SignedRem, self.clone()))
		}
		let width = self.width();
		match self.zip_access_data_mut(rhs)? {
			Inl(lhs, rhs) => {
				let mut l = lhs.clone();
				let mut r = rhs.clone();
				l.sign_extend_from(width).unwrap();
				r.sign_extend_from(width).unwrap();
				let lval = l.repr() as i64;
				let rval = r.repr() as i64;
				let result = lval.wrapping_rem(rval) as DigitRepr;
				*lhs = Digit(result);
			}
			Ext(_lhs, _rhs) => {
				unimplemented!()
			}
		}
		self.clear_unused_bits();
		Ok(())
	}

}

// ============================================================================
//  Standard `ops` trait implementations.
// ----------------------------------------------------------------------------
// 
//  `ApInt` implements some `std::ops` traits for improved usability.
//  Only traits for operations that do not depend on the signedness
//  interpretation of the specific `ApInt` instance are actually implemented.
//  Operations like `mul`, `div` and `rem` are not expected to have an
//  implementation since a favor in unsigned or signed cannot be decided.
// ============================================================================

// ============================================================================
//  Unary arithmetic negation: `std::ops::Add` and `std::ops::AddAssign`
// ============================================================================

impl Neg for ApInt {
	type Output = ApInt;

	fn neg(self) -> Self::Output {
		self.into_negate()
	}
}

impl<'a> Neg for &'a ApInt {
	type Output = ApInt;

	fn neg(self) -> Self::Output {
		self.clone().into_negate()
	}
}

impl<'a> Neg for &'a mut ApInt {
	type Output = &'a mut ApInt;

	fn neg(self) -> Self::Output {
		self.negate();
		self
	}
}

// ============================================================================
//  Add and Add-Assign: `std::ops::Add` and `std::ops::AddAssign`
// ============================================================================

impl<'a> Add<&'a ApInt> for ApInt {
	type Output = ApInt;

	fn add(self, rhs: &'a ApInt) -> Self::Output {
		self.into_checked_add(rhs).unwrap()
	}
}

impl<'a, 'b> Add<&'a ApInt> for &'b ApInt {
	type Output = ApInt;

	fn add(self, rhs: &'a ApInt) -> Self::Output {
		self.clone().into_checked_add(rhs).unwrap()
	}
}

impl<'a> AddAssign<&'a ApInt> for ApInt {
	fn add_assign(&mut self, rhs: &'a ApInt) {
		self.checked_add_assign(rhs).unwrap()
	}
}

// ============================================================================
//  Sub and Sub-Assign: `std::ops::Sub` and `std::ops::SubAssign`
// ============================================================================

impl<'a> Sub<&'a ApInt> for ApInt {
	type Output = ApInt;

	fn sub(self, rhs: &'a ApInt) -> Self::Output {
		self.into_checked_sub(rhs).unwrap()
	}
}

impl<'a, 'b> Sub<&'a ApInt> for &'b ApInt {
	type Output = ApInt;

	fn sub(self, rhs: &'a ApInt) -> Self::Output {
		self.clone().into_checked_sub(rhs).unwrap()
	}
}

impl<'a> SubAssign<&'a ApInt> for ApInt {
	fn sub_assign(&mut self, rhs: &'a ApInt) {
		self.checked_sub_assign(rhs).unwrap()
	}
}

// ============================================================================
//  Mul and Mul-Assign: `std::ops::Mul` and `std::ops::MulAssign`
// ============================================================================

impl<'a> Mul<&'a ApInt> for ApInt {
	type Output = ApInt;

	fn mul(self, rhs: &'a ApInt) -> Self::Output {
		self.into_checked_mul(rhs).unwrap()
	}
}

impl<'a, 'b> Mul<&'a ApInt> for &'b ApInt {
	type Output = ApInt;

	fn mul(self, rhs: &'a ApInt) -> Self::Output {
		self.clone().into_checked_mul(rhs).unwrap()
	}
}

impl<'a> MulAssign<&'a ApInt> for ApInt {
	fn mul_assign(&mut self, rhs: &'a ApInt) {
		self.checked_mul_assign(rhs).unwrap();
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	mod negate {
		use super::*;

		use bitwidth::{BitWidth};

		fn assert_symmetry(input: ApInt, expected: ApInt) {
			assert_eq!(input.clone().into_negate(), expected.clone());
			assert_eq!(expected.into_negate(), input);
		}

		fn test_vals() -> impl Iterator<Item = i128> {
			[0_i128, 1, 2, 4, 5, 7, 10, 42, 50, 100, 128, 150,
			 1337, 123123, 999999, 987432, 77216417].into_iter().map(|v| *v)
		}

		#[test]
		fn simple() {
			assert_symmetry(ApInt::zero(BitWidth::w1()), ApInt::zero(BitWidth::w1()));
			assert_symmetry(ApInt::one(BitWidth::w1()), ApInt::all_set(BitWidth::w1()));
		}

		#[test]
		fn range() {
			for v in test_vals() {
				assert_symmetry(ApInt::from_i8(v as i8), ApInt::from_i8(-v as i8));
				assert_symmetry(ApInt::from_i16(v as i16), ApInt::from_i16(-v as i16));
				assert_symmetry(ApInt::from_i32(v as i32), ApInt::from_i32(-v as i32));
				assert_symmetry(ApInt::from_i64(v as i64), ApInt::from_i64(-v as i64));
				assert_symmetry(ApInt::from_i128(v), ApInt::from_i128(-v));
			}
		}
	}

	mod mul {
		use super::*;

		#[test]
		fn simple() {
			let lhs = ApInt::from(11_u32);
			let rhs = ApInt::from(5_u32);
			let result = lhs.into_checked_mul(&rhs).unwrap();
			assert_eq!(result, ApInt::from(55_u32));
		}
	}

	mod udiv {
		use super::*;

		#[test]
		fn simple() {
			let lhs = ApInt::from(56_u32);
			let rhs = ApInt::from(7_u32);
			let result = lhs.into_checked_udiv(&rhs).unwrap();
			assert_eq!(result, ApInt::from(8_u32));
		}
	}

	mod sdiv {
		use super::*;

		#[test]
		fn simple() {
			let lhs = ApInt::from(72_i32);
			let rhs = ApInt::from(12_i32);
			let result = lhs.into_checked_sdiv(&rhs).unwrap();
			assert_eq!(result, ApInt::from(6_u32));
		}

		#[test]
		fn with_neg() {
			let lhs = ApInt::from(72_i32);
			let rhs = ApInt::from(-12_i32);
			let result = lhs.into_checked_sdiv(&rhs).unwrap();
			assert_eq!(result, ApInt::from(-6_i32));
		}
	}

	mod urem {
		use super::*;

		#[test]
		fn simple() {
			let lhs = ApInt::from(15_u32);
			let rhs = ApInt::from(4_u32);
			let result = lhs.into_checked_urem(&rhs).unwrap();
			assert_eq!(result, ApInt::from(3_u32));
		}
	}

	mod srem {
		use super::*;

		#[test]
		fn simple() {
			let lhs = ApInt::from(23_i32);
			let rhs = ApInt::from(7_i32);
			let result = lhs.into_checked_srem(&rhs).unwrap();
			assert_eq!(result, ApInt::from(2_u32));
		}

		#[test]
		fn with_neg() {
			let lhs = ApInt::from(-23_i32);
			let rhs = ApInt::from(7_i32);
			let result = lhs.into_checked_srem(&rhs).unwrap();
			assert_eq!(result, ApInt::from(-2_i32));
		}
	}

}
