// --------------------------------------------------
// external
// --------------------------------------------------
use quote::{
    quote,
    format_ident,
};
use syn::{
    Path,
    Type,
    Token,
    parse::{
        Parse,
        ParseStream,
    },
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
            let err = crosstalk::Error::PublisherMismatch(std::any::type_name::<D>().into(), std::any::type_name::<#dtype>().into());
            if std::any::TypeId::of::<D>() == std::any::TypeId::of::<#dtype>() {
                // --------------------------------------------------
                // get the flume sender
                // --------------------------------------------------
                let fsen = match self.senders.contains_key(&topic) {
                    true => {
                        let fsen_ = crosstalk::downcast::<flume::Sender<#dtype>>(self.senders.remove(&topic).unwrap()).unwrap();
                        let fsen = fsen_.clone();
                        self.senders.insert(topic, Box::new(fsen_));
                        fsen
                    },
                    false => {
                        let (sender, receiver) =  flume::unbounded::<#dtype>();
                        self.senders.insert(topic, Box::new(sender.clone()));
                        self.receivers.insert(topic, Box::new(receiver));
                        sender
                    }
                };
                match crosstalk::downcast::<crosstalk::Publisher<D, #tenum>>(Box::new(crosstalk::Publisher::new(fsen, topic))) {
                    Ok(publisher) => Ok(publisher),
                    Err(_e) => { // <-- this should never happen
                        tracing::error!(?err);
                        Err(Box::new(err))
                    }
                }
            } else {
                tracing::error!(?err);
                Err(Box::new(err))
            }
        }
    };
    match case {
        Some(case) => quote! { #case #contents },
        None => quote! { _ #contents }
    }
}


fn get_subscriber_arm(case: Option<&Path>, dtype: &Type) -> TokenStream2 {
    let contents = quote! {
         => {
            let err = crosstalk::Error::SubscriberMismatch(std::any::type_name::<D>().into(), std::any::type_name::<#dtype>().into());
            if std::any::TypeId::of::<D>() == std::any::TypeId::of::<#dtype>() {
                // --------------------------------------------------
                // get the flume receiver
                // --------------------------------------------------
                let frec = match self.receivers.contains_key(&topic) {
                    true => {
                        let frec_ = crosstalk::downcast::<flume::Receiver<#dtype>>(self.receivers.remove(&topic).unwrap()).unwrap();
                        let frec = frec_.clone();
                        self.receivers.insert(topic, Box::new(frec_));
                        frec
                    },
                    false => {
                        let (sender, receiver) = flume::unbounded::<#dtype>();
                        self.senders.insert(topic, Box::new(sender));
                        self.receivers.insert(topic, Box::new(receiver.clone()));
                        receiver
                    }
                };
                // --------------------------------------------------
                // get the distribution id and the crossbeam receiver
                // --------------------------------------------------
                let mut update_threads = false;
                let (ndist, did, crec) = match self.distributors.contains_key(&topic) {
                    true => {
                        // --------------------------------------------------
                        // get the distribution
                        // --------------------------------------------------
                        let dists = self.distributors.get_mut(&topic).unwrap();
                        // --------------------------------------------------
                        // create sender / receiver
                        // --------------------------------------------------
                        // let (sender, receiver) = crossbeam::channel::unbounded::<#dtype>();
                        let (sender, receiver) = flume::unbounded::<#dtype>();
                        // --------------------------------------------------
                        // get the unique distributor id, increment it
                        // --------------------------------------------------
                        let did = self.uniq_dist_id_incr.get_mut(&topic).unwrap();
                        *did += 1;
                        // --------------------------------------------------
                        // insert distributor
                        // --------------------------------------------------
                        dists.insert(*did, Box::new(sender));
                        // --------------------------------------------------
                        // update number of distributors
                        // --------------------------------------------------
                        let ndist = dists.len();
                        tracing::debug!("Updating num distributors from {} to {}", self.num_dist_per_topic.get(&topic).unwrap().load(std::sync::atomic::Ordering::SeqCst), ndist.clone());
                        self.num_dist_per_topic.get_mut(&topic).unwrap().store(ndist.clone(), std::sync::atomic::Ordering::SeqCst);
                        // --------------------------------------------------
                        // set all termination flags to false, if needed
                        // --------------------------------------------------
                        update_threads = ndist > 1; // { self.update_distribution_threads::<D>(&topic) };
                        // --------------------------------------------------
                        // return
                        // --------------------------------------------------
                        (ndist, did.clone(), receiver)
                    },
                    false => {
                        // --------------------------------------------------
                        // create sender / receiver
                        // --------------------------------------------------
                        // let (sender, receiver) = crossbeam::channel::unbounded::<#dtype>();
                        let (sender, receiver) = flume::unbounded::<#dtype>();
                        // --------------------------------------------------
                        // get the unique distributor id
                        // --------------------------------------------------
                        let did = 0;
                        self.uniq_dist_id_incr.insert(topic, did.clone());
                        // --------------------------------------------------
                        // insert distributor
                        // --------------------------------------------------
                        self.distributors.insert(topic, std::collections::HashMap::new());
                        let dists = self.distributors.get_mut(&topic).unwrap();
                        dists.insert(did.clone(), Box::new(sender));
                        // --------------------------------------------------
                        // set number of distributors
                        // --------------------------------------------------
                        let ndist = dists.len();
                        tracing::debug!("Updating num distributors from <nothing> to {}", ndist.clone());
                        self.num_dist_per_topic.insert(topic, std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(ndist.clone())));
                        // --------------------------------------------------
                        // return
                        // --------------------------------------------------
                        (ndist, did.clone(), receiver)
                    }
                };
                // --------------------------------------------------
                // set all termination flags to false, if needed
                // --------------------------------------------------
                if update_threads {
                    tracing::debug!("Telling distribution threads to kill thread with ndist={}", ndist - 1);
                    self.update_distribution_threads::<D>(&topic, Some(ndist - 1))
                };
                // --------------------------------------------------
                // return
                // --------------------------------------------------
                match crosstalk::downcast::<crosstalk::Receiver<D>>(Box::new(crosstalk::Receiver::new(frec, self.num_dist_per_topic.get(&topic).unwrap().clone(), crec))) {
                    Ok(rec) => Ok(crosstalk::Subscriber::new(did, rec, topic)),
                    Err(_e) => { // <-- this should never happen
                        tracing::error!(?err);
                        Err(Box::new(err))
                    }
                }
            } else {
                tracing::error!(?err);
                Err(Box::new(err))
            }
        }
    };
    match case {
        Some(case) => quote! { #case #contents },
        None => quote! { _ #contents }
    }
}


#[proc_macro]
pub fn unbounded(input: TokenStream) -> TokenStream {
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
        {

            impl crosstalk::PubSub<#enum_master> for crosstalk::UnboundedCommonNode<#enum_master> {

                fn publisher<D: 'static>(&mut self, topic: #enum_master) -> Result<crosstalk::Publisher<D, #enum_master>, Box<dyn std::error::Error>> {
                    match topic {
                        #(#pub_arms,)*
                    }
                }

                fn subscriber<D: Clone + Send + 'static>(&mut self, topic: #enum_master) -> Result<crosstalk::Subscriber<D, #enum_master>, Box<dyn std::error::Error>> {
                    match topic {
                        #(#sub_arms,)*
                    }
                }
            
                fn delete_publisher<D: 'static>(&mut self, _publisher: crosstalk::Publisher<D, #enum_master>) {}
            
                fn delete_subscriber<D: Clone + Send + 'static>(&mut self, subscriber: crosstalk::Subscriber<D, #enum_master>) {
                    let topic = subscriber.topic.clone();
                    if let Some(dists) = self.distributors.get_mut(&topic) {
                        dists.remove(&subscriber.id);
                    }
                    if let Some(ndist) = self.num_dist_per_topic.get(&topic) {
                        let n = ndist.fetch_update(std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst, |x| Some(x - 1));
                        match n {
                            Ok(n) => match n {
                                // 0 | 1 => self.set_termination_flag(&topic),
                                0..=2 => { let _ = self.restart_forwarding(&topic, Some(n)); },
                                _ => self.update_distribution_threads::<D>(&topic, Some(n)),
                            },
                            Err(_) => {}
                        }
                    }
                }
            }

            crosstalk::UnboundedCommonNode::<#enum_master>::new()

        }
    };

    // --------------------------------------------------
    // return
    // --------------------------------------------------
    TokenStream::from(output)
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
