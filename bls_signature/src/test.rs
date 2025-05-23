#![cfg(test)]
extern crate std;

use soroban_sdk::{
    crypto::bls12_381::{Fr, G1Affine, G2Affine},
    testutils::BytesN as _,
    vec, Bytes, BytesN, Env, Vec,
};

use crate::{AccError, IncrementContract, IncrementContractClient};

use hex_literal::hex;

#[derive(Debug)]
pub struct KeyPair {
    pub sk: [u8; 32],
    pub pk: [u8; 96],
}

const DST: &str = "BLSSIG-V01-CS01-with-BLS12381G2_XMD:SHA-256_SSWU_RO_";

static KEY_PAIRS: &[KeyPair] = &[
    KeyPair {
        sk: hex!("18a5ac3cfa6d0b10437a92c96f553311fc0e25500d691ae4b26581e6f925ec83"),
        pk: hex!("0914e32703bad05ccf4180e240e44e867b26580f36e09331997b2e9effe1f509b1a804fc7ba1f1334c8d41f060dd72550901c5549caef45212a236e288a785d762a087092c769bfa79611b96d73521ddd086b7e05b5c7e4210f50c2ee832e183"),
    },
    KeyPair {
        sk: hex!("738dbecafa122ee3c953f07e78461a4281cadec00c869098bac48c8c57b63374"),
        pk: hex!("05f4708a013699229f67d0e16f7c2af8a6557d6d11b737286cfb9429e092c31c412f623d61c7de259c33701aa5387b5004e2c03e8b7ea2740b10a5b4fd050eecca45ccf5588d024cbb7adc963006c29d45a38cb7a06ce2ac45fce52fc0d36572"),
    },
    KeyPair {
        sk: hex!("4bff25b53f29c8af15cf9b8e69988c3ff79c80811d5027c80920f92fad8d137d"),
        pk: hex!("18d0fef68a72e0746f8481fa72b78f945bf75c3a1e036fbbde62a421d8f9568a2ded235a27ad3eb0dc234b298b54dd540f61577bc4c6e8842f8aa953af57a6783924c479e78b0d4959038d3d108b3f6dc6a1b02ec605cb6d789af16cfe67f689"),
    },
    KeyPair {
        sk: hex!("2110f7dae25c4300e1a9681bad6311a547269dba69e94efd342cc208ff50813b"),
        pk: hex!("1643b04cc21f8af9492509c51a6e20e67fa7923f4fbd52f6fcf73c6a4013f864e3e29eb03f54d234582250ebb5df21140381d0c735e868adfe62f85cf8e85d279864333dbe70656a5f35ebc52c5b497f1c65c7a0144bb0c9a1d843f1a8fb9979"),
    },
    KeyPair {
        sk: hex!("1e4b6d54ac58d317cbe6fb0472c3cbf2e60ea157edea21354cbc198770f81448"),
        pk: hex!("02286d1a83a93f35c3461dd71d0840e83e1cd3275ee1af1bfd90ec2366485e9f7f18730f5b686f4810480f1ce5c63dca13a2fac1774aa4e22c29abb9280796d72a2bd0ef963dc76fd45090012bae4a727a6dce49550d9bc9776705f825e24731"),
    },
    KeyPair {
        sk: hex!("471145761f5cd9d0a9a511f1a80657edfcddc43424e4a5582040ea75c4649909"),
        pk: hex!("0b7920a3f2a50cfd6dc132a46b7163d3f7d6b1d03d9fcf450eb05dfa89991a269e707e3412270dc422b664d7adda782c11c973232e975ef0d4b4fb5626b563df542fd1862f80bce17cd09bcbce8884bdda4ac9286bf94854dd29cd511a9103a7"),
    },
    KeyPair {
        sk: hex!("1914beab355b0a86a7bcd37f2e036a9c2c6bff7f16d8bf3e23e42b7131b44701"),
        pk: hex!("1872237fb7ceccc1a6e85f83988c226cc47db75496e41cf20e8a4b93e8fd5e91d0cdcc3b2946a352223ec2b7817a2aae0dc4e6bb7b97c855828670362fcbd0ad6453f28e4fa4b7a075ac8bb1d69a4a1bb8c6723900fead307239f04a9bcec0ad"),
    },
    KeyPair {
        sk: hex!("46b19b928638068780ba82e76dfeaeaf5c37790cdf37f580e206dc6599c72dc7"),
        pk: hex!("0fd1a6b1e46b83a197bbf1dc2a854d024caa5ead5a54893c9767392c837d7c070e86a9206ddba1801332f9d74e0f78e9175419ccc40a966bf4c12a7f8500519e2b83cebd61e32121379911925bf7ae6d2c0d8ec4dcc411d4bbcd14763c1a9d31"),
    },
    KeyPair {
        sk: hex!("0ce3cd1dcaecf002715228aeb0645c6a7fd9990ace3d79515c547dac120bb9f7"),
        pk: hex!("19f7e9dcd4ce2bef92180b60d0c7c7b48b1924a36f9fbb93e9ecb8acb3219e26033b83facd4dc6d2e3f9fa0fffafeca8168bd4824e31dc9dfd977fbf037210508bc807c1a6d20f98a044911f6b689328f3f25dd35a6c05e8c6ac3ac6ef0def91"),
    },
    KeyPair {
        sk: hex!("6b4b27ba3ffc953eff3b974142cdac75f98c8c4ab26f93d5adfd49da5d462c3f"),
        pk: hex!("15f55ec5572026d6c3c7c62b3ce3c5d7539045e9f492f2b1b0860c0af5f5f6b34531dfe4626a92d5c23ac6ad44330cf40e63a8a7234edbb41539c5484eff2cd23b2f0d502a7fd74501b1a05ffee29b24e79cb1ee9fb9b804d84f486283101ee0"),
    },
];

fn aggregate_pk_bytes(env: &Env) -> BytesN<96> {
    let bls = env.crypto().bls12_381();
    let mut agg_pk = G1Affine::from_bytes(BytesN::from_array(env, &KEY_PAIRS[0].pk));
    for i in 1..KEY_PAIRS.len() {
        let pk = G1Affine::from_bytes(BytesN::from_array(env, &KEY_PAIRS[i].pk));
        agg_pk = bls.g1_add(&agg_pk, &pk);
    }
    agg_pk.to_bytes()
}

fn sign_and_aggregate(env: &Env, msg: &Bytes) -> BytesN<192> {
    let bls = env.crypto().bls12_381();
    let mut vec_sk: Vec<Fr> = vec![env];
    for kp in KEY_PAIRS {
        vec_sk.push_back(Fr::from_bytes(BytesN::from_array(env, &kp.sk)));
    }
    let dst = Bytes::from_slice(env, DST.as_bytes());
    let msg_g2 = bls.hash_to_g2(&msg, &dst);
    let vec_msg: Vec<G2Affine> = vec![
        env,
        msg_g2.clone(),
        msg_g2.clone(),
        msg_g2.clone(),
        msg_g2.clone(),
        msg_g2.clone(),
        msg_g2.clone(),
        msg_g2.clone(),
        msg_g2.clone(),
        msg_g2.clone(),
        msg_g2.clone(),
    ];
    bls.g2_msm(vec_msg, vec_sk).to_bytes()
}

fn create_client(e: &Env) -> IncrementContractClient {
    IncrementContractClient::new(e, &e.register(IncrementContract {}, ()))
}

#[test]
fn test() {
    let env = Env::default();
    let pk = aggregate_pk_bytes(&env);
    env.mock_all_auths();

    let client = create_client(&env);
    client.init(&pk);
    let payload = BytesN::random(&env);
    let sig_val = sign_and_aggregate(&env, &payload.clone().into()).to_val();

    env.try_invoke_contract_check_auth::<AccError>(&client.address, &payload, sig_val, &vec![&env])
        .unwrap();
    env.cost_estimate().budget().print();
}
