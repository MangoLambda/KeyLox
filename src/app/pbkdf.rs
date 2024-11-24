use argon2::Argon2;

use super::models::vault_security_params::KdfParamsEnum;

pub fn derive_key(
    kdf_params: &KdfParamsEnum,
    password: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let argon2_params = match kdf_params {
        KdfParamsEnum::Argon2(argon2_params) => argon2_params,
        KdfParamsEnum::Pbkdf2(_) => return Err("PBKDF2 is not supported.".into()),
    };

    let config = argon2::ParamsBuilder::default()
        .m_cost(argon2_params.get_mem_cost_mib() * 1024)
        .t_cost(argon2_params.get_time_cost())
        .p_cost(argon2_params.get_parallel_cost())
        .output_len(argon2_params.get_output_size() as usize)
        .build()
        .unwrap();

    // TODO: Error handling
    let mut output_key_material: Vec<u8> = vec![0; argon2_params.get_output_size() as usize];
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, config);
    argon2
        .hash_password_into(
            password.as_bytes(),
            &argon2_params.get_salt(),
            &mut output_key_material,
        )
        .unwrap();

    Ok(output_key_material)
}
