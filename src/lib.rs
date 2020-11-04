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
/// use apply_method::*;
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
    /// use apply_method::*;
    /// use std::path::PathBuf;
    /// let mut exact_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    /// exact_path.push("src/lib.rs");
    /// let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).apply(|it| it.push("src/lib.rs"));
    /// assert_eq!(path, exact_path);
    /// ```
    ///
    /// ```
    /// use apply_method::*;
    /// use std::path::PathBuf;
    /// let mut exact_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    /// exact_path.push("src/lib.rs");
    /// let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    ///     .apply(|it| it.push("src"))
    ///     .apply(|it| it.push("lib.rs"));
    /// assert_eq!(path, exact_path);
    /// ```
    ///
    fn apply<F, R>(self, f: F) -> Self
    where
        F: FnOnce(&mut Self) -> R;

    /// Apply the function with one parameter given as a parameter to self.
    ///
    /// # Example
    ///
    /// ```
    /// use apply_method::*;
    /// use std::path::PathBuf;
    /// let mut exact_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    /// exact_path.push("src/lib.rs");
    /// let path =
    ///     PathBuf::from(env!("CARGO_MANIFEST_DIR")).apply_with_param(PathBuf::push, "src/lib.rs");
    /// assert_eq!(path, exact_path);
    ///
    /// ```
    ///
    fn apply_with_param<F, P, R>(self, f: F, p: P) -> Self
    where
        F: FnOnce(&mut Self, P) -> R;

    /// Apply apply_with_param repeatedly to multiple parameters.
    ///
    /// # Example
    ///
    /// ```
    /// use apply_method::*;
    /// use std::path::PathBuf;
    /// let mut exact_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    /// exact_path.push("src");
    /// exact_path.push("lib.rs");
    /// let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    ///     .apply_with_params(PathBuf::push, vec!["src", "lib.rs"]);
    /// assert_eq!(path, exact_path);
    /// ```
    fn apply_with_params<F, P, R>(self, f: F, p: Vec<P>) -> Self
    where
        F: Fn(&mut Self, P) -> R;
}

impl<T> Applicable for T {
    fn apply<F, R>(self, f: F) -> Self
    where
        F: FnOnce(&mut Self) -> R,
    {
        let mut receiver = self;
        f(&mut receiver);
        receiver
    }

    fn apply_with_param<F, P, R>(self, f: F, p: P) -> Self
    where
        F: FnOnce(&mut Self, P) -> R,
    {
        let mut receiver = self;
        f(&mut receiver, p);
        receiver
    }

    fn apply_with_params<F, P, R>(self, f: F, p: Vec<P>) -> Self
    where
        F: Fn(&mut Self, P) -> R,
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
    use std::collections::HashMap;
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

    #[test]
    fn test_apply_non_unit_return_method_case() {
        let mut exact_map = HashMap::new();
        exact_map.insert(1, "one");
        exact_map.insert(2, "two");
        let map = HashMap::new()
            .apply(|it| it.insert(1, "one"))
            .apply(|it| it.insert(2, "two"));
        assert_eq!(map, exact_map);
    }
}
