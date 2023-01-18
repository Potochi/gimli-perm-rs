pub trait GimliPermutation {
    type StateType;

    fn gimli(state: &Self::StateType) -> Self::StateType;
    fn gimli_inplace(state: &mut Self::StateType);
}
