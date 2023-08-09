pub mod run_length_encoded {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[allow(unused_variables)]
    pub fn serialize<I, S, T>(source: I, s: S) -> Result<S::Ok, S::Error>
    where
        I: IntoIterator<Item = T>,
        S: Serializer,
        T: Copy + PartialEq + Serialize,
    {
        todo!()
    }

    #[allow(unused_variables)]
    pub fn deserialize<'de, D, T>(d: D) -> Result<Vec<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Copy + Deserialize<'de>,
    {
        todo!()
    }
}

pub mod bit_vec {
    use bitvec::prelude::*;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(bit_vec: &BitVec, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        super::run_length_encoded::serialize(
            bit_vec.iter().by_vals().map(|b| if b { 1u8 } else { 0u8 }),
            s,
        )
    }

    pub fn deserialize<'de, D>(d: D) -> Result<BitVec, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(super::run_length_encoded::deserialize::<_, u8>(d)?
            .iter()
            .map(|n| *n != 0)
            .collect::<BitVec>())
    }
}
