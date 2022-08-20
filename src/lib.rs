mod owl;

pub use owl::*;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_owls() {
        let owl = Owl::default();
        assert_eq!(format!("{owl}"), String::from(">(^v^)<"))
    }
}
