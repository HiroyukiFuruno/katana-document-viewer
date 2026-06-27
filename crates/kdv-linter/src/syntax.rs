pub struct AttributeOps;

impl AttributeOps {
    pub fn has_cfg_test_attr(attrs: &[syn::Attribute]) -> bool {
        attrs.iter().any(Self::is_test_attr)
    }

    pub fn is_allow_attr(attr: &syn::Attribute) -> bool {
        if attr.path().is_ident("allow") {
            return true;
        }

        let syn::Meta::List(list) = &attr.meta else {
            return false;
        };

        list.path.is_ident("cfg_attr") && list.tokens.to_string().contains("allow")
    }

    fn is_test_attr(attr: &syn::Attribute) -> bool {
        if attr.path().is_ident("test") {
            return true;
        }

        if !attr.path().is_ident("cfg") {
            return false;
        }

        let Ok(syn::Meta::Path(path)) = attr.parse_args::<syn::Meta>() else {
            return false;
        };
        path.is_ident("test")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_cfg_test_attr_detects_test() {
        let item_attr: syn::Attribute = syn::parse_quote!(#[test]);
        let cfg_attr: syn::Attribute = syn::parse_quote!(#[cfg(test)]);
        let other_attr: syn::Attribute = syn::parse_quote!(#[cfg(feature = "a")]);

        assert!(AttributeOps::has_cfg_test_attr(&[item_attr]));
        assert!(AttributeOps::has_cfg_test_attr(&[cfg_attr]));
        assert!(!AttributeOps::has_cfg_test_attr(&[other_attr]));
    }

    #[test]
    fn is_allow_attr_detects_allow_notations() {
        let allow_attr: syn::Attribute = syn::parse_quote!(#[allow(unused_imports)]);
        let cfg_attr_allow: syn::Attribute =
            syn::parse_quote!(#[cfg_attr(test, allow(unused_imports))]);
        let normal_attr: syn::Attribute = syn::parse_quote!(#[derive(Debug)]);

        assert!(AttributeOps::is_allow_attr(&allow_attr));
        assert!(AttributeOps::is_allow_attr(&cfg_attr_allow));
        assert!(!AttributeOps::is_allow_attr(&normal_attr));
    }

    #[test]
    fn is_allow_attr_detects_path_not_list() {
        let path_attr: syn::Attribute = syn::parse_quote!(#[deprecated]);

        assert!(!AttributeOps::is_allow_attr(&path_attr));
    }

    #[test]
    fn has_cfg_test_attr_detects_non_test_attribute() {
        let derive_attr: syn::Attribute = syn::parse_quote!(#[derive(Debug)]);

        assert!(!AttributeOps::has_cfg_test_attr(&[derive_attr]));
    }
}
