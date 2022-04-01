mod stream {
    use crate::hash;
    fn encrypt_in_place(
        data: &[u8],
        salt: &[u8; 64],
        s_cost: usize,
        t_cost: usize,
    ) -> [u8; 64] {
        let stream = hash::Sashimi::new();
        let mut mask = ;
        salt.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::hash;

    #[test]
    fn import_path_test() {
        let _t = hash::Sashimi::new();
    }
}
