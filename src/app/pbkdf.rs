use argon2::Argon2;

const KEY_SIZE: usize = 32;
const TIME_COST: u32 = 3;
const PARALLELISM_COST: u32 = 4;
const MEM_COST_MIB: u32 = 1024;
const MEM_COST_KIB: u32 = MEM_COST_MIB * 1024;

pub fn derive_key(password: &str, salt: &[u8]) -> [u8; KEY_SIZE] {
    let mut output_key_material = [0u8; 32]; // Can be any desired size
    let config = argon2::ParamsBuilder::default()
        .m_cost(MEM_COST_KIB)
        .t_cost(TIME_COST)
        .p_cost(PARALLELISM_COST)
        .output_len(KEY_SIZE)
        .build()
        .unwrap();

    // TODO: Error handling
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, config);
    argon2
        .hash_password_into(password.as_bytes(), salt, &mut output_key_material)
        .unwrap();

    output_key_material
}
