#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub const SPIRV: &[u8] = include_bytes!(env!("shader.spv"));
