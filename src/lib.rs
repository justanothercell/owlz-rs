//! # Owlz
//!
//! `owlz` is a crate for creating random owl ascii emojis.
//! # Example
//!
//! ```
//! println!("{}", Owl::default());
//! println!("{}", Owl::random());
//! println!("{}",
//!     Owl {
//!         beak: Beak::Happy,
//!         eyes: Eyes::Happy,
//!         head: Head::Curly,
//!         wing_shape: WingShape::None,
//!         wings: Wings::Outward
//!     }
//! );
//! ```

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
