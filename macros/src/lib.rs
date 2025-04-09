#![doc(html_root_url = "https://docs.rs/crosstalk-macros/1.0")]
//! Macros for [`crosstalk`](https://crates.io/crates/crosstalk)
//! 
//! ## License
//! 
//! Crosstalk is released under the MIT license [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT)
// --------------------------------------------------
// external
// --------------------------------------------------
use quote::{quote, format_ident};
use syn::{
    Data,
    Path,
    Type,
    Token,
    parse::{
        Parse,
        ParseStream,
    },
    DeriveInput,
    parse_macro_input,
    punctuated::Punctuated,
};
use proc_macro::TokenStream;
use std::collections::HashSet;
use proc_macro2::TokenStream as TokenStream2;

#[proc_macro]
#[inline(always)]
/// Macro used to initialize a [`crosstalk`](https://crates.io/crates/crosstalk) node
/// 
/// This correlates variants in the enum (topics) with datatypes used to 
/// communicate on said topics. These channels broadcast messages from publishers
/// to subscribers, without catastrophic consumption of the data
/// 
/// Any variants missing from the macro will automatically be added
/// using the [`String`] datatype
/// 
/// The enum variants and the datatypes are formatted in the following fashion:
/// 
/// ```rust ignore
/// crosstalk::init!{
///     TopicEnum::Variant1 => bool,
///     TopicEnum::Variant2 => String,
///     TopicEnum::Variant3 => i32,
/// }
/// ```
/// 
/// Where `TopicEnum::<VariantName>` is the name of the enum, followed by 
/// a `=>` and the datatype to be used on that topic of communication
/// 
/// # Examples
/// 
/// ```ignore
/// use crosstalk::AsTopic;
/// 
/// #[derive(AsTopic)]
/// enum ExampleTopics {
///     BoolChannel,
///     StringChannel,
///     IntChannel,
///     MissingChannel,
/// }
/// 
/// crosstalk::init!{
///     ExampleTopics::BoolChannel => bool,
///     ExampleTopics::StringChannel => String,
///     ExampleTopics::IntChannel => i32,
/// }
/// // `ExampleTopics::MissingChannel`` will be added automatically with datatype `String``
/// ```
pub fn init(input: TokenStream) -> TokenStream {
    init_inner(input, quote!(::crosstalk::))
}

#[proc_macro]
#[inline(always)]
/// The [`init`] macro for testing. This is meant
/// to be internal to `crosstalk` only.
/// 
/// See [`init`] for actual usage
pub fn init_test(input: TokenStream) -> TokenStream {
    init_inner(input, quote!(crate::))
}

/// Internal implementation
/// 
/// This is only used for testing, so that `mod tests;` can use `crosstalk::init!`
/// expansion, replacing all instances of `crosstalk` with `crate`
/// 
/// See [`init`] for the proper usage
fn init_inner(input: TokenStream, source: TokenStream2) -> TokenStream {
    // --------------------------------------------------
    // parse
    // --------------------------------------------------
    let NodeFields(fields) = parse_macro_input!(input as NodeFields);
    
    // --------------------------------------------------
    // see if there are multiple enums for topics
    // --------------------------------------------------
    let unique_enum_names = fields
        .iter()
        .map(|nf| nf
            .topic
            .segments
            .first()
            .map(|s|
                s
                .ident
                .to_string()
            ).expect("Expected enum name to be path-like")
        )
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    if unique_enum_names.len() > 1 {
        let error_desc = "Multiple topic enum's found in crosstalk node initialization";
        let error_msg = "Please use only one enum to represent topics, and use the absolute path to the enum (e.g. `TopicEnum::MyTopic` instead of `MyTopic`)";
        panic!("\n{}\nFound:\n{:#?}\n{}", error_desc, unique_enum_names, error_msg);
    }
    let enum_master = format_ident!( "{}", &unique_enum_names[0]);

    // --------------------------------------------------
    // get topic names/types
    // --------------------------------------------------
    let nt = fields
        .iter()
        .map(|nf| (nf.topic.clone(), nf.dtype.clone()))
        .collect::<Vec<_>>();

    // --------------------------------------------------
    // default type 
    // --------------------------------------------------
    let dt: Type = syn::parse_quote! { String };

    // --------------------------------------------------
    // publisher arms
    // - add default case
    // --------------------------------------------------
    let mut pub_arms: Vec<TokenStream2> = nt
        .iter()
        .map(|(n, t)| get_publisher_arm(Some(n), t, &source))
        .collect();
    pub_arms.push(get_publisher_arm(None, &dt, &source));

    // --------------------------------------------------
    // subscriber arms
    // - add default case
    // --------------------------------------------------
    let mut sub_arms: Vec<TokenStream2> = nt
        .iter()
        .map(|(n, t)| get_subscriber_arm(Some(n), t, &source))
        .collect::<Vec<_>>();
    sub_arms.push(get_subscriber_arm(None, &dt, &source));

    // --------------------------------------------------
    // output
    // --------------------------------------------------
    let output: TokenStream2 = quote! {
        #[automatically_derived]
        impl #source CrosstalkPubSub<#enum_master> for #source ImplementedBoundedNode<#enum_master> {
            #[doc = " Get a [`crosstalk::Publisher`] for the given topic"]
            #[doc = ""]
            #[doc = " See [`crosstalk::BoundedNode::publisher`] for more information"]
            fn publisher<D: #source CrosstalkData>(&mut self, topic: #enum_master) -> Result<#source Publisher<D, #enum_master>, #source Error> {
                match topic {
                    #(#pub_arms,)*
                }
            }
            
            #[doc = " Get a [`crosstalk::Subscriber`] for the given topic"]
            #[doc = ""]
            #[doc = " See [`crosstalk::BoundedNode::subscriber`] for more information"]
            fn subscriber<D: #source CrosstalkData>(&mut self, topic: #enum_master) -> Result<#source Subscriber<D, #enum_master>, #source Error> {
                match topic {
                    #(#sub_arms,)*
                }
            }
            
            #[inline(always)]
            #[doc = " Get a [`crosstalk::Publisher`] and [`crosstalk::Subscriber`] for the given topic"]
            #[doc = ""]
            #[doc = " See [`crosstalk::BoundedNode::pubsub`] for more information"]
            fn pubsub<D: #source CrosstalkData>(&mut self, topic: #enum_master) -> Result<(#source Publisher<D, #enum_master>, #source Subscriber<D, #enum_master>), #source Error> {
                match (self.publisher(topic), self.subscriber(topic)) {
                    (Ok(publisher), Ok(subscriber)) => Ok((publisher, subscriber)),
                    (Err(err), _) => Err(err),
                    (_, Err(err)) => Err(err),
                    _ => unreachable!(),
                }
            }
        }
    };

    // --------------------------------------------------
    // return
    // --------------------------------------------------
    TokenStream::from(output)
}

#[proc_macro_derive(AsTopic)]
#[inline(always)]
/// The [`AsTopic`] derive macro
/// 
/// This is used to distinguish enums as "crosstalk topics"
/// by implementing the [`crosstalk::CrosstalkTopic`] trait
/// amongst other traits
/// 
/// This will automatically implement the following traits:
/// 
/// * Clone
/// * Copy
/// * PartialEq
/// * Eq
/// * Hash
///
/// # Example
/// 
/// ```ignore
/// use crosstalk::AsTopic;
/// 
/// #[derive(AsTopic)]
/// enum ExampleTopics {
///     BoolChannel,
///     StringChannel,
///     IntChannel,
///     MissingChannel,
/// }
/// ```
pub fn derive_enum_as_topic(input: TokenStream) -> TokenStream {
    derive_enum_as_topic_inner(input, quote!(::crosstalk::))
}

#[proc_macro_derive(AsTopicTest)]
#[inline(always)]
/// The [`AsTopicTest`] derive macro for testing. This is meant
/// to be internal to crosstalk only.
/// 
/// See [`derive_enum_as_topic`] for actual usage
pub fn derive_enum_as_topic_test(input: TokenStream) -> TokenStream {
    derive_enum_as_topic_inner(input, quote!(crate::))
}

/// Internal implementation
/// 
/// This is only used for testing, so that `mod tests;` can implement `crosstalk::CrosstalkTopic`
/// as `crate::CrosstalkTopic`
/// 
/// See [`derive_enum_as_topic`] for the proper usage
fn derive_enum_as_topic_inner(input: TokenStream, source: TokenStream2) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    let Data::Enum(_) = &input.data else {
        panic!("CrosstalkTopic can only be derived on enums");
    };
    
    let name = &input.ident;

    let expanded = quote! {
        #[automatically_derived]
        impl ::core::clone::Clone for #name {
            #[inline]
            fn clone(&self) -> #name { *self }
        }

        #[automatically_derived]
        impl ::core::marker::Copy for #name {}
        
        // TODO: impl this once stable
        // #[automatically_derived]
        // impl ::core::marker::StructuralPartialEq for #name {}
        
        #[automatically_derived]
        impl ::core::cmp::PartialEq for #name {
            #[inline]
            fn eq(&self, other: &#name) -> bool {
                let __self_tag = ::std::mem::discriminant(self);
                let __arg1_tag = ::std::mem::discriminant(other);
                __self_tag == __arg1_tag
            }
        }
        
        #[automatically_derived]
        impl ::core::cmp::Eq for #name { }
        
        #[automatically_derived]
        impl ::core::hash::Hash for #name {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
                let __self_tag = ::std::mem::discriminant(self);
                ::core::hash::Hash::hash(&__self_tag, state)
            }
        }
        
        #[automatically_derived]
        impl #source CrosstalkTopic for #name {}
    };

    TokenStream::from(expanded)
}

/// Individual field for the [`crosstalk_macros::init!`] macro
/// 
/// # Format
/// 
/// ```text
/// `<Enum>::<Variant> => <Type>`
/// ```
struct NodeField {
    topic: Path,
    _arrow: Token![=>],
    dtype: Type,
}
/// [`NodeField`] implementation of [`syn::parse::Parse`]
impl Parse for NodeField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(NodeField {
            topic: input.parse()?,
            _arrow: input.parse()?,
            dtype: input.parse()?,
        })
    }
}

/// Fields for the [`crosstalk_macros::init!`] macro
/// 
/// # Format
/// 
/// ```text
/// crosstalk_macros::init!{
///     `<Enum>::<Variant> => <Type>`,
///     `<Enum>::<Variant> => <Type>`,
/// }
/// ```
struct NodeFields(Punctuated<NodeField, Token![,]>);

/// [`NodeFields`] implementation of [`syn::parse::Parse`]
impl Parse for NodeFields {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content = Punctuated::<NodeField, Token![,]>::parse_terminated(input)?;
        Ok(NodeFields(content))
    }
}

/// Get publisher arm (used in type-matching within the [`crosstalk_macros::init!`] macro)
/// 
/// This helps fill in the `match` statement in the [`crosstalk_macros::init!`] macro
/// with all the arms that are valid for a given topic and datatype
fn get_publisher_arm(case: Option<&Path>, dtype: &Type, source: &TokenStream2) -> TokenStream2 {
    let contents = quote! {
        => {
            let err = #source Error::PublisherMismatch(
                ::std::any::type_name::<D>(),
                ::std::any::type_name::<#dtype>(),
            );
            if ::std::any::TypeId::of::<D>()
            == ::std::any::TypeId::of::<#dtype>() {
                // --------------------------------------------------
                // if the datatype matches, get the tokio brdcst sender
                // to create the subscriber
                // --------------------------------------------------
                let tsen = match self.senders.contains_key(&topic) {
                    true => {
                        #[allow(clippy::unwrap_used)]
                        // can't use .get here because it returns a reference,
                        // and need to consume the value in order to downcast
                        // it. as a result, .contains_key() is used in junction
                        // with .remove.unwrap()
                        let tsen_ = self.senders.remove(&topic).unwrap();
                        let tsen_ = #source __macro_exports::downcast::<#source __macro_exports::broadcast::Sender<#dtype>>(tsen_, err)?;
                        let tsen = tsen_.clone();
                        self.senders.insert(topic, Box::new(tsen_));
                        tsen
                    },
                    false => {
                        // size is defined during crosstalk::BoundedNode::new(size)
                        let (sender, _) =  #source __macro_exports::broadcast::channel::<#dtype>(self.size);
                        self.senders.insert(topic, Box::new(sender.clone()));
                        sender
                    },
                };
                // --------------------------------------------------
                // create and return publisher
                // --------------------------------------------------
                // this downcasts from #dtype -> D. These are the same type,
                // due to check made above
                // --------------------------------------------------
                let sender = #source __macro_exports::downcast::<#source __macro_exports::broadcast::Sender<D>>(Box::new(tsen), err)?;
                Ok(#source Publisher::new(topic, sender))
            } else {
                // --------------------------------------------------
                // if the datatype does not match, return an error
                // --------------------------------------------------
                Err(err)
            }
        }
    };
    // --------------------------------------------------
    // if arm not specified, use default (_)
    // --------------------------------------------------
    match case {
        Some(case) => quote! { #case #contents },
        None => quote! { _ #contents }
    }
}

/// Get subscriber arm (used in type-matching within the [`crosstalk_macros::init!`] macro)
/// 
/// This helps fill in the `match` statement in the [`crosstalk_macros::init!`] macro
/// with all the arms that are valid for a given topic and datatype
fn get_subscriber_arm(case: Option<&Path>, dtype: &Type, source: &TokenStream2) -> TokenStream2 {
    let contents = quote! {
        => {
            let err = #source Error::SubscriberMismatch(
                ::std::any::type_name::<D>(),
                ::std::any::type_name::<#dtype>(),
            );
            if ::std::any::TypeId::of::<D>()
            == ::std::any::TypeId::of::<#dtype>() {
                // --------------------------------------------------
                // if the datatype matches, get the tokio brdcst sender
                // to create the subscriber
                // --------------------------------------------------
                let tsen = match self.senders.contains_key(&topic) {
                    true => {
                        #[allow(clippy::unwrap_used)]
                        // can't use .get here because it returns a reference,
                        // and need to consume the value in order to downcast
                        // it. as a result, .contains_key() is used in junction
                        // with .remove.unwrap()
                        let tsen_ = self.senders.remove(&topic).unwrap();
                        let tsen_ = #source __macro_exports::downcast::<#source __macro_exports::broadcast::Sender<#dtype>>(tsen_, err)?;
                        let tsen = tsen_.clone();
                        self.senders.insert(topic, Box::new(tsen_));
                        tsen
                    },
                    false => {
                        // size is defined during crosstalk::BoundedNode::new(size)
                        let (sender, _) =  #source __macro_exports::broadcast::channel::<#dtype>(self.size);
                        self.senders.insert(topic, Box::new(sender.clone()));
                        sender
                    },
                };
                // --------------------------------------------------
                // create and return subscriber
                // --------------------------------------------------
                // this downcasts from #dtype -> D. These are the same type,
                // due to check made above
                // --------------------------------------------------
                let sender = #source __macro_exports::downcast::<#source __macro_exports::broadcast::Sender<D>>(Box::new(tsen), err)?;
                Ok(#source Subscriber::new(topic, None, ::std::sync::Arc::new(sender)))
            } else {
                // --------------------------------------------------
                // if the datatype does not match, return an error
                // --------------------------------------------------
                Err(err)
            }
        }
    };
    // --------------------------------------------------
    // if arm not specified, use default (_)
    // --------------------------------------------------
    match case {
        Some(case) => quote! { #case #contents },
        None => quote! { _ #contents }
    }
}
