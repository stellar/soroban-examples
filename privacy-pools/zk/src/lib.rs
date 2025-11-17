#![no_std]

use soroban_sdk::{
    contracterror,
    crypto::bls12_381::{Fr, G1Affine, G2Affine, G1_SERIALIZED_SIZE, G2_SERIALIZED_SIZE},
    vec, Env, Vec, Bytes, U256,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Groth16Error {
    MalformedVerifyingKey = 0,
}

#[derive(Clone)]
pub struct VerificationKey {
    pub alpha: G1Affine,
    pub beta: G2Affine,
    pub gamma: G2Affine,
    pub delta: G2Affine,
    pub ic: Vec<G1Affine>,
}

impl VerificationKey {
    pub fn to_bytes(&self, env: &Env) -> Bytes {
        let mut bytes = Bytes::new(env);
        bytes.append(&Bytes::from_slice(env, &self.alpha.to_bytes().to_array()));
        bytes.append(&Bytes::from_slice(env, &self.beta.to_bytes().to_array()));
        bytes.append(&Bytes::from_slice(env, &self.gamma.to_bytes().to_array()));
        bytes.append(&Bytes::from_slice(env, &self.delta.to_bytes().to_array()));
        // Serialize ic length as u32 (big endian)
        let ic_len = self.ic.len() as u32;
        let ic_len_bytes = ic_len.to_be_bytes();
        bytes.append(&Bytes::from_slice(env, &ic_len_bytes));
        for g1 in self.ic.iter() {
            bytes.append(&Bytes::from_slice(env, &g1.to_bytes().to_array()));
        }
        bytes
    }

    pub fn from_bytes(env: &Env, bytes: &Bytes) -> Result<Self, Groth16Error> {
        let mut pos = 0;
        // Helper to extract a fixed-size array from Bytes
        fn take<const N: usize>(bytes: &Bytes, pos: &mut usize) -> [u8; N] {
            let start = *pos as u32;
            let end = (*pos + N) as u32;
            let mut arr = [0u8; N];
            bytes.slice(start..end).copy_into_slice(&mut arr);
            *pos += N;
            arr
        }
        
        // Deserialize fields
        let alpha = G1Affine::from_array(env, &take::<G1_SERIALIZED_SIZE>(bytes, &mut pos));
        let beta = G2Affine::from_array(env, &take::<G2_SERIALIZED_SIZE>(bytes, &mut pos));
        let gamma = G2Affine::from_array(env, &take::<G2_SERIALIZED_SIZE>(bytes, &mut pos));
        let delta = G2Affine::from_array(env, &take::<G2_SERIALIZED_SIZE>(bytes, &mut pos));
        // ic length
        let ic_len_bytes = take::<4>(bytes, &mut pos);
        let ic_len = u32::from_be_bytes(ic_len_bytes) as usize;
        let mut ic = Vec::new(env);
        for _ in 0..ic_len {
            let g1 = G1Affine::from_array(env, &take::<G1_SERIALIZED_SIZE>(bytes, &mut pos));
            ic.push_back(g1);
        }
        Ok(VerificationKey { alpha, beta, gamma, delta, ic })
    }
}

#[derive(Clone)]
pub struct Proof {
    pub a: G1Affine,
    pub b: G2Affine,
    pub c: G1Affine,
}

impl Proof {
    pub fn to_bytes(&self, env: &Env) -> Bytes {
        let mut bytes = Bytes::new(env);
        bytes.append(&Bytes::from_slice(env, &self.a.to_bytes().to_array()));
        bytes.append(&Bytes::from_slice(env, &self.b.to_bytes().to_array()));
        bytes.append(&Bytes::from_slice(env, &self.c.to_bytes().to_array()));
        bytes
    }

    pub fn from_bytes(env: &Env, bytes: &Bytes) -> Self {
        let mut pos = 0;
        fn take<const N: usize>(bytes: &Bytes, pos: &mut usize) -> [u8; N] {
            let start = *pos as u32;
            let end = (*pos + N) as u32;
            let mut arr = [0u8; N];
            bytes.slice(start..end).copy_into_slice(&mut arr);
            *pos += N;
            arr
        }
        let a = G1Affine::from_array(env, &take::<G1_SERIALIZED_SIZE>(bytes, &mut pos));
        let b = G2Affine::from_array(env, &take::<G2_SERIALIZED_SIZE>(bytes, &mut pos));
        let c = G1Affine::from_array(env, &take::<G1_SERIALIZED_SIZE>(bytes, &mut pos));
        Proof { a, b, c }
    }
}

#[derive(Clone)]
pub struct PublicSignals {
    pub pub_signals: Vec<Fr>,
}

impl PublicSignals {
    pub fn to_bytes(&self, env: &Env) -> Bytes {
        let mut bytes = Bytes::new(env);
        let len = self.pub_signals.len() as u32;
        let len_bytes = len.to_be_bytes();
        bytes.append(&Bytes::from_slice(env, &len_bytes));
        for fr in self.pub_signals.iter() {
            let u256 = fr.to_u256();
            let arr32 = u256.to_be_bytes();
            bytes.append(&arr32);
        }
        bytes
    }

    pub fn from_bytes(env: &Env, bytes: &Bytes) -> Self {
        let mut pos = 0;
        fn take<const N: usize>(bytes: &Bytes, pos: &mut usize) -> [u8; N] {
            let start = *pos as u32;
            let end = (*pos + N) as u32;
            let mut arr = [0u8; N];
            bytes.slice(start..end).copy_into_slice(&mut arr);
            *pos += N;
            arr
        }
        // Read length (u32, big-endian)
        let len_bytes = take::<4>(bytes, &mut pos);
        let len = u32::from_be_bytes(len_bytes) as usize;
        let mut pub_signals = Vec::new(env);
        for _ in 0..len {
            let arr = take::<32>(bytes, &mut pos);
            let u256 = U256::from_be_bytes(env, &Bytes::from_array(env, &arr));
            let fr = Fr::from_u256(u256);
            pub_signals.push_back(fr);
        }
        PublicSignals { pub_signals }
    }
}


pub struct Groth16Verifier;

impl Groth16Verifier {
    pub fn verify_proof(
        env: &Env,
        vk: VerificationKey,
        proof: Proof,
        pub_signals: &Vec<Fr>,
    ) -> Result<bool, Groth16Error> {
        let bls = env.crypto().bls12_381();

        // Prepare proof inputs:
        // Compute vk_x = ic[0] + sum(pub_signals[i] * ic[i+1])
        if pub_signals.len() + 1 != vk.ic.len() {
            return Err(Groth16Error::MalformedVerifyingKey);
        }
        let mut vk_x = vk.ic.get(0).unwrap();
        for (s, v) in pub_signals.iter().zip(vk.ic.iter().skip(1)) {
            let prod = bls.g1_mul(&v, &s);
            vk_x = bls.g1_add(&vk_x, &prod);
        }

        // Compute the pairing:
        // e(-A, B) * e(alpha, beta) * e(vk_x, gamma) * e(C, delta) == 1
        let neg_a = -proof.a;
        let vp1 = vec![env, neg_a, vk.alpha, vk_x, proof.c];
        let vp2 = vec![&env, proof.b, vk.beta, vk.gamma, vk.delta];

        Ok(bls.pairing_check(vp1, vp2))
    }
}

#[cfg(test)]
mod test;
