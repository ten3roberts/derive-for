/// Defines several structs each using the same derive procedures.
/// ## Example
/// ```
/// #[macro_use] extern crate derive_for;
/// derive_for!(
/// ( Clone, Debug, PartialEq, Eq)
/// pub struct Foo{a: i32, name: String};
/// pub struct Bar(u32, u32);
/// );
/// ```
#[macro_export]
macro_rules! derive_for {
    // Derive tuple
    (($b:ident $(,$a:ident)*) $(#[$attr:meta])* $vis:vis struct $name:ident($($fields:tt)*); $($rest:tt)*) => {
        #[derive($($a,)* $b)]
        $(#[$attr])*
        $vis struct $name($($fields)*);
        derive_for!( ($b, $($a,)*) $($rest)*);
    };
    // Derive tuple with trailing derive comma
    (($b:ident $(,$a:ident)*,) $(#[$attr:meta])* $vis:vis struct $name:ident($($fields:tt)*); $($rest:tt)*) => {
        #[derive($($a,)* $b)]
        $(#[$attr])*
        $vis struct $name($($fields)*);
        derive_for!( ($b, $($a,)*) $($rest)*);
    };
    // Derive struct
    (($b:ident $(,$a:ident)*) $(#[$attr:meta])* $vis:vis struct $name:ident{ $($fields:tt)* }; $($rest:tt)*) => {
        #[derive($($a,)* $b)]
        $(#[$attr])*
        $vis struct $name{ $($fields)* }
        derive_for!( ($b, $($a,)*) $($rest)*);
    };
    // Derive struct with trailing derive comma
    (($b:ident $(,$a:ident)*,) $(#[$attr:meta])* $vis:vis struct $name:ident{ $($fields:tt)* }; $($rest:tt)*) => {
        #[derive($($a,)* $b)]
        $(#[$attr])*
        $vis struct $name{ $($fields)* }
        derive_for!( ($b, $($a,)*) $($rest)*);
    };
    (($($_:ident,)*)) => {};
}

#[cfg(test)]
#[macro_use]
mod tests {
    use crate::derive_for;

    derive_for!(
    ( Clone, Debug, PartialEq, Eq)
    pub struct Foo{a: i32, name: String};
    pub struct Bar(u32, u32);
        );

    #[test]
    pub fn test() {
        let foo = Foo {
            a: 5,
            name: "Hello".into(),
        };
        let bar = Bar(3, 6);

        // Debug
        println!("{:?}, {:?}", foo, bar);

        // Eq
        assert_eq!(
            foo,
            Foo {
                a: 5,
                name: "Hello".into()
            }
        );
        assert_eq!(bar, Bar(3, 6));

        // Clone
        assert_eq!(foo, foo.clone());
        assert_eq!(bar, bar.clone());
    }
}
