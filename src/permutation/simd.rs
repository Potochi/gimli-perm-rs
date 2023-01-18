use std::ops::{Shl, Shr};
use std::simd::{Simd, simd_swizzle};

use crate::constants::{GIMLI_XOR_CONSTANT};
use crate::permutation::traits::GimliPermutation;
use crate::simd_rotate;

#[derive(Debug, Clone, Copy)]
pub struct GimliAVX2State
{
    state: [Simd<u32, 4>; 3],
}

impl GimliAVX2State
{
    pub fn from_state( state: [Simd<u32, 4>; 3]) -> GimliAVX2State{
        Self {
            state
        }
    }

    pub fn from_arr(arr: [u32; 12]) -> Self {
        Self {
            state: [
                Simd::<u32, 4>::from_array(arr[0..4].try_into().unwrap()),
                Simd::<u32, 4>::from_array(arr[4..8].try_into().unwrap()),
                Simd::<u32, 4>::from_array(arr[8..12].try_into().unwrap()),
            ]
        }
    }

    pub fn to_arr(&self) -> [u32; 12] {
        self.state.map(|x| *x.as_array()).concat().try_into().unwrap()
    }
}

impl std::ops::BitXor for GimliAVX2State {
    type Output = GimliAVX2State;

    fn bitxor(self, rhs: Self) -> Self::Output {
        GimliAVX2State::from_state([
            self.state[0] ^ rhs.state[0],
            self.state[1] ^ rhs.state[1],
            self.state[2] ^ rhs.state[2],
        ])
    }
}


const AVX_ROUND_CONSTANTS: [Simd<u32, 4>; 6] = [
    Simd::<u32, 4>::from_array([GIMLI_XOR_CONSTANT ^ 4u32, 0u32, 0u32, 0u32]),
    Simd::<u32, 4>::from_array([GIMLI_XOR_CONSTANT ^ 8u32, 0u32, 0u32, 0u32]),
    Simd::<u32, 4>::from_array([GIMLI_XOR_CONSTANT ^ 12u32, 0u32, 0u32, 0u32]),
    Simd::<u32, 4>::from_array([GIMLI_XOR_CONSTANT ^ 16u32, 0u32, 0u32, 0u32]),
    Simd::<u32, 4>::from_array([GIMLI_XOR_CONSTANT ^ 20u32, 0u32, 0u32, 0u32]),
    Simd::<u32, 4>::from_array([GIMLI_XOR_CONSTANT ^ 24u32, 0u32, 0u32, 0u32]),
];

pub struct GimliAVX2;

impl GimliPermutation for GimliAVX2 {
    type StateType = GimliAVX2State;

    fn gimli(state: &Self::StateType) -> Self::StateType {
        let mut modified_state = state.clone();

        Self::gimli_inplace(&mut modified_state);

        modified_state
    }

    fn gimli_inplace(state: &mut Self::StateType) {
        for r in (1..=24).rev() {
            let x = simd_rotate!(state.state[0], 24);
            let y = simd_rotate!(state.state[1], 9);
            let z = state.state[2];

            state.state[2] = x ^ z.shl(Simd::<u32, 4>::splat(1)) ^
                (y & z).shl(Simd::<u32, 4>::splat(2));
            state.state[1] = y ^ x ^ (x | z).shl(Simd::<u32, 4>::splat(1));
            state.state[0] = z ^ y ^ (x & y).shl(Simd::<u32, 4>::splat(3));

            if r % 4 == 0 {
                state.state[0] = simd_swizzle!(state.state[0], [1, 0, 3, 2]);
            } else if r % 4 == 2 {
                state.state[0] = simd_swizzle!(state.state[0], [2, 3, 0, 1]);
            }

            if r % 4 == 0 {
                state.state[0] ^= AVX_ROUND_CONSTANTS[r / 4 - 1];
            }
        }
    }
}


#[test]
fn test_gimli_avx2() {
    use crate::constants::*;
    let state: GimliAVX2State = GimliAVX2State::from_arr(GIMLI_REF_INPUT);

    let output = GimliAVX2::gimli(&state);

    assert_eq!(output.to_arr(), GIMLI_REF_OUTPUT);
}
