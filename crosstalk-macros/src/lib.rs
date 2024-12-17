//! Macros for [`crosstalk`](https://crates.io/crates/crosstalk)
//! 
//! ## License
//! 
//! Crosstalk is released under the MIT license [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT)
// --------------------------------------------------
// external
// --------------------------------------------------
use quote::{
    quote,
    format_ident,
};
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
use proc_macro2::Ident;
use proc_macro::TokenStream;
use std::collections::HashSet;
use proc_macro2::TokenStream as TokenStream2;

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
fn get_publisher_arm(tenum: &Ident, case: Option<&Path>, dtype: &Type) -> TokenStream2 {
    let contents = quote! {
        => {
            let err = ::crosstalk::Error::PublisherMismatch(
                ::std::any::type_name::<D>().into(),
                ::std::any::type_name::<#dtype>().into(),
            );
            if ::std::any::TypeId::of::<D>()
            == ::std::any::TypeId::of::<#dtype>() {
                // --------------------------------------------------
                // if the datatype matches, get the flume sender to create the publisher
                // --------------------------------------------------
                let tsen = match self.senders.contains_key(&topic) {
                    true => {
                        let tsen_ = ::crosstalk::downcast::<
                                    ::crosstalk::external::broadcast::Sender<#dtype>
                                    >(self.senders.remove(&topic).unwrap()).unwrap();
                        let tsen = tsen_.clone();
                        self.senders.insert(topic, Box::new(tsen_));
                        tsen
                    },
                    false => {
                        // size is defined during crosstalk::BoundedNode::new(size)
                        let (sender, _) =  ::crosstalk::external::broadcast::channel::<#dtype>(self.size);
                        self.senders.insert(topic, Box::new(sender.clone()));
                        sender
                    }
                };
                // --------------------------------------------------
                // create and return publisher
                // --------------------------------------------------
                match ::crosstalk::downcast::<::crosstalk::Publisher<D, #tenum>>(Box::new(::crosstalk::Publisher::new(tsen, topic))) {
                    Ok(publisher) => Ok(publisher),
                    Err(_) => Err(Box::new(err)), // <-- this should never happen
                }
            } else {
                // --------------------------------------------------
                // if the datatype does not match, return an error
                // --------------------------------------------------
                Err(Box::new(err))
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
fn get_subscriber_arm(case: Option<&Path>, dtype: &Type) -> TokenStream2 {
    let contents = quote! {
        => {
            let err = ::crosstalk::Error::SubscriberMismatch(
                ::std::any::type_name::<D>().into(),
                ::std::any::type_name::<#dtype>().into(),
            );
            if ::std::any::TypeId::of::<D>()
            == ::std::any::TypeId::of::<#dtype>() {
                // --------------------------------------------------
                // if the datatype matches, get the flume receiver to create the subscriber
                // --------------------------------------------------
                let tsen = match self.senders.contains_key(&topic) {
                    true => {
                        let tsen_ = ::crosstalk::downcast::<
                                    ::crosstalk::external::broadcast::Sender<#dtype>
                                    >(self.senders.remove(&topic).unwrap()).unwrap();
                        let tsen = tsen_.clone();
                        self.senders.insert(topic, Box::new(tsen_));
                        tsen
                    },
                    false => {
                        // size is defined during crosstalk::BoundedNode::new(size)
                        let (sender, _) =  ::crosstalk::external::broadcast::channel::<#dtype>(self.size);
                        self.senders.insert(topic, Box::new(sender.clone()));
                        sender
                    }
                };
                // --------------------------------------------------
                // create and return subscriber
                // --------------------------------------------------
                match ::crosstalk::downcast::<::crosstalk::external::broadcast::Sender<D>>(Box::new(tsen)) {
                    Ok(sender) => Ok(::crosstalk::Subscriber::new(topic, None, ::std::sync::Arc::new(sender))),
                    Err(_) => Err(Box::new(err)), // <-- this should never happen
                }
            } else {
                // --------------------------------------------------
                // if the datatype does not match, return an error
                // --------------------------------------------------
                Err(Box::new(err))
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

#[proc_macro]
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
    // --------------------------------------------------
    // parse
    // --------------------------------------------------
    let NodeFields(fields) = parse_macro_input!(input as NodeFields);
    
    // --------------------------------------------------
    // see if there are multiple enums for topics
    // --------------------------------------------------
    let unique_enum_names = fields
        .iter()
        .map(|nf|
            nf
            .topic
            .segments
            .first()
            .map(|s|
                s
                .ident
                .to_string()
            ).unwrap()
        )
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    if unique_enum_names.len() > 1 {
        // TODO: change this to use crosstalk::Error
        let error_desc = "Multiple Topic Enums found in constructing crosstalk node";
        let error_msg = "Please use only one Enum to represent Topics, and use the absolute path to the Enum (e.g. `TopicEnum::MyTopic` instead of `MyTopic`)";
        panic!("\n{}\nFound:\n{:#?}\n{}", error_desc, unique_enum_names, error_msg);
    }
    let enum_master = format_ident!( "{}", &unique_enum_names[0]);

    // --------------------------------------------------
    // get topic names/types
    // --------------------------------------------------
    let nt = fields
        .iter()
        .map(|nf| (&nf.topic, &nf.dtype))
        .map(|(n, t)| (
            n.to_owned(),
            t.to_owned(),
        ))
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
        .map(|(n, t)| {
            get_publisher_arm(&enum_master, Some(n), t)
        }).collect();
    pub_arms.push(get_publisher_arm(&enum_master, None, &dt));

    // --------------------------------------------------
    // subscriber arms
    // - add default case
    // --------------------------------------------------
    let mut sub_arms: Vec<TokenStream2> = nt
        .iter()
        .map(|(n, t)| get_subscriber_arm(Some(n), t))
        .collect::<Vec<_>>();
    sub_arms.push(get_subscriber_arm(None, &dt));

    // --------------------------------------------------
    // output
    // --------------------------------------------------
    let output: TokenStream2 = quote! {
        #[automatically_derived]
        impl ::crosstalk::CrosstalkPubSub<#enum_master> for ::crosstalk::ImplementedBoundedNode<#enum_master> {
            #[doc = " Get a [`crosstalk::Publisher`] for the given topic"]
            #[doc = ""]
            #[doc = " See [`crosstalk::BoundedNode::publisher`] for more information"]
            fn publisher<D: 'static>(&mut self, topic: #enum_master) -> Result<::crosstalk::Publisher<D, #enum_master>, Box<dyn ::std::error::Error>> {
                match topic {
                    #(#pub_arms,)*
                }
            }
            
            #[doc = " Get a [`crosstalk::Subscriber`] for the given topic"]
            #[doc = ""]
            #[doc = " See [`crosstalk::BoundedNode::subscriber`] for more information"]
            fn subscriber<D: Clone + Send + 'static>(&mut self, topic: #enum_master) -> Result<::crosstalk::Subscriber<D, #enum_master>, Box<dyn ::std::error::Error>> {
                match topic {
                    #(#sub_arms,)*
                }
            }
            
            #[doc = " Get a [`crosstalk::Publisher`] and [`crosstalk::Subscriber`] for the given topic"]
            #[doc = ""]
            #[doc = " See [`crosstalk::BoundedNode::pubsub`] for more information"]
            fn pubsub<D: Clone + Send + 'static>(&mut self, topic: #enum_master) -> Result<(::crosstalk::Publisher<D, #enum_master>, ::crosstalk::Subscriber<D, #enum_master>), Box<dyn ::std::error::Error>> {
                match self.publisher(topic) {
                    Ok(publisher) => {
                        match self.subscriber(topic) {
                            Ok(subscriber) => Ok((publisher, subscriber)),
                            Err(err) => Err(err),
                        }
                    },
                    Err(err) => Err(err),
                }
            }
            
            #[doc = " Delete a [`crosstalk::Publisher`] for the given topic"]
            #[doc = ""]
            #[doc = " See [`crosstalk::BoundedNode::delete_publisher`] for more information"]
            fn delete_publisher<D: 'static>(&self, publisher: ::crosstalk::Publisher<D, #enum_master>) {}

            #[doc = " Delete a [`crosstalk::Subscriber`] for the given topic"]
            #[doc = ""]
            #[doc = " See [`crosstalk::BoundedNode::delete_subscriber`] for more information"]
            fn delete_subscriber<D: Clone + Send + 'static>(&self, subscriber: ::crosstalk::Subscriber<D, #enum_master>) {}
        }
    };

    // --------------------------------------------------
    // return
    // --------------------------------------------------
    TokenStream::from(output)
}


#[proc_macro_derive(AsTopic)]
/// The [`AsTopic`] derive macro
/// 
/// This is used to distinguish enums as "crosstalk topics"
/// by implementing the [`crosstalk::CrosstalkTopic`] trait
/// amongst other traits
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
    let input = parse_macro_input!(input as DeriveInput);

    // ensure only dealing with enums
    if let Data::Enum(_) = &input.data {
    } else { panic!("CrosstalkTopic can only be derived on enums"); }
    // TODO: get error from crosstalk errors

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
        impl ::crosstalk::CrosstalkTopic for #name {}
    };

    TokenStream::from(expanded)
}
