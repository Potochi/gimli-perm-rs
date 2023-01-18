use std::ops::Shl;

use crate::{gimli_get, gimli_idx};
use crate::constants::{GIMLI_COLS, GIMLI_SIZE, GIMLI_XOR_CONSTANT};
use crate::permutation::traits::GimliPermutation;

#[derive(Debug, Clone, Copy)]
pub struct GimliState {
    pub state: [u32; 12],
}

impl GimliState {
    pub fn from_arr(state: [u32; GIMLI_SIZE]) -> Self {
        Self { state }
    }
}

impl std::ops::BitXor for GimliState {
    type Output = GimliState;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut new_state = Self::Output::from_arr(self.state);

        for i in 0..GIMLI_SIZE {
            new_state.state[i] ^= rhs.state[i];
        }

        new_state
    }
}

pub struct GimliNaive;

impl GimliNaive {
    fn round_function(state: &mut GimliState) {
        for j in 0usize..=3 {
            let x = gimli_get!(state, 0, j).rotate_left(24);
            let y = gimli_get!(state, 1, j).rotate_left(9);
            let z = gimli_get!(state, 2, j);

            state.state[gimli_idx!( 2, j)] = x ^ z.shl(1) ^ (y & z).shl(2);
            state.state[gimli_idx!( 1, j)] = y ^ x ^ (x | z).shl(1);
            state.state[gimli_idx!( 0, j)] = z ^ y ^ (x & y).shl(3);
        }
    }
}

impl GimliPermutation for GimliNaive {
    type StateType = GimliState;

    fn gimli(state: &GimliState) -> Self::StateType {
        let mut modified_state = state.clone();

        Self::gimli_inplace(&mut modified_state);

        modified_state
    }

    fn gimli_inplace(state: &mut Self::StateType) {
        for r in (1..=24).rev() {
            Self::round_function(state);

            if r % 4 == 0 {
                state.state.swap(gimli_idx!(0, 0), gimli_idx!(0, 1));
                state.state.swap(gimli_idx!(0, 2), gimli_idx!(0, 3));
            } else if r % 4 == 2 {
                state.state.swap(gimli_idx!(0, 0), gimli_idx!(0, 2));
                state.state.swap(gimli_idx!(0, 1), gimli_idx!(0, 3));
            }

            if r % 4 == 0 {
                state.state[gimli_idx!(0, 0)] ^= GIMLI_XOR_CONSTANT ^ r;
            }
        }
    }
}

#[test]
fn test_gimli_naive() {
    use crate::constants::*;

    let state = GimliState::from_arr(GIMLI_REF_INPUT);

    let output = GimliNaive::gimli(&state);

    assert_eq!(output.state, GIMLI_REF_OUTPUT);
}
