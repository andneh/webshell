pub struct Certificate {
    pub name: String,
    pub public_key: String,
    pub private_key: String,
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2+ 2;
        assert_eq!(result, 4);
    }
}
