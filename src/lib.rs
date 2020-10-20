/// Allows you to apply any function given as a parameter to the object.
///
/// As you are able to connect operations to the object with chains, it allow you to describe the
/// sequence of operations neatly.
/// This is useful, for example, if you want to change the internal state with other method after
/// the object is created.
///
/// # Examples
///
/// ```
/// #[derive(Debug, PartialEq)]
/// struct Dog {
///     name: String,
///     size: String,
/// }
/// impl Dog {
///     fn new() -> Self {
///         Self {
///             name: "Pochi".to_string(),
///             size: "Middle".to_string(),
///         }
///     }
/// }
/// let mut exact_dog = Dog::new();
/// exact_dog.size = "Big".to_string();
/// let dog = Dog::new().apply(|it| it.size = "Big".to_string());
/// assert_eq!(dog, exact_dog);
/// ```
///
pub trait Applicable {
    /// Apply the function given as a parameter to self.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut exact_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    /// exact_path.push("src/lib.rs");
    /// let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).apply(|it| it.push("src/lib.rs"));
    /// assert_eq!(path, exact_path);
    /// ```
    ///
    /// ```
    /// let mut exact_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    /// exact_path.push("src/lib.rs");
    /// let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    ///     .apply(|it| it.push("src"))
    ///     .apply(|it| it.push("lib.rs"));
    /// assert_eq!(path, exact_path);
    /// ```
    ///
    fn apply<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut Self);

    /// Apply the function with one parameter given as a parameter to self.
    ///
    /// # Example
    ///
    /// ```
    /// let mut exact_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    /// exact_path.push("src/lib.rs");
    /// let path =
    ///     PathBuf::from(env!("CARGO_MANIFEST_DIR")).apply_with_param(PathBuf::push, "src/lib.rs");
    /// assert_eq!(path, exact_path);
    ///
    /// ```
    ///
    fn apply_with_param<F, P>(self, f: F, p: P) -> Self
    where
        F: FnOnce(&mut Self, P);

    /// Apply apply_with_param repeatedly to multiple parameters.
    ///
    /// # Example
    ///
    /// ```
    /// let mut exact_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    /// exact_path.push("src");
    /// exact_path.push("lib.rs");
    /// let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    ///     .apply_with_params(PathBuf::push, vec!["src", "lib.rs"]);
    /// assert_eq!(path, exact_path);
    /// ```
    fn apply_with_params<F, P>(self, f: F, p: Vec<P>) -> Self
    where
        F: Fn(&mut Self, P);
}

impl<T> Applicable for T {
    fn apply<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut Self),
    {
        let mut receiver = self;
        f(&mut receiver);
        receiver
    }

    fn apply_with_param<F, P>(self, f: F, p: P) -> Self
    where
        F: FnOnce(&mut Self, P),
    {
        let mut receiver = self;
        f(&mut receiver, p);
        receiver
    }

    fn apply_with_params<F, P>(self, f: F, p: Vec<P>) -> Self
    where
        F: Fn(&mut Self, P),
    {
        let mut receiver = self;
        for param in p {
            f(&mut receiver, param);
        }
        receiver
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_apply() {
        let mut exact_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        exact_path.push("src/lib.rs");
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).apply(|it| it.push("src/lib.rs"));
        assert_eq!(path, exact_path);
    }

    #[test]
    fn test_apply_twice() {
        let mut exact_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        exact_path.push("src/lib.rs");
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .apply(|it| it.push("src"))
            .apply(|it| it.push("lib.rs"));
        assert_eq!(path, exact_path);
    }

    #[test]
    fn test_apply_ohter_usage() {
        #[derive(Debug, PartialEq)]
        struct Dog {
            name: String,
            size: String,
        }
        impl Dog {
            fn new() -> Self {
                Self {
                    name: "Pochi".to_string(),
                    size: "Middle".to_string(),
                }
            }
        }
        let mut exact_dog = Dog::new();
        exact_dog.size = "Big".to_string();
        let dog = Dog::new().apply(|it| it.size = "Big".to_string());
        assert_eq!(dog, exact_dog);
    }

    #[test]
    fn test_apply_param() {
        let mut exact_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        exact_path.push("src/lib.rs");
        let path =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).apply_with_param(PathBuf::push, "src/lib.rs");
        assert_eq!(path, exact_path);
    }

    #[test]
    fn test_apply_params() {
        let mut exact_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        exact_path.push("src");
        exact_path.push("lib.rs");
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .apply_with_params(PathBuf::push, vec!["src", "lib.rs"]);
        assert_eq!(path, exact_path);
    }
}
