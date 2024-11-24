use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum Argon2VariantEnum {
    Argon2i,
    Argon2d,
    Argon2id,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum Pbkdf2HashFunctionEnum {
    Sha256,
    Sha512,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum KdfParamsEnum {
    Argon2(Argon2Params),
    Pbkdf2(Pbkdf2Params),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Argon2Params {
    variant: Argon2VariantEnum,
    version: u32,
    mem_cost_mib: u32,
    time_cost: u32,
    parallel_cost: u32,
    output_size: u32,
    salt: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Pbkdf2Params {
    hash_function: Pbkdf2HashFunctionEnum,
    iterations: u32,
    output_size: u32,
    salt: Vec<u8>,
}

impl Argon2Params {
    pub fn new(
        variant: Argon2VariantEnum,
        version: u32,
        mem_cost: u32,
        time_cost: u32,
        parallel_cost: u32,
        output_size: u32,
        salt: Vec<u8>,
    ) -> Self {
        Argon2Params {
            variant,
            version,
            mem_cost_mib: mem_cost,
            time_cost,
            parallel_cost,
            output_size,
            salt,
        }
    }

    pub fn get_variant(&self) -> Argon2VariantEnum {
        self.variant.clone()
    }

    pub fn get_version(&self) -> u32 {
        self.version
    }

    pub fn get_mem_cost_mib(&self) -> u32 {
        self.mem_cost_mib
    }

    pub fn get_time_cost(&self) -> u32 {
        self.time_cost
    }

    pub fn get_parallel_cost(&self) -> u32 {
        self.parallel_cost
    }

    pub fn get_output_size(&self) -> u32 {
        self.output_size
    }

    pub fn get_salt(&self) -> Vec<u8> {
        self.salt.clone()
    }
}

impl Pbkdf2Params {
    pub fn new(
        hash_function: Pbkdf2HashFunctionEnum,
        iterations: u32,
        output_size: u32,
        salt: Vec<u8>,
    ) -> Self {
        Pbkdf2Params {
            hash_function,
            iterations,
            output_size,
            salt,
        }
    }
    pub fn get_hash_function(&self) -> Pbkdf2HashFunctionEnum {
        self.hash_function.clone()
    }

    pub fn get_iterations(&self) -> u32 {
        self.iterations
    }

    pub fn get_output_size(&self) -> u32 {
        self.output_size
    }

    pub fn get_salt(&self) -> Vec<u8> {
        self.salt.clone()
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum EncryptionParamsEnum {
    Aegis(AegisParams),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct AegisParams {
    key_size: u32,
    nonce: Vec<u8>,
    auth_tag: Vec<u8>,
}

impl AegisParams {
    pub fn new(key_size: u32, nonce: Vec<u8>, auth_tag: Vec<u8>) -> Self {
        AegisParams {
            key_size,
            nonce,
            auth_tag,
        }
    }

    pub fn get_key_size(&self) -> u32 {
        self.key_size
    }

    pub fn get_nonce(&self) -> Vec<u8> {
        self.nonce.clone()
    }

    pub fn get_auth_tag(&self) -> Vec<u8> {
        self.auth_tag.clone()
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct VaultSecurityParams {
    kdf_params: KdfParamsEnum,
    encryption_params: EncryptionParamsEnum,
}

impl VaultSecurityParams {
    pub fn new(kdf_params: KdfParamsEnum, encryption_params: EncryptionParamsEnum) -> Self {
        VaultSecurityParams {
            kdf_params: kdf_params,
            encryption_params: encryption_params,
        }
    }

    pub fn get_kdf_params(&self) -> KdfParamsEnum {
        self.kdf_params.clone()
    }

    pub fn get_encryption_params(&self) -> EncryptionParamsEnum {
        self.encryption_params.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_argon2_params() {
        let params = Argon2Params {
            variant: Argon2VariantEnum::Argon2id,
            version: 0x13,
            mem_cost_mib: 4096,
            time_cost: 3,
            parallel_cost: 1,
            output_size: 32,
            salt: vec![1, 2, 3, 4],
        };

        assert_eq!(params.get_variant(), Argon2VariantEnum::Argon2id);
        assert_eq!(params.get_version(), 0x13);
        assert_eq!(params.get_mem_cost_mib(), 4096);
        assert_eq!(params.get_time_cost(), 3);
        assert_eq!(params.get_parallel_cost(), 1);
        assert_eq!(params.get_output_size(), 32);
        assert_eq!(params.get_salt(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_pbkdf2_params() {
        let params = Pbkdf2Params {
            hash_function: Pbkdf2HashFunctionEnum::Sha512,
            iterations: 10000,
            output_size: 32,
            salt: vec![5, 6, 7, 8],
        };

        assert_eq!(params.get_hash_function(), Pbkdf2HashFunctionEnum::Sha512);
        assert_eq!(params.get_iterations(), 10000);
        assert_eq!(params.get_output_size(), 32);
        assert_eq!(params.get_salt(), vec![5, 6, 7, 8]);
    }

    #[test]
    fn test_aegis_params() {
        let params = AegisParams {
            key_size: 32,
            nonce: vec![9, 10, 11, 12],
            auth_tag: vec![13, 14, 15, 16],
        };

        assert_eq!(params.get_key_size(), 32);
        assert_eq!(params.get_nonce(), vec![9, 10, 11, 12]);
        assert_eq!(params.get_auth_tag(), vec![13, 14, 15, 16]);
    }

    #[test]
    fn test_vault_security_params() {
        let kdf_params = KdfParamsEnum::Argon2(Argon2Params {
            variant: Argon2VariantEnum::Argon2id,
            version: 0x13,
            mem_cost_mib: 4096,
            time_cost: 3,
            parallel_cost: 1,
            output_size: 32,
            salt: vec![1, 2, 3, 4],
        });

        let encryption_params = EncryptionParamsEnum::Aegis(AegisParams {
            key_size: 32,
            nonce: vec![9, 10, 11, 12],
            auth_tag: vec![13, 14, 15, 16],
        });

        let vault_params = VaultSecurityParams::new(kdf_params.clone(), encryption_params.clone());

        assert_eq!(vault_params.get_kdf_params(), kdf_params);
        assert_eq!(vault_params.get_encryption_params(), encryption_params);
    }

    #[test]
    fn test_serialize_deserialize_vault_security_params() {
        let kdf_params = KdfParamsEnum::Argon2(Argon2Params {
            variant: Argon2VariantEnum::Argon2id,
            version: 0x13,
            mem_cost_mib: 4096,
            time_cost: 3,
            parallel_cost: 1,
            output_size: 32,
            salt: vec![1, 2, 3, 4],
        });

        let encryption_params = EncryptionParamsEnum::Aegis(AegisParams {
            key_size: 32,
            nonce: vec![9, 10, 11, 12],
            auth_tag: vec![13, 14, 15, 16],
        });

        let vault_params = VaultSecurityParams::new(kdf_params.clone(), encryption_params.clone());

        let serialized = rmp_serde::to_vec(&vault_params).unwrap();
        let deserialized: VaultSecurityParams = rmp_serde::from_slice(&serialized).unwrap();

        assert_eq!(deserialized.get_kdf_params(), kdf_params);
        assert_eq!(deserialized.get_encryption_params(), encryption_params);
    }
}
