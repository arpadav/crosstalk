#![feature(prelude_import)]
#![allow(dead_code)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::thread;
use std::collections::HashMap;
use crosstalk;
enum TopicZoo {
    Topic1,
    Topic2,
    Topic3,
    Topic4,
    Topic5,
    Topic6,
}
#[automatically_derived]
impl ::core::clone::Clone for TopicZoo {
    #[inline]
    fn clone(&self) -> TopicZoo {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for TopicZoo {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for TopicZoo {}
#[automatically_derived]
impl ::core::cmp::PartialEq for TopicZoo {
    #[inline]
    fn eq(&self, other: &TopicZoo) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for TopicZoo {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::hash::Hash for TopicZoo {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_tag, state)
    }
}
struct Vehicle {
    make: String,
    model: String,
    color: Color,
    wheels: u8,
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Vehicle {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Vehicle {
    #[inline]
    fn eq(&self, other: &Vehicle) -> bool {
        self.make == other.make && self.model == other.model && self.color == other.color
            && self.wheels == other.wheels
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Vehicle {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "Vehicle",
            "make",
            &self.make,
            "model",
            &self.model,
            "color",
            &self.color,
            "wheels",
            &&self.wheels,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Vehicle {
    #[inline]
    fn clone(&self) -> Vehicle {
        Vehicle {
            make: ::core::clone::Clone::clone(&self.make),
            model: ::core::clone::Clone::clone(&self.model),
            color: ::core::clone::Clone::clone(&self.color),
            wheels: ::core::clone::Clone::clone(&self.wheels),
        }
    }
}
enum Color {
    Red,
    Blue,
    Green,
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Color {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Color {
    #[inline]
    fn eq(&self, other: &Color) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Color {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                Color::Red => "Red",
                Color::Blue => "Blue",
                Color::Green => "Green",
            },
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Color {
    #[inline]
    fn clone(&self) -> Color {
        match self {
            Color::Red => Color::Red,
            Color::Blue => Color::Blue,
            Color::Green => Color::Green,
        }
    }
}
impl crosstalk::PubSub<TopicZoo> for crosstalk::InnerUnboundedCommonNode<TopicZoo> {
    fn publisher<D: 'static>(
        &mut self,
        topic: TopicZoo,
    ) -> Result<crosstalk::Publisher<D, TopicZoo>, Box<dyn std::error::Error>> {
        match topic {
            TopicZoo::Topic1 => {
                let err = crosstalk::Error::PublisherMismatch(
                    std::any::type_name::<D>().into(),
                    std::any::type_name::<Vec<u32>>().into(),
                );
                if std::any::TypeId::of::<D>() == std::any::TypeId::of::<Vec<u32>>() {
                    let fsen = match self.senders.contains_key(&topic) {
                        true => {
                            let fsen_ = crosstalk::downcast::<
                                crosstalk::flume::Sender<Vec<u32>>,
                            >(self.senders.remove(&topic).unwrap())
                                .unwrap();
                            let fsen = fsen_.clone();
                            self.senders.insert(topic, Box::new(fsen_));
                            fsen
                        }
                        false => {
                            let (sender, receiver) = crosstalk::flume::unbounded::<
                                Vec<u32>,
                            >();
                            self.senders.insert(topic, Box::new(sender.clone()));
                            self.receivers.insert(topic, Box::new(receiver));
                            sender
                        }
                    };
                    match crosstalk::downcast::<
                        crosstalk::Publisher<D, TopicZoo>,
                    >(Box::new(crosstalk::Publisher::new(fsen, topic))) {
                        Ok(publisher) => Ok(publisher),
                        Err(_e) => {
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event src/main.rs:35",
                                            "crosstalk",
                                            ::tracing::Level::ERROR,
                                            ::core::option::Option::Some("src/main.rs"),
                                            ::core::option::Option::Some(35u32),
                                            ::core::option::Option::Some("crosstalk"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["err"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::ERROR
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::ERROR
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(&debug(&err) as &dyn Value),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            Err(Box::new(err))
                        }
                    }
                } else {
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event src/main.rs:35",
                                    "crosstalk",
                                    ::tracing::Level::ERROR,
                                    ::core::option::Option::Some("src/main.rs"),
                                    ::core::option::Option::Some(35u32),
                                    ::core::option::Option::Some("crosstalk"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["err"],
                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let enabled = ::tracing::Level::ERROR
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::ERROR
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = __CALLSITE.interest();
                                !interest.is_never()
                                    && ::tracing::__macro_support::__is_enabled(
                                        __CALLSITE.metadata(),
                                        interest,
                                    )
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = __CALLSITE.metadata().fields().iter();
                                __CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &::core::iter::Iterator::next(&mut iter)
                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                ::core::option::Option::Some(&debug(&err) as &dyn Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    Err(Box::new(err))
                }
            }
            TopicZoo::Topic2 => {
                let err = crosstalk::Error::PublisherMismatch(
                    std::any::type_name::<D>().into(),
                    std::any::type_name::<String>().into(),
                );
                if std::any::TypeId::of::<D>() == std::any::TypeId::of::<String>() {
                    let fsen = match self.senders.contains_key(&topic) {
                        true => {
                            let fsen_ = crosstalk::downcast::<
                                crosstalk::flume::Sender<String>,
                            >(self.senders.remove(&topic).unwrap())
                                .unwrap();
                            let fsen = fsen_.clone();
                            self.senders.insert(topic, Box::new(fsen_));
                            fsen
                        }
                        false => {
                            let (sender, receiver) = crosstalk::flume::unbounded::<
                                String,
                            >();
                            self.senders.insert(topic, Box::new(sender.clone()));
                            self.receivers.insert(topic, Box::new(receiver));
                            sender
                        }
                    };
                    match crosstalk::downcast::<
                        crosstalk::Publisher<D, TopicZoo>,
                    >(Box::new(crosstalk::Publisher::new(fsen, topic))) {
                        Ok(publisher) => Ok(publisher),
                        Err(_e) => {
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event src/main.rs:35",
                                            "crosstalk",
                                            ::tracing::Level::ERROR,
                                            ::core::option::Option::Some("src/main.rs"),
                                            ::core::option::Option::Some(35u32),
                                            ::core::option::Option::Some("crosstalk"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["err"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::ERROR
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::ERROR
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(&debug(&err) as &dyn Value),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            Err(Box::new(err))
                        }
                    }
                } else {
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event src/main.rs:35",
                                    "crosstalk",
                                    ::tracing::Level::ERROR,
                                    ::core::option::Option::Some("src/main.rs"),
                                    ::core::option::Option::Some(35u32),
                                    ::core::option::Option::Some("crosstalk"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["err"],
                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let enabled = ::tracing::Level::ERROR
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::ERROR
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = __CALLSITE.interest();
                                !interest.is_never()
                                    && ::tracing::__macro_support::__is_enabled(
                                        __CALLSITE.metadata(),
                                        interest,
                                    )
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = __CALLSITE.metadata().fields().iter();
                                __CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &::core::iter::Iterator::next(&mut iter)
                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                ::core::option::Option::Some(&debug(&err) as &dyn Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    Err(Box::new(err))
                }
            }
            TopicZoo::Topic3 => {
                let err = crosstalk::Error::PublisherMismatch(
                    std::any::type_name::<D>().into(),
                    std::any::type_name::<Vehicle>().into(),
                );
                if std::any::TypeId::of::<D>() == std::any::TypeId::of::<Vehicle>() {
                    let fsen = match self.senders.contains_key(&topic) {
                        true => {
                            let fsen_ = crosstalk::downcast::<
                                crosstalk::flume::Sender<Vehicle>,
                            >(self.senders.remove(&topic).unwrap())
                                .unwrap();
                            let fsen = fsen_.clone();
                            self.senders.insert(topic, Box::new(fsen_));
                            fsen
                        }
                        false => {
                            let (sender, receiver) = crosstalk::flume::unbounded::<
                                Vehicle,
                            >();
                            self.senders.insert(topic, Box::new(sender.clone()));
                            self.receivers.insert(topic, Box::new(receiver));
                            sender
                        }
                    };
                    match crosstalk::downcast::<
                        crosstalk::Publisher<D, TopicZoo>,
                    >(Box::new(crosstalk::Publisher::new(fsen, topic))) {
                        Ok(publisher) => Ok(publisher),
                        Err(_e) => {
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event src/main.rs:35",
                                            "crosstalk",
                                            ::tracing::Level::ERROR,
                                            ::core::option::Option::Some("src/main.rs"),
                                            ::core::option::Option::Some(35u32),
                                            ::core::option::Option::Some("crosstalk"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["err"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::ERROR
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::ERROR
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(&debug(&err) as &dyn Value),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            Err(Box::new(err))
                        }
                    }
                } else {
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event src/main.rs:35",
                                    "crosstalk",
                                    ::tracing::Level::ERROR,
                                    ::core::option::Option::Some("src/main.rs"),
                                    ::core::option::Option::Some(35u32),
                                    ::core::option::Option::Some("crosstalk"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["err"],
                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let enabled = ::tracing::Level::ERROR
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::ERROR
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = __CALLSITE.interest();
                                !interest.is_never()
                                    && ::tracing::__macro_support::__is_enabled(
                                        __CALLSITE.metadata(),
                                        interest,
                                    )
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = __CALLSITE.metadata().fields().iter();
                                __CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &::core::iter::Iterator::next(&mut iter)
                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                ::core::option::Option::Some(&debug(&err) as &dyn Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    Err(Box::new(err))
                }
            }
            TopicZoo::Topic4 => {
                let err = crosstalk::Error::PublisherMismatch(
                    std::any::type_name::<D>().into(),
                    std::any::type_name::<HashMap<&str, Vec<Vehicle>>>().into(),
                );
                if std::any::TypeId::of::<D>()
                    == std::any::TypeId::of::<HashMap<&str, Vec<Vehicle>>>()
                {
                    let fsen = match self.senders.contains_key(&topic) {
                        true => {
                            let fsen_ = crosstalk::downcast::<
                                crosstalk::flume::Sender<HashMap<&str, Vec<Vehicle>>>,
                            >(self.senders.remove(&topic).unwrap())
                                .unwrap();
                            let fsen = fsen_.clone();
                            self.senders.insert(topic, Box::new(fsen_));
                            fsen
                        }
                        false => {
                            let (sender, receiver) = crosstalk::flume::unbounded::<
                                HashMap<&str, Vec<Vehicle>>,
                            >();
                            self.senders.insert(topic, Box::new(sender.clone()));
                            self.receivers.insert(topic, Box::new(receiver));
                            sender
                        }
                    };
                    match crosstalk::downcast::<
                        crosstalk::Publisher<D, TopicZoo>,
                    >(Box::new(crosstalk::Publisher::new(fsen, topic))) {
                        Ok(publisher) => Ok(publisher),
                        Err(_e) => {
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event src/main.rs:35",
                                            "crosstalk",
                                            ::tracing::Level::ERROR,
                                            ::core::option::Option::Some("src/main.rs"),
                                            ::core::option::Option::Some(35u32),
                                            ::core::option::Option::Some("crosstalk"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["err"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::ERROR
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::ERROR
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(&debug(&err) as &dyn Value),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            Err(Box::new(err))
                        }
                    }
                } else {
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event src/main.rs:35",
                                    "crosstalk",
                                    ::tracing::Level::ERROR,
                                    ::core::option::Option::Some("src/main.rs"),
                                    ::core::option::Option::Some(35u32),
                                    ::core::option::Option::Some("crosstalk"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["err"],
                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let enabled = ::tracing::Level::ERROR
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::ERROR
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = __CALLSITE.interest();
                                !interest.is_never()
                                    && ::tracing::__macro_support::__is_enabled(
                                        __CALLSITE.metadata(),
                                        interest,
                                    )
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = __CALLSITE.metadata().fields().iter();
                                __CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &::core::iter::Iterator::next(&mut iter)
                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                ::core::option::Option::Some(&debug(&err) as &dyn Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    Err(Box::new(err))
                }
            }
            TopicZoo::Topic5 => {
                let err = crosstalk::Error::PublisherMismatch(
                    std::any::type_name::<D>().into(),
                    std::any::type_name::<Color>().into(),
                );
                if std::any::TypeId::of::<D>() == std::any::TypeId::of::<Color>() {
                    let fsen = match self.senders.contains_key(&topic) {
                        true => {
                            let fsen_ = crosstalk::downcast::<
                                crosstalk::flume::Sender<Color>,
                            >(self.senders.remove(&topic).unwrap())
                                .unwrap();
                            let fsen = fsen_.clone();
                            self.senders.insert(topic, Box::new(fsen_));
                            fsen
                        }
                        false => {
                            let (sender, receiver) = crosstalk::flume::unbounded::<
                                Color,
                            >();
                            self.senders.insert(topic, Box::new(sender.clone()));
                            self.receivers.insert(topic, Box::new(receiver));
                            sender
                        }
                    };
                    match crosstalk::downcast::<
                        crosstalk::Publisher<D, TopicZoo>,
                    >(Box::new(crosstalk::Publisher::new(fsen, topic))) {
                        Ok(publisher) => Ok(publisher),
                        Err(_e) => {
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event src/main.rs:35",
                                            "crosstalk",
                                            ::tracing::Level::ERROR,
                                            ::core::option::Option::Some("src/main.rs"),
                                            ::core::option::Option::Some(35u32),
                                            ::core::option::Option::Some("crosstalk"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["err"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::ERROR
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::ERROR
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(&debug(&err) as &dyn Value),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            Err(Box::new(err))
                        }
                    }
                } else {
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event src/main.rs:35",
                                    "crosstalk",
                                    ::tracing::Level::ERROR,
                                    ::core::option::Option::Some("src/main.rs"),
                                    ::core::option::Option::Some(35u32),
                                    ::core::option::Option::Some("crosstalk"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["err"],
                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let enabled = ::tracing::Level::ERROR
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::ERROR
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = __CALLSITE.interest();
                                !interest.is_never()
                                    && ::tracing::__macro_support::__is_enabled(
                                        __CALLSITE.metadata(),
                                        interest,
                                    )
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = __CALLSITE.metadata().fields().iter();
                                __CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &::core::iter::Iterator::next(&mut iter)
                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                ::core::option::Option::Some(&debug(&err) as &dyn Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    Err(Box::new(err))
                }
            }
            _ => {
                let err = crosstalk::Error::PublisherMismatch(
                    std::any::type_name::<D>().into(),
                    std::any::type_name::<String>().into(),
                );
                if std::any::TypeId::of::<D>() == std::any::TypeId::of::<String>() {
                    let fsen = match self.senders.contains_key(&topic) {
                        true => {
                            let fsen_ = crosstalk::downcast::<
                                crosstalk::flume::Sender<String>,
                            >(self.senders.remove(&topic).unwrap())
                                .unwrap();
                            let fsen = fsen_.clone();
                            self.senders.insert(topic, Box::new(fsen_));
                            fsen
                        }
                        false => {
                            let (sender, receiver) = crosstalk::flume::unbounded::<
                                String,
                            >();
                            self.senders.insert(topic, Box::new(sender.clone()));
                            self.receivers.insert(topic, Box::new(receiver));
                            sender
                        }
                    };
                    match crosstalk::downcast::<
                        crosstalk::Publisher<D, TopicZoo>,
                    >(Box::new(crosstalk::Publisher::new(fsen, topic))) {
                        Ok(publisher) => Ok(publisher),
                        Err(_e) => {
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event src/main.rs:35",
                                            "crosstalk",
                                            ::tracing::Level::ERROR,
                                            ::core::option::Option::Some("src/main.rs"),
                                            ::core::option::Option::Some(35u32),
                                            ::core::option::Option::Some("crosstalk"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["err"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::ERROR
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::ERROR
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(&debug(&err) as &dyn Value),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            Err(Box::new(err))
                        }
                    }
                } else {
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event src/main.rs:35",
                                    "crosstalk",
                                    ::tracing::Level::ERROR,
                                    ::core::option::Option::Some("src/main.rs"),
                                    ::core::option::Option::Some(35u32),
                                    ::core::option::Option::Some("crosstalk"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["err"],
                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let enabled = ::tracing::Level::ERROR
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::ERROR
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = __CALLSITE.interest();
                                !interest.is_never()
                                    && ::tracing::__macro_support::__is_enabled(
                                        __CALLSITE.metadata(),
                                        interest,
                                    )
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = __CALLSITE.metadata().fields().iter();
                                __CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &::core::iter::Iterator::next(&mut iter)
                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                ::core::option::Option::Some(&debug(&err) as &dyn Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    Err(Box::new(err))
                }
            }
        }
    }
    fn subscriber<D: Clone + Send + 'static>(
        &mut self,
        topic: TopicZoo,
    ) -> Result<crosstalk::Subscriber<D, TopicZoo>, Box<dyn std::error::Error>> {
        match topic {
            TopicZoo::Topic1 => {
                let err = crosstalk::Error::SubscriberMismatch(
                    std::any::type_name::<D>().into(),
                    std::any::type_name::<Vec<u32>>().into(),
                );
                if std::any::TypeId::of::<D>() == std::any::TypeId::of::<Vec<u32>>() {
                    let frec = match self.receivers.contains_key(&topic) {
                        true => {
                            let frec_ = crosstalk::downcast::<
                                crosstalk::flume::Receiver<Vec<u32>>,
                            >(self.receivers.remove(&topic).unwrap())
                                .unwrap();
                            let frec = frec_.clone();
                            self.receivers.insert(topic, Box::new(frec_));
                            frec
                        }
                        false => {
                            let (sender, receiver) = crosstalk::flume::unbounded::<
                                Vec<u32>,
                            >();
                            self.senders.insert(topic, Box::new(sender));
                            self.receivers.insert(topic, Box::new(receiver.clone()));
                            receiver
                        }
                    };
                    let mut update_threads = false;
                    let (ndist, did, crec) = match self.distributors.contains_key(&topic)
                    {
                        true => {
                            let (sender, receiver) = crosstalk::flume::unbounded::<
                                Vec<u32>,
                            >();
                            let dists = self.distributors.get_mut(&topic).unwrap();
                            let did = self.uniq_dist_id_incr.get_mut(&topic).unwrap();
                            *did += 1;
                            dists.insert(*did, Box::new(sender));
                            let ndist = dists.len();
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event src/main.rs:35",
                                            "crosstalk",
                                            ::tracing::Level::DEBUG,
                                            ::core::option::Option::Some("src/main.rs"),
                                            ::core::option::Option::Some(35u32),
                                            ::core::option::Option::Some("crosstalk"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::DEBUG
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::DEBUG
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &format_args!(
                                                                "Updating num distributors from {0} to {1}", self
                                                                .num_dist_per_topic.get(& topic).unwrap()
                                                                .load(std::sync::atomic::Ordering::SeqCst), ndist.clone()
                                                            ) as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            self.num_dist_per_topic
                                .get_mut(&topic)
                                .unwrap()
                                .store(ndist.clone(), std::sync::atomic::Ordering::SeqCst);
                            update_threads = ndist > 1;
                            (ndist, did.clone(), receiver)
                        }
                        false => {
                            let (sender, receiver) = crosstalk::flume::unbounded::<
                                Vec<u32>,
                            >();
                            let did = 0;
                            self.uniq_dist_id_incr.insert(topic, did.clone());
                            self.distributors
                                .insert(topic, std::collections::HashMap::new());
                            let dists = self.distributors.get_mut(&topic).unwrap();
                            dists.insert(did.clone(), Box::new(sender));
                            let ndist = dists.len();
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event src/main.rs:35",
                                            "crosstalk",
                                            ::tracing::Level::DEBUG,
                                            ::core::option::Option::Some("src/main.rs"),
                                            ::core::option::Option::Some(35u32),
                                            ::core::option::Option::Some("crosstalk"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::DEBUG
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::DEBUG
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &format_args!(
                                                                "Updating num distributors from <nothing> to {0}", ndist
                                                                .clone()
                                                            ) as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            self.num_dist_per_topic
                                .insert(
                                    topic,
                                    std::sync::Arc::new(
                                        std::sync::atomic::AtomicUsize::new(ndist.clone()),
                                    ),
                                );
                            (ndist, did.clone(), receiver)
                        }
                    };
                    if update_threads {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/main.rs:35",
                                        "crosstalk",
                                        ::tracing::Level::DEBUG,
                                        ::core::option::Option::Some("src/main.rs"),
                                        ::core::option::Option::Some(35u32),
                                        ::core::option::Option::Some("crosstalk"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::DEBUG
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::DEBUG
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Telling distribution threads to kill thread with ndist={0}",
                                                            ndist - 1
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        self.update_distribution_threads::<D>(&topic, Some(ndist - 1))
                    }
                    match crosstalk::downcast::<
                        crosstalk::Receiver<D>,
                    >(
                        Box::new(
                            crosstalk::Receiver::new(
                                frec,
                                self.num_dist_per_topic.get(&topic).unwrap().clone(),
                                crec,
                            ),
                        ),
                    ) {
                        Ok(rec) => Ok(crosstalk::Subscriber::new(did, rec, topic)),
                        Err(_e) => {
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event src/main.rs:35",
                                            "crosstalk",
                                            ::tracing::Level::ERROR,
                                            ::core::option::Option::Some("src/main.rs"),
                                            ::core::option::Option::Some(35u32),
                                            ::core::option::Option::Some("crosstalk"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["err"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::ERROR
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::ERROR
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(&debug(&err) as &dyn Value),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            Err(Box::new(err))
                        }
                    }
                } else {
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event src/main.rs:35",
                                    "crosstalk",
                                    ::tracing::Level::ERROR,
                                    ::core::option::Option::Some("src/main.rs"),
                                    ::core::option::Option::Some(35u32),
                                    ::core::option::Option::Some("crosstalk"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["err"],
                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let enabled = ::tracing::Level::ERROR
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::ERROR
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = __CALLSITE.interest();
                                !interest.is_never()
                                    && ::tracing::__macro_support::__is_enabled(
                                        __CALLSITE.metadata(),
                                        interest,
                                    )
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = __CALLSITE.metadata().fields().iter();
                                __CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &::core::iter::Iterator::next(&mut iter)
                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                ::core::option::Option::Some(&debug(&err) as &dyn Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    Err(Box::new(err))
                }
            }
            TopicZoo::Topic2 => {
                let err = crosstalk::Error::SubscriberMismatch(
                    std::any::type_name::<D>().into(),
                    std::any::type_name::<String>().into(),
                );
                if std::any::TypeId::of::<D>() == std::any::TypeId::of::<String>() {
                    let frec = match self.receivers.contains_key(&topic) {
                        true => {
                            let frec_ = crosstalk::downcast::<
                                crosstalk::flume::Receiver<String>,
                            >(self.receivers.remove(&topic).unwrap())
                                .unwrap();
                            let frec = frec_.clone();
                            self.receivers.insert(topic, Box::new(frec_));
                            frec
                        }
                        false => {
                            let (sender, receiver) = crosstalk::flume::unbounded::<
                                String,
                            >();
                            self.senders.insert(topic, Box::new(sender));
                            self.receivers.insert(topic, Box::new(receiver.clone()));
                            receiver
                        }
                    };
                    let mut update_threads = false;
                    let (ndist, did, crec) = match self.distributors.contains_key(&topic)
                    {
                        true => {
                            let (sender, receiver) = crosstalk::flume::unbounded::<
                                String,
                            >();
                            let dists = self.distributors.get_mut(&topic).unwrap();
                            let did = self.uniq_dist_id_incr.get_mut(&topic).unwrap();
                            *did += 1;
                            dists.insert(*did, Box::new(sender));
                            let ndist = dists.len();
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event src/main.rs:35",
                                            "crosstalk",
                                            ::tracing::Level::DEBUG,
                                            ::core::option::Option::Some("src/main.rs"),
                                            ::core::option::Option::Some(35u32),
                                            ::core::option::Option::Some("crosstalk"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::DEBUG
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::DEBUG
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &format_args!(
                                                                "Updating num distributors from {0} to {1}", self
                                                                .num_dist_per_topic.get(& topic).unwrap()
                                                                .load(std::sync::atomic::Ordering::SeqCst), ndist.clone()
                                                            ) as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            self.num_dist_per_topic
                                .get_mut(&topic)
                                .unwrap()
                                .store(ndist.clone(), std::sync::atomic::Ordering::SeqCst);
                            update_threads = ndist > 1;
                            (ndist, did.clone(), receiver)
                        }
                        false => {
                            let (sender, receiver) = crosstalk::flume::unbounded::<
                                String,
                            >();
                            let did = 0;
                            self.uniq_dist_id_incr.insert(topic, did.clone());
                            self.distributors
                                .insert(topic, std::collections::HashMap::new());
                            let dists = self.distributors.get_mut(&topic).unwrap();
                            dists.insert(did.clone(), Box::new(sender));
                            let ndist = dists.len();
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event src/main.rs:35",
                                            "crosstalk",
                                            ::tracing::Level::DEBUG,
                                            ::core::option::Option::Some("src/main.rs"),
                                            ::core::option::Option::Some(35u32),
                                            ::core::option::Option::Some("crosstalk"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::DEBUG
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::DEBUG
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &format_args!(
                                                                "Updating num distributors from <nothing> to {0}", ndist
                                                                .clone()
                                                            ) as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            self.num_dist_per_topic
                                .insert(
                                    topic,
                                    std::sync::Arc::new(
                                        std::sync::atomic::AtomicUsize::new(ndist.clone()),
                                    ),
                                );
                            (ndist, did.clone(), receiver)
                        }
                    };
                    if update_threads {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/main.rs:35",
                                        "crosstalk",
                                        ::tracing::Level::DEBUG,
                                        ::core::option::Option::Some("src/main.rs"),
                                        ::core::option::Option::Some(35u32),
                                        ::core::option::Option::Some("crosstalk"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::DEBUG
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::DEBUG
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Telling distribution threads to kill thread with ndist={0}",
                                                            ndist - 1
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        self.update_distribution_threads::<D>(&topic, Some(ndist - 1))
                    }
                    match crosstalk::downcast::<
                        crosstalk::Receiver<D>,
                    >(
                        Box::new(
                            crosstalk::Receiver::new(
                                frec,
                                self.num_dist_per_topic.get(&topic).unwrap().clone(),
                                crec,
                            ),
                        ),
                    ) {
                        Ok(rec) => Ok(crosstalk::Subscriber::new(did, rec, topic)),
                        Err(_e) => {
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event src/main.rs:35",
                                            "crosstalk",
                                            ::tracing::Level::ERROR,
                                            ::core::option::Option::Some("src/main.rs"),
                                            ::core::option::Option::Some(35u32),
                                            ::core::option::Option::Some("crosstalk"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["err"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::ERROR
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::ERROR
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(&debug(&err) as &dyn Value),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            Err(Box::new(err))
                        }
                    }
                } else {
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event src/main.rs:35",
                                    "crosstalk",
                                    ::tracing::Level::ERROR,
                                    ::core::option::Option::Some("src/main.rs"),
                                    ::core::option::Option::Some(35u32),
                                    ::core::option::Option::Some("crosstalk"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["err"],
                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let enabled = ::tracing::Level::ERROR
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::ERROR
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = __CALLSITE.interest();
                                !interest.is_never()
                                    && ::tracing::__macro_support::__is_enabled(
                                        __CALLSITE.metadata(),
                                        interest,
                                    )
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = __CALLSITE.metadata().fields().iter();
                                __CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &::core::iter::Iterator::next(&mut iter)
                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                ::core::option::Option::Some(&debug(&err) as &dyn Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    Err(Box::new(err))
                }
            }
            TopicZoo::Topic3 => {
                let err = crosstalk::Error::SubscriberMismatch(
                    std::any::type_name::<D>().into(),
                    std::any::type_name::<Vehicle>().into(),
                );
                if std::any::TypeId::of::<D>() == std::any::TypeId::of::<Vehicle>() {
                    let frec = match self.receivers.contains_key(&topic) {
                        true => {
                            let frec_ = crosstalk::downcast::<
                                crosstalk::flume::Receiver<Vehicle>,
                            >(self.receivers.remove(&topic).unwrap())
                                .unwrap();
                            let frec = frec_.clone();
                            self.receivers.insert(topic, Box::new(frec_));
                            frec
                        }
                        false => {
                            let (sender, receiver) = crosstalk::flume::unbounded::<
                                Vehicle,
                            >();
                            self.senders.insert(topic, Box::new(sender));
                            self.receivers.insert(topic, Box::new(receiver.clone()));
                            receiver
                        }
                    };
                    let mut update_threads = false;
                    let (ndist, did, crec) = match self.distributors.contains_key(&topic)
                    {
                        true => {
                            let (sender, receiver) = crosstalk::flume::unbounded::<
                                Vehicle,
                            >();
                            let dists = self.distributors.get_mut(&topic).unwrap();
                            let did = self.uniq_dist_id_incr.get_mut(&topic).unwrap();
                            *did += 1;
                            dists.insert(*did, Box::new(sender));
                            let ndist = dists.len();
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event src/main.rs:35",
                                            "crosstalk",
                                            ::tracing::Level::DEBUG,
                                            ::core::option::Option::Some("src/main.rs"),
                                            ::core::option::Option::Some(35u32),
                                            ::core::option::Option::Some("crosstalk"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::DEBUG
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::DEBUG
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &format_args!(
                                                                "Updating num distributors from {0} to {1}", self
                                                                .num_dist_per_topic.get(& topic).unwrap()
                                                                .load(std::sync::atomic::Ordering::SeqCst), ndist.clone()
                                                            ) as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            self.num_dist_per_topic
                                .get_mut(&topic)
                                .unwrap()
                                .store(ndist.clone(), std::sync::atomic::Ordering::SeqCst);
                            update_threads = ndist > 1;
                            (ndist, did.clone(), receiver)
                        }
                        false => {
                            let (sender, receiver) = crosstalk::flume::unbounded::<
                                Vehicle,
                            >();
                            let did = 0;
                            self.uniq_dist_id_incr.insert(topic, did.clone());
                            self.distributors
                                .insert(topic, std::collections::HashMap::new());
                            let dists = self.distributors.get_mut(&topic).unwrap();
                            dists.insert(did.clone(), Box::new(sender));
                            let ndist = dists.len();
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event src/main.rs:35",
                                            "crosstalk",
                                            ::tracing::Level::DEBUG,
                                            ::core::option::Option::Some("src/main.rs"),
                                            ::core::option::Option::Some(35u32),
                                            ::core::option::Option::Some("crosstalk"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::DEBUG
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::DEBUG
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &format_args!(
                                                                "Updating num distributors from <nothing> to {0}", ndist
                                                                .clone()
                                                            ) as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            self.num_dist_per_topic
                                .insert(
                                    topic,
                                    std::sync::Arc::new(
                                        std::sync::atomic::AtomicUsize::new(ndist.clone()),
                                    ),
                                );
                            (ndist, did.clone(), receiver)
                        }
                    };
                    if update_threads {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/main.rs:35",
                                        "crosstalk",
                                        ::tracing::Level::DEBUG,
                                        ::core::option::Option::Some("src/main.rs"),
                                        ::core::option::Option::Some(35u32),
                                        ::core::option::Option::Some("crosstalk"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::DEBUG
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::DEBUG
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Telling distribution threads to kill thread with ndist={0}",
                                                            ndist - 1
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        self.update_distribution_threads::<D>(&topic, Some(ndist - 1))
                    }
                    match crosstalk::downcast::<
                        crosstalk::Receiver<D>,
                    >(
                        Box::new(
                            crosstalk::Receiver::new(
                                frec,
                                self.num_dist_per_topic.get(&topic).unwrap().clone(),
                                crec,
                            ),
                        ),
                    ) {
                        Ok(rec) => Ok(crosstalk::Subscriber::new(did, rec, topic)),
                        Err(_e) => {
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event src/main.rs:35",
                                            "crosstalk",
                                            ::tracing::Level::ERROR,
                                            ::core::option::Option::Some("src/main.rs"),
                                            ::core::option::Option::Some(35u32),
                                            ::core::option::Option::Some("crosstalk"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["err"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::ERROR
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::ERROR
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(&debug(&err) as &dyn Value),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            Err(Box::new(err))
                        }
                    }
                } else {
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event src/main.rs:35",
                                    "crosstalk",
                                    ::tracing::Level::ERROR,
                                    ::core::option::Option::Some("src/main.rs"),
                                    ::core::option::Option::Some(35u32),
                                    ::core::option::Option::Some("crosstalk"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["err"],
                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let enabled = ::tracing::Level::ERROR
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::ERROR
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = __CALLSITE.interest();
                                !interest.is_never()
                                    && ::tracing::__macro_support::__is_enabled(
                                        __CALLSITE.metadata(),
                                        interest,
                                    )
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = __CALLSITE.metadata().fields().iter();
                                __CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &::core::iter::Iterator::next(&mut iter)
                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                ::core::option::Option::Some(&debug(&err) as &dyn Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    Err(Box::new(err))
                }
            }
            TopicZoo::Topic4 => {
                let err = crosstalk::Error::SubscriberMismatch(
                    std::any::type_name::<D>().into(),
                    std::any::type_name::<HashMap<&str, Vec<Vehicle>>>().into(),
                );
                if std::any::TypeId::of::<D>()
                    == std::any::TypeId::of::<HashMap<&str, Vec<Vehicle>>>()
                {
                    let frec = match self.receivers.contains_key(&topic) {
                        true => {
                            let frec_ = crosstalk::downcast::<
                                crosstalk::flume::Receiver<HashMap<&str, Vec<Vehicle>>>,
                            >(self.receivers.remove(&topic).unwrap())
                                .unwrap();
                            let frec = frec_.clone();
                            self.receivers.insert(topic, Box::new(frec_));
                            frec
                        }
                        false => {
                            let (sender, receiver) = crosstalk::flume::unbounded::<
                                HashMap<&str, Vec<Vehicle>>,
                            >();
                            self.senders.insert(topic, Box::new(sender));
                            self.receivers.insert(topic, Box::new(receiver.clone()));
                            receiver
                        }
                    };
                    let mut update_threads = false;
                    let (ndist, did, crec) = match self.distributors.contains_key(&topic)
                    {
                        true => {
                            let (sender, receiver) = crosstalk::flume::unbounded::<
                                HashMap<&str, Vec<Vehicle>>,
                            >();
                            let dists = self.distributors.get_mut(&topic).unwrap();
                            let did = self.uniq_dist_id_incr.get_mut(&topic).unwrap();
                            *did += 1;
                            dists.insert(*did, Box::new(sender));
                            let ndist = dists.len();
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event src/main.rs:35",
                                            "crosstalk",
                                            ::tracing::Level::DEBUG,
                                            ::core::option::Option::Some("src/main.rs"),
                                            ::core::option::Option::Some(35u32),
                                            ::core::option::Option::Some("crosstalk"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::DEBUG
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::DEBUG
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &format_args!(
                                                                "Updating num distributors from {0} to {1}", self
                                                                .num_dist_per_topic.get(& topic).unwrap()
                                                                .load(std::sync::atomic::Ordering::SeqCst), ndist.clone()
                                                            ) as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            self.num_dist_per_topic
                                .get_mut(&topic)
                                .unwrap()
                                .store(ndist.clone(), std::sync::atomic::Ordering::SeqCst);
                            update_threads = ndist > 1;
                            (ndist, did.clone(), receiver)
                        }
                        false => {
                            let (sender, receiver) = crosstalk::flume::unbounded::<
                                HashMap<&str, Vec<Vehicle>>,
                            >();
                            let did = 0;
                            self.uniq_dist_id_incr.insert(topic, did.clone());
                            self.distributors
                                .insert(topic, std::collections::HashMap::new());
                            let dists = self.distributors.get_mut(&topic).unwrap();
                            dists.insert(did.clone(), Box::new(sender));
                            let ndist = dists.len();
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event src/main.rs:35",
                                            "crosstalk",
                                            ::tracing::Level::DEBUG,
                                            ::core::option::Option::Some("src/main.rs"),
                                            ::core::option::Option::Some(35u32),
                                            ::core::option::Option::Some("crosstalk"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::DEBUG
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::DEBUG
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &format_args!(
                                                                "Updating num distributors from <nothing> to {0}", ndist
                                                                .clone()
                                                            ) as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            self.num_dist_per_topic
                                .insert(
                                    topic,
                                    std::sync::Arc::new(
                                        std::sync::atomic::AtomicUsize::new(ndist.clone()),
                                    ),
                                );
                            (ndist, did.clone(), receiver)
                        }
                    };
                    if update_threads {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/main.rs:35",
                                        "crosstalk",
                                        ::tracing::Level::DEBUG,
                                        ::core::option::Option::Some("src/main.rs"),
                                        ::core::option::Option::Some(35u32),
                                        ::core::option::Option::Some("crosstalk"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::DEBUG
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::DEBUG
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Telling distribution threads to kill thread with ndist={0}",
                                                            ndist - 1
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        self.update_distribution_threads::<D>(&topic, Some(ndist - 1))
                    }
                    match crosstalk::downcast::<
                        crosstalk::Receiver<D>,
                    >(
                        Box::new(
                            crosstalk::Receiver::new(
                                frec,
                                self.num_dist_per_topic.get(&topic).unwrap().clone(),
                                crec,
                            ),
                        ),
                    ) {
                        Ok(rec) => Ok(crosstalk::Subscriber::new(did, rec, topic)),
                        Err(_e) => {
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event src/main.rs:35",
                                            "crosstalk",
                                            ::tracing::Level::ERROR,
                                            ::core::option::Option::Some("src/main.rs"),
                                            ::core::option::Option::Some(35u32),
                                            ::core::option::Option::Some("crosstalk"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["err"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::ERROR
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::ERROR
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(&debug(&err) as &dyn Value),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            Err(Box::new(err))
                        }
                    }
                } else {
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event src/main.rs:35",
                                    "crosstalk",
                                    ::tracing::Level::ERROR,
                                    ::core::option::Option::Some("src/main.rs"),
                                    ::core::option::Option::Some(35u32),
                                    ::core::option::Option::Some("crosstalk"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["err"],
                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let enabled = ::tracing::Level::ERROR
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::ERROR
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = __CALLSITE.interest();
                                !interest.is_never()
                                    && ::tracing::__macro_support::__is_enabled(
                                        __CALLSITE.metadata(),
                                        interest,
                                    )
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = __CALLSITE.metadata().fields().iter();
                                __CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &::core::iter::Iterator::next(&mut iter)
                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                ::core::option::Option::Some(&debug(&err) as &dyn Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    Err(Box::new(err))
                }
            }
            TopicZoo::Topic5 => {
                let err = crosstalk::Error::SubscriberMismatch(
                    std::any::type_name::<D>().into(),
                    std::any::type_name::<Color>().into(),
                );
                if std::any::TypeId::of::<D>() == std::any::TypeId::of::<Color>() {
                    let frec = match self.receivers.contains_key(&topic) {
                        true => {
                            let frec_ = crosstalk::downcast::<
                                crosstalk::flume::Receiver<Color>,
                            >(self.receivers.remove(&topic).unwrap())
                                .unwrap();
                            let frec = frec_.clone();
                            self.receivers.insert(topic, Box::new(frec_));
                            frec
                        }
                        false => {
                            let (sender, receiver) = crosstalk::flume::unbounded::<
                                Color,
                            >();
                            self.senders.insert(topic, Box::new(sender));
                            self.receivers.insert(topic, Box::new(receiver.clone()));
                            receiver
                        }
                    };
                    let mut update_threads = false;
                    let (ndist, did, crec) = match self.distributors.contains_key(&topic)
                    {
                        true => {
                            let (sender, receiver) = crosstalk::flume::unbounded::<
                                Color,
                            >();
                            let dists = self.distributors.get_mut(&topic).unwrap();
                            let did = self.uniq_dist_id_incr.get_mut(&topic).unwrap();
                            *did += 1;
                            dists.insert(*did, Box::new(sender));
                            let ndist = dists.len();
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event src/main.rs:35",
                                            "crosstalk",
                                            ::tracing::Level::DEBUG,
                                            ::core::option::Option::Some("src/main.rs"),
                                            ::core::option::Option::Some(35u32),
                                            ::core::option::Option::Some("crosstalk"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::DEBUG
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::DEBUG
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &format_args!(
                                                                "Updating num distributors from {0} to {1}", self
                                                                .num_dist_per_topic.get(& topic).unwrap()
                                                                .load(std::sync::atomic::Ordering::SeqCst), ndist.clone()
                                                            ) as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            self.num_dist_per_topic
                                .get_mut(&topic)
                                .unwrap()
                                .store(ndist.clone(), std::sync::atomic::Ordering::SeqCst);
                            update_threads = ndist > 1;
                            (ndist, did.clone(), receiver)
                        }
                        false => {
                            let (sender, receiver) = crosstalk::flume::unbounded::<
                                Color,
                            >();
                            let did = 0;
                            self.uniq_dist_id_incr.insert(topic, did.clone());
                            self.distributors
                                .insert(topic, std::collections::HashMap::new());
                            let dists = self.distributors.get_mut(&topic).unwrap();
                            dists.insert(did.clone(), Box::new(sender));
                            let ndist = dists.len();
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event src/main.rs:35",
                                            "crosstalk",
                                            ::tracing::Level::DEBUG,
                                            ::core::option::Option::Some("src/main.rs"),
                                            ::core::option::Option::Some(35u32),
                                            ::core::option::Option::Some("crosstalk"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::DEBUG
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::DEBUG
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &format_args!(
                                                                "Updating num distributors from <nothing> to {0}", ndist
                                                                .clone()
                                                            ) as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            self.num_dist_per_topic
                                .insert(
                                    topic,
                                    std::sync::Arc::new(
                                        std::sync::atomic::AtomicUsize::new(ndist.clone()),
                                    ),
                                );
                            (ndist, did.clone(), receiver)
                        }
                    };
                    if update_threads {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/main.rs:35",
                                        "crosstalk",
                                        ::tracing::Level::DEBUG,
                                        ::core::option::Option::Some("src/main.rs"),
                                        ::core::option::Option::Some(35u32),
                                        ::core::option::Option::Some("crosstalk"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::DEBUG
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::DEBUG
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Telling distribution threads to kill thread with ndist={0}",
                                                            ndist - 1
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        self.update_distribution_threads::<D>(&topic, Some(ndist - 1))
                    }
                    match crosstalk::downcast::<
                        crosstalk::Receiver<D>,
                    >(
                        Box::new(
                            crosstalk::Receiver::new(
                                frec,
                                self.num_dist_per_topic.get(&topic).unwrap().clone(),
                                crec,
                            ),
                        ),
                    ) {
                        Ok(rec) => Ok(crosstalk::Subscriber::new(did, rec, topic)),
                        Err(_e) => {
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event src/main.rs:35",
                                            "crosstalk",
                                            ::tracing::Level::ERROR,
                                            ::core::option::Option::Some("src/main.rs"),
                                            ::core::option::Option::Some(35u32),
                                            ::core::option::Option::Some("crosstalk"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["err"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::ERROR
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::ERROR
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(&debug(&err) as &dyn Value),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            Err(Box::new(err))
                        }
                    }
                } else {
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event src/main.rs:35",
                                    "crosstalk",
                                    ::tracing::Level::ERROR,
                                    ::core::option::Option::Some("src/main.rs"),
                                    ::core::option::Option::Some(35u32),
                                    ::core::option::Option::Some("crosstalk"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["err"],
                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let enabled = ::tracing::Level::ERROR
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::ERROR
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = __CALLSITE.interest();
                                !interest.is_never()
                                    && ::tracing::__macro_support::__is_enabled(
                                        __CALLSITE.metadata(),
                                        interest,
                                    )
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = __CALLSITE.metadata().fields().iter();
                                __CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &::core::iter::Iterator::next(&mut iter)
                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                ::core::option::Option::Some(&debug(&err) as &dyn Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    Err(Box::new(err))
                }
            }
            _ => {
                let err = crosstalk::Error::SubscriberMismatch(
                    std::any::type_name::<D>().into(),
                    std::any::type_name::<String>().into(),
                );
                if std::any::TypeId::of::<D>() == std::any::TypeId::of::<String>() {
                    let frec = match self.receivers.contains_key(&topic) {
                        true => {
                            let frec_ = crosstalk::downcast::<
                                crosstalk::flume::Receiver<String>,
                            >(self.receivers.remove(&topic).unwrap())
                                .unwrap();
                            let frec = frec_.clone();
                            self.receivers.insert(topic, Box::new(frec_));
                            frec
                        }
                        false => {
                            let (sender, receiver) = crosstalk::flume::unbounded::<
                                String,
                            >();
                            self.senders.insert(topic, Box::new(sender));
                            self.receivers.insert(topic, Box::new(receiver.clone()));
                            receiver
                        }
                    };
                    let mut update_threads = false;
                    let (ndist, did, crec) = match self.distributors.contains_key(&topic)
                    {
                        true => {
                            let (sender, receiver) = crosstalk::flume::unbounded::<
                                String,
                            >();
                            let dists = self.distributors.get_mut(&topic).unwrap();
                            let did = self.uniq_dist_id_incr.get_mut(&topic).unwrap();
                            *did += 1;
                            dists.insert(*did, Box::new(sender));
                            let ndist = dists.len();
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event src/main.rs:35",
                                            "crosstalk",
                                            ::tracing::Level::DEBUG,
                                            ::core::option::Option::Some("src/main.rs"),
                                            ::core::option::Option::Some(35u32),
                                            ::core::option::Option::Some("crosstalk"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::DEBUG
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::DEBUG
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &format_args!(
                                                                "Updating num distributors from {0} to {1}", self
                                                                .num_dist_per_topic.get(& topic).unwrap()
                                                                .load(std::sync::atomic::Ordering::SeqCst), ndist.clone()
                                                            ) as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            self.num_dist_per_topic
                                .get_mut(&topic)
                                .unwrap()
                                .store(ndist.clone(), std::sync::atomic::Ordering::SeqCst);
                            update_threads = ndist > 1;
                            (ndist, did.clone(), receiver)
                        }
                        false => {
                            let (sender, receiver) = crosstalk::flume::unbounded::<
                                String,
                            >();
                            let did = 0;
                            self.uniq_dist_id_incr.insert(topic, did.clone());
                            self.distributors
                                .insert(topic, std::collections::HashMap::new());
                            let dists = self.distributors.get_mut(&topic).unwrap();
                            dists.insert(did.clone(), Box::new(sender));
                            let ndist = dists.len();
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event src/main.rs:35",
                                            "crosstalk",
                                            ::tracing::Level::DEBUG,
                                            ::core::option::Option::Some("src/main.rs"),
                                            ::core::option::Option::Some(35u32),
                                            ::core::option::Option::Some("crosstalk"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::DEBUG
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::DEBUG
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(
                                                            &format_args!(
                                                                "Updating num distributors from <nothing> to {0}", ndist
                                                                .clone()
                                                            ) as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            self.num_dist_per_topic
                                .insert(
                                    topic,
                                    std::sync::Arc::new(
                                        std::sync::atomic::AtomicUsize::new(ndist.clone()),
                                    ),
                                );
                            (ndist, did.clone(), receiver)
                        }
                    };
                    if update_threads {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event src/main.rs:35",
                                        "crosstalk",
                                        ::tracing::Level::DEBUG,
                                        ::core::option::Option::Some("src/main.rs"),
                                        ::core::option::Option::Some(35u32),
                                        ::core::option::Option::Some("crosstalk"),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::DEBUG
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::DEBUG
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::core::iter::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::core::option::Option::Some(
                                                        &format_args!(
                                                            "Telling distribution threads to kill thread with ndist={0}",
                                                            ndist - 1
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        self.update_distribution_threads::<D>(&topic, Some(ndist - 1))
                    }
                    match crosstalk::downcast::<
                        crosstalk::Receiver<D>,
                    >(
                        Box::new(
                            crosstalk::Receiver::new(
                                frec,
                                self.num_dist_per_topic.get(&topic).unwrap().clone(),
                                crec,
                            ),
                        ),
                    ) {
                        Ok(rec) => Ok(crosstalk::Subscriber::new(did, rec, topic)),
                        Err(_e) => {
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event src/main.rs:35",
                                            "crosstalk",
                                            ::tracing::Level::ERROR,
                                            ::core::option::Option::Some("src/main.rs"),
                                            ::core::option::Option::Some(35u32),
                                            ::core::option::Option::Some("crosstalk"),
                                            ::tracing_core::field::FieldSet::new(
                                                &["err"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::ERROR
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::ERROR
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::core::iter::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::core::option::Option::Some(&debug(&err) as &dyn Value),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            Err(Box::new(err))
                        }
                    }
                } else {
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event src/main.rs:35",
                                    "crosstalk",
                                    ::tracing::Level::ERROR,
                                    ::core::option::Option::Some("src/main.rs"),
                                    ::core::option::Option::Some(35u32),
                                    ::core::option::Option::Some("crosstalk"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["err"],
                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let enabled = ::tracing::Level::ERROR
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::ERROR
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = __CALLSITE.interest();
                                !interest.is_never()
                                    && ::tracing::__macro_support::__is_enabled(
                                        __CALLSITE.metadata(),
                                        interest,
                                    )
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = __CALLSITE.metadata().fields().iter();
                                __CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &::core::iter::Iterator::next(&mut iter)
                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                ::core::option::Option::Some(&debug(&err) as &dyn Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    Err(Box::new(err))
                }
            }
        }
    }
    fn pubsub<D: Clone + Send + 'static>(
        &mut self,
        topic: TopicZoo,
    ) -> Result<
        (crosstalk::Publisher<D, TopicZoo>, crosstalk::Subscriber<D, TopicZoo>),
        Box<dyn std::error::Error>,
    > {
        match self.publisher(topic) {
            Ok(publisher) => {
                match self.subscriber(topic) {
                    Ok(subscriber) => Ok((publisher, subscriber)),
                    Err(err) => Err(err),
                }
            }
            Err(err) => Err(err),
        }
    }
    fn delete_publisher<D: 'static>(
        &mut self,
        _publisher: crosstalk::Publisher<D, TopicZoo>,
    ) {}
    fn delete_subscriber<D: Clone + Send + 'static>(
        &mut self,
        subscriber: crosstalk::Subscriber<D, TopicZoo>,
    ) {
        let topic = subscriber.topic.clone();
        if let Some(dists) = self.distributors.get_mut(&topic) {
            dists.remove(&subscriber.id);
        }
        if let Some(ndist) = self.num_dist_per_topic.get(&topic) {
            let n = ndist
                .fetch_update(
                    std::sync::atomic::Ordering::SeqCst,
                    std::sync::atomic::Ordering::SeqCst,
                    |x| Some(x - 1),
                );
            match n {
                Ok(n) => {
                    match n {
                        0..=2 => {
                            let _ = self.restart_forwarding(&topic, Some(n));
                        }
                        _ => self.update_distribution_threads::<D>(&topic, Some(n)),
                    }
                }
                Err(_) => {}
            }
        }
    }
}
fn main() {
    let mut node = crosstalk::UnboundedCommonNode::<TopicZoo>::new();
    let (pub0_topic4, sub0_topic4) = node.pubsub(TopicZoo::Topic4).unwrap();
    let pub1_topic4 = pub0_topic4.clone();
    let sub1_topic4 = node.subscriber(TopicZoo::Topic4).unwrap();
    let pub0_topic5 = node.publisher::<Color>(TopicZoo::Topic5).unwrap();
    pub0_topic5.write(Color::Red);
    let message = HashMap::from([
        (
            "my vehicles",
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    Vehicle {
                        make: "Nissan".to_string(),
                        model: "Rogue".to_string(),
                        color: Color::Blue,
                        wheels: 4,
                    },
                    Vehicle {
                        make: "Toyota".to_string(),
                        model: "Prius".to_string(),
                        color: Color::Green,
                        wheels: 4,
                    },
                ]),
            ),
        ),
    ]);
    thread::spawn(move || {
        pub0_topic4.write(message);
    });
    let received_0 = sub0_topic4.read();
    let received_1 = sub1_topic4.read();
    {
        ::std::io::_print(format_args!("{0:?}\n", received_0));
    };
    {
        ::std::io::_print(format_args!("{0:?}\n", received_1));
    };
    match (&received_0, &received_1) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    node.delete_subscriber(sub1_topic4);
    let message2 = HashMap::from([
        (
            "my vehicles 2",
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    Vehicle {
                        make: "Honda".to_string(),
                        model: "Civic".to_string(),
                        color: Color::Red,
                        wheels: 4,
                    },
                ]),
            ),
        ),
    ]);
    pub1_topic4.write(message2);
    node.delete_publisher(pub1_topic4);
    thread::spawn(move || {
            let received_2 = sub0_topic4.read_blocking();
            {
                ::std::io::_print(format_args!("{0:?}\n", received_2));
            };
        })
        .join()
        .unwrap();
}
