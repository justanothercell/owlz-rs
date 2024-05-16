use enum_assoc::Assoc;
use rand_derive2::RandGen;
use std::fmt::{Display, Formatter};

/// Create an Owl
///
/// # Example
///
/// ```
/// println!("{}", Owl::default());
/// println!("{}", Owl::random());
/// println!("{}",
///     Owl {
///         beak: Beak::Happy,
///         eyes: Eyes::Happy,
///         head: Head::Curly,
///         wing_shape: WingShape::None,
///         wings: Wings::Outward
///     }
/// );
/// ```
#[derive(RandGen)]
pub struct Owl {
    pub beak: Beak,
    pub eyes: Eyes,
    pub head: Head,
    pub wing_shape: WingShape,
    pub wings: Wings,
}

impl Owl {
    pub fn random() -> Self {
        Owl::generate_random()
    }
}

impl Default for Owl {
    fn default() -> Self {
        Owl {
            beak: Beak::Beak,
            eyes: Eyes::Happy,
            head: Head::Round,
            wing_shape: WingShape::None,
            wings: Wings::Outward,
        }
    }
}

impl Display for Owl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}{}{}",
            self.wings.shape().0,
            self.wing_shape.shape().0,
            self.head.shape().0,
            self.eyes.shape().0,
            self.beak.shape(),
            self.eyes.shape().1,
            self.head.shape().1,
            self.wing_shape.shape().1,
            self.wings.shape().1
        )
    }
}

#[derive(RandGen, Assoc)]
#[func(pub fn shape(&self) -> (&'static str, &'static str))]
pub enum Head {
    #[assoc(shape = ("", ""))]
    None,
    #[assoc(shape = ("[", "]"))]
    Square,
    #[assoc(shape = ("{", "}"))]
    Curly,
    #[assoc(shape = ("(", ")"))]
    Round,
    #[assoc(shape = ("|", "|"))]
    Pipe,
}

#[derive(RandGen, Assoc)]
#[func(pub fn shape(&self) -> (&'static str, &'static str))]
pub enum Eyes {
    #[assoc(shape = ("^", "^"))]
    Happy,
    #[assoc(shape = ("°", "°"))]
    Surprised,
    #[assoc(shape = ("*", "*"))]
    Flashed,
    #[assoc(shape = ("'", "'"))]
    Tired,
    #[assoc(shape = ("\"", "\""))]
    Double,
    #[assoc(shape = ("`", "`"))]
    TiltedLeft,
    #[assoc(shape = ("´", "´"))]
    TiltedRight,
}

#[derive(RandGen, Assoc)]
#[func(pub fn shape(&self) -> &'static str)]
pub enum Beak {
    #[assoc(shape = "")]
    None,
    #[assoc(shape = "v")]
    Beak,
    #[assoc(shape = "u")]
    Happy,
    #[assoc(shape = "n")]
    Sad,
    #[assoc(shape = "~")]
    Confused,
    #[assoc(shape = "o")]
    Surprised,
    #[assoc(shape = "#")]
    Muted,
    #[assoc(shape = ".")]
    Dot,
    #[assoc(shape = "-")]
    Dash,
    #[assoc(shape = "_")]
    Underscore,
    #[assoc(shape = "+")]
    Plus,
    #[assoc(shape = "-")]
    Minus,
}

#[derive(RandGen, Assoc)]
#[func(pub fn shape(&self) -> (&'static str, &'static str))]
pub enum WingShape {
    #[assoc(shape = ("", ""))]
    None,
    #[assoc(shape = ("/", "\\"))]
    Inward,
    #[assoc(shape = ("\\", "/"))]
    Outward,
    #[assoc(shape = ("|", "|"))]
    Pipe,
    #[assoc(shape = ("|", "|"))]
    Bang,
    #[assoc(shape = (":", ":"))]
    Colon,
}

#[derive(RandGen, Assoc)]
#[func(pub fn shape(&self) -> (&'static str, &'static str))]
pub enum Wings {
    #[assoc(shape = ("", ""))]
    None,
    #[assoc(shape = (">", "<"))]
    Outward,
    #[assoc(shape = ("<", ">"))]
    Inward,
}
