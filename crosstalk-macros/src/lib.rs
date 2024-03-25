//! # Crosstalk Macros
//!
//! This crate provides macros for simplifying communication between threads using the Crosstalk library.
//!
//! ## Benchmarks
//!
//! The benchmarks module contains functions to benchmark different communication scenarios.
//!
//! ## External Dependencies
//!
//! This file imports external dependencies such as `quote` and `syn` for code generation and parsing.
//!
//! ## Functionality
//!
//! This file contains various functions for handling message transmission and reception between publishers and subscribers.
//! The functions are designed to work efficiently and return when messages are successfully received.
//! There are also functions for draining messages from subscribers and handling multiple publishers and subscribers.
//!
//! ## Constants
//!
//! The file defines constants for the number of publishers, subscribers, and messages used in the benchmarks.
//!
//! ## Custom Structs
//!
//! There are examples of custom structs like `DetectorOutput`, though they are currently commented out for future use.
//!
//! ## Topics Enum
//!
//! An example enum `TopicZoo` is defined for different topics that can be communicated between threads.
//!
//! ## Utility Functions
//!
//! Utility functions like `write` and `read` are provided for writing messages to publishers and reading messages from subscribers.
//!
//! ## Error Handling
//!
//! The code includes error handling for downcasting message types using the `downcast` function.
//!
//! ## Benchmarking
//!
//! Benchmarks are set up for various communication scenarios to measure performance and efficiency.
//!
//! ## Thread Management
//!
//! The file manages threads for communication between publishers and subscribers, ensuring messages are received and processed correctly.
//!
//! ## License
//!
//! This code is licensed under the MIT license.

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
    PathArguments,
    GenericArgument,
    parse_macro_input,
    punctuated::Punctuated,
};
use proc_macro2::Ident;
use proc_macro::TokenStream;
use std::collections::HashSet;
use proc_macro2::TokenStream as TokenStream2;


#[derive(Debug)]
struct NodeField {
    topic: Path,
    _arrow: Token![=>],
    dtype: Type,
}
impl Parse for NodeField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(NodeField {
            topic: input.parse()?,
            _arrow: input.parse()?,
            dtype: input.parse()?,
        })
    }
}


struct NodeFields(Punctuated<NodeField, Token![,]>);
impl Parse for NodeFields {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content = Punctuated::<NodeField, Token![,]>::parse_terminated(input)?;
        Ok(NodeFields(content))
    }
}


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
                        if self.create_runtimes {
                            let rt = match ::crosstalk::external::runtime::Runtime::new() {
                                Ok(rt) => ::std::sync::Arc::new(rt),
                                Err(err) => {
                                    ::crosstalk::external::log::error!("{}", err);
                                    return Err(Box::new(err))
                                }
                            };
                            self.runtimes.insert(topic, rt);
                        }
                        sender
                    }
                };
                // --------------------------------------------------
                // create and return publisher
                // --------------------------------------------------
                match ::crosstalk::downcast::<::crosstalk::Publisher<D, #tenum>>(Box::new(::crosstalk::Publisher::new(tsen, topic))) {
                    Ok(publisher) => Ok(publisher),
                    Err(_e) => { // <-- this should never happen
                        ::crosstalk::external::log::error!("{}", err);
                        Err(Box::new(err))
                    }
                }
            } else {
                // --------------------------------------------------
                // if the datatype does not match, return an error
                // --------------------------------------------------
                ::crosstalk::external::log::error!("{}", err);
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
                let rt = match self.create_runtimes {
                    true => match self.runtimes.get(&topic) {
                            Some(rt) => Some(rt.clone()),
                            None => {
                                let rt = match ::crosstalk::external::runtime::Runtime::new() {
                                    Ok(rt) => ::std::sync::Arc::new(rt),
                                    Err(err) => {
                                        ::crosstalk::external::log::error!("{}", err);
                                        return Err(Box::new(err))
                                    }
                                };
                                self.runtimes.insert(topic, rt.clone());
                                Some(rt)
                            }
                        },
                    false => None,
                };
                // --------------------------------------------------
                // create and return subscriber
                // --------------------------------------------------
                match ::crosstalk::downcast::<::crosstalk::external::broadcast::Sender<D>>(Box::new(tsen)) {
                    Ok(sender) => Ok(::crosstalk::Subscriber::new(topic, None, ::std::sync::Arc::new(sender), rt)),
                    // Ok(rec) => Ok(::crosstalk::Subscriber::new(did, ::std::sync::Arc::new(::std::sync::Mutex::new(self)), rec, topic)),
                    Err(_e) => { // <-- this should never happen
                        ::crosstalk::external::log::error!("{}", err);
                        Err(Box::new(err))
                    }
                }
            } else {
                // --------------------------------------------------
                // if the datatype does not match, return an error
                // --------------------------------------------------
                ::crosstalk::external::log::error!("{}", err);
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

            fn publisher<D: 'static>(&mut self, topic: #enum_master) -> Result<::crosstalk::Publisher<D, #enum_master>, Box<dyn ::std::error::Error>> {
                match topic {
                    #(#pub_arms,)*
                }
            }

            fn subscriber<D: Clone + Send + 'static>(&mut self, topic: #enum_master) -> Result<::crosstalk::Subscriber<D, #enum_master>, Box<dyn ::std::error::Error>> {
                match topic {
                    #(#sub_arms,)*
                }
            }

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
        
            fn delete_publisher<D: 'static>(&mut self, _publisher: ::crosstalk::Publisher<D, #enum_master>) {}
        
            fn delete_subscriber<D: Clone + Send + 'static>(&mut self, subscriber: ::crosstalk::Subscriber<D, #enum_master>) {}

        }

    };

    // --------------------------------------------------
    // return
    // --------------------------------------------------
    TokenStream::from(output)
}


#[proc_macro_derive(AsTopic)]
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


fn _type2fish(ty: &Type) -> TokenStream2 {
    match ty {
        Type::Path(type_path) => {
            let mut tokens = TokenStream2::new();
            for (i, segment) in type_path.path.segments.iter().enumerate() {
                if i > 0 {
                    tokens.extend(quote!(::));
                }
                let ident = &segment.ident;
                tokens.extend(quote!(#ident));
                match &segment.arguments {
                    PathArguments::AngleBracketed(args) => {
                        let args_tokens: Vec<TokenStream2> = args.args.iter().map(|arg| {
                            match arg {
                                GenericArgument::Type(ty) => _type2fish(ty),
                                // Extend this match to handle other GenericArgument variants as needed
                                _ => quote!(#arg),
                            }
                        }).collect();
                        if !args_tokens.is_empty() {
                            tokens.extend(quote!(::<#(#args_tokens),*>));
                        }
                    },
                    // Handle other PathArguments variants if necessary
                    _ => {}
                }
            }
            tokens
        },
        // Extend this match to handle other Type variants as needed
        _ => quote!(#ty),
    }
}
