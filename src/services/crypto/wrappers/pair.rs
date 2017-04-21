use errors::crypto::CryptoError;

pub struct PointG1 {}

pub struct PointG2 {}

#[derive(Copy, Clone)]
pub struct GroupOrderElement {}

pub struct Pair {}

impl PointG1 {
    pub fn new() -> Result<PointG1, CryptoError> {
        // generate random point from the group G1
        unimplemented!();
    }

    pub fn new_inf() -> Result<PointG1, CryptoError> {
        unimplemented!()
    }

    pub fn mul(&self, gr: &GroupOrderElement) -> Result<PointG1, CryptoError> {
        unimplemented!();
    }

    pub fn add(&self, q: &PointG1) -> Result<PointG1, CryptoError> {
        unimplemented!()
    }

    pub fn sub(&self, q: &PointG1) -> Result<PointG1, CryptoError> {
        unimplemented!()
    }

    pub fn to_string(&self) -> Result<String, CryptoError> {
        unimplemented!();
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        unimplemented!();
    }

    pub fn from_bytes(b: &[u8]) -> Result<PointG1, CryptoError> {
        unimplemented!();
    }
}

impl GroupOrderElement {
    pub fn new() -> Result<GroupOrderElement, CryptoError> {
        // returns random element in 0, ..., GroupOrder-1
        unimplemented!();
    }

    pub fn pow_mod(&self, e: &GroupOrderElement) -> Result<GroupOrderElement, CryptoError> {
        // need to use powmod where n - group_order
        unimplemented!();
    }

    pub fn add_mod(&self, r: &GroupOrderElement) -> Result<GroupOrderElement, CryptoError> {
        //need to use rmod after add
        unimplemented!()
    }

    pub fn mul_mod(&self, r: &GroupOrderElement) -> Result<GroupOrderElement, CryptoError> {
        // use modmul where n - group_order
        unimplemented!();
    }

    pub fn inverse(&self) -> Result<GroupOrderElement, CryptoError> {
        unimplemented!();
    }

    pub fn to_string(&self) -> Result<String, CryptoError> {
        unimplemented!();
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        unimplemented!();
    }

    pub fn from_bytes(b: &[u8]) -> Result<GroupOrderElement, CryptoError> {
        unimplemented!();
    }
}

impl Pair {
    pub fn pair(p: &PointG1, q: &PointG1) -> Result<Pair, CryptoError> {
        unimplemented!();
    }

    pub fn mul(&self, b: &Pair) -> Result<Pair, CryptoError> {
        unimplemented!();
    }

    pub fn pow(&self, b: &GroupOrderElement) -> Result<Pair, CryptoError> {
        unimplemented!();
    }

    pub fn inverse(&self) -> Result<Pair, CryptoError> {
        unimplemented!();
    }

    pub fn to_string(&self) -> Result<String, CryptoError> {
        unimplemented!();
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        unimplemented!();
    }

    pub fn from_bytes(b: &[u8]) -> Result<Pair, CryptoError> {
        unimplemented!();
    }
}