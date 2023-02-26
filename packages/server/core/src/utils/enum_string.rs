macro_rules! enum_string {
  ($name:ident, [$($val:ident),*]) => {
      #[derive(Clone, Debug, PartialEq)]
      pub enum $name {
          $( $val, )*
      }

      impl ToString for $name {
          fn to_string(&self) -> String {
              let str = match self {
                  $(
                      Self::$val => stringify!($val),
                  )*
              };

              str.to_string()
          }
      }

      impl FromStr for $name {
          type Err = String;

          fn from_str(s: &str) -> Result<Self, Self::Err> {
              match s {
                  $(
                      stringify!($val) => Ok(Self::$val),
                  )*
                  _ => Err("no match".to_string()),
              }
          }
      }
  };
}

pub(crate) use enum_string;
