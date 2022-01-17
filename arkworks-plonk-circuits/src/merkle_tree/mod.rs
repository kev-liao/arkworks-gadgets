use std::marker::PhantomData;

use crate::poseidon::poseidon::{FieldHasherGadget, PoseidonParametersVar};
use ark_ec::models::TEModelParameters;
use ark_ff::PrimeField;
use arkworks_gadgets::{
	merkle_tree::simple_merkle::{Path, SparseMerkleTree},
	poseidon::field_hasher::Poseidon,
};
use plonk::{constraint_system::StandardComposer, error::Error, prelude::Variable};

#[derive(Clone)]
pub struct PathVar<
	F: PrimeField,
	P: TEModelParameters<BaseField = F>,
	HG: FieldHasherGadget<F, P>,
	const N: usize,
> {
	path: [(Variable, Variable); N], // Or should we use Vec< ...> ?
	_field: PhantomData<F>,
	_te: PhantomData<P>,
	_hg: PhantomData<HG>,
}

impl<
		F: PrimeField,
		P: TEModelParameters<BaseField = F>,
		HG: FieldHasherGadget<F, P>,
		const N: usize,
	> PathVar<F, P, HG, N>
{
	fn from_native(composer: &mut StandardComposer<F, P>, native: Path<F, HG::Native, N>) -> Self {
		// Initialize the array
		let mut path_vars = [(composer.zero_var(), composer.zero_var()); N];

		for i in 0..N {
			path_vars[i] = (
				composer.add_input(native.path[i].0),
				composer.add_input(native.path[i].1),
			);
		}

		PathVar {
			path: path_vars,
			_field: PhantomData,
			_te: PhantomData,
			_hg: PhantomData,
		}
	}

	pub fn check_membership(
		&self,
		composer: &mut StandardComposer<F, P>,
		root_hash: &Variable,
		leaf: &Variable,
		hasher: &HG,
	) -> Result<Variable, Error> {
		Ok(composer.zero_var())
	}

	pub fn calculate_root(
		&self,
		composer: &mut StandardComposer<F, P>,
		leaf: &Variable,
		hash_gadget: &HG,
	) -> Result<Variable, Error> {
		// The `leaf` variable is carrying raw (unhashed) data, so hash first
		let leaf_hash = hash_gadget.hash(composer, &[*leaf])?;

		// Check if leaf is one of the bottom-most siblings
		let leaf_is_left = composer.is_eq_with_output(leaf_hash, self.path[0].0);

		Ok(composer.zero_var())
	}
}
