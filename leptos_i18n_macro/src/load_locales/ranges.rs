use std::{
    marker::PhantomData,
    ops::{Bound, Not},
    rc::Rc,
    str::FromStr,
};

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::ParseBuffer;

use crate::load_locales::locale::LocalesOrNamespaces;
use crate::utils::key::{Key, KeyPath};

use super::{
    declare_locales::parse_range_pairs,
    error::{Error, Result},
    parsed_value::{InterpolationKeys, ParsedValue, ParsedValueSeed},
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum RangeType {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
}

impl Default for RangeType {
    fn default() -> Self {
        Self::I32
    }
}

type DefaultRangeType = i32;

impl core::fmt::Display for RangeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RangeType::I8 => f.write_str("i8"),
            RangeType::I16 => f.write_str("i16"),
            RangeType::I32 => f.write_str("i32"),
            RangeType::I64 => f.write_str("i64"),
            RangeType::U8 => f.write_str("u8"),
            RangeType::U16 => f.write_str("u16"),
            RangeType::U32 => f.write_str("u32"),
            RangeType::U64 => f.write_str("u64"),
            RangeType::F32 => f.write_str("f32"),
            RangeType::F64 => f.write_str("f64"),
        }
    }
}

impl ToTokens for RangeType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let range_type = match self {
            RangeType::I8 => quote!(i8),
            RangeType::I16 => quote!(i16),
            RangeType::I32 => quote!(i32),
            RangeType::I64 => quote!(i64),
            RangeType::U8 => quote!(u8),
            RangeType::U16 => quote!(u16),
            RangeType::U32 => quote!(u32),
            RangeType::U64 => quote!(u64),
            RangeType::F32 => quote!(f32),
            RangeType::F64 => quote!(f64),
        };
        tokens.extend(range_type)
    }
}

impl RangeType {
    #[inline]
    const fn should_have_fallback(self) -> bool {
        matches!(self, RangeType::F64 | RangeType::F32)
    }
}

pub type RangesInner<T> = Vec<(Range<T>, ParsedValue)>;

#[derive(Debug, Clone, PartialEq)]
pub enum Ranges {
    I8(RangesInner<i8>),
    I16(RangesInner<i16>),
    I32(RangesInner<i32>),
    I64(RangesInner<i64>),
    U8(RangesInner<u8>),
    U16(RangesInner<u16>),
    U32(RangesInner<u32>),
    U64(RangesInner<u64>),
    F32(RangesInner<f32>),
    F64(RangesInner<f64>),
}

impl Ranges {
    pub fn as_string_impl(&self) -> TokenStream {
        match self {
            Ranges::I8(ranges) => Self::to_tokens_integers_string(ranges),
            Ranges::I16(ranges) => Self::to_tokens_integers_string(ranges),
            Ranges::I32(ranges) => Self::to_tokens_integers_string(ranges),
            Ranges::I64(ranges) => Self::to_tokens_integers_string(ranges),
            Ranges::U8(ranges) => Self::to_tokens_integers_string(ranges),
            Ranges::U16(ranges) => Self::to_tokens_integers_string(ranges),
            Ranges::U32(ranges) => Self::to_tokens_integers_string(ranges),
            Ranges::U64(ranges) => Self::to_tokens_integers_string(ranges),
            Ranges::F32(ranges) => Self::to_tokens_floats_string(ranges),
            Ranges::F64(ranges) => Self::to_tokens_floats_string(ranges),
        }
    }

    pub fn resolve_foreign_keys(
        &self,
        values: &LocalesOrNamespaces,
        top_locale: &Rc<Key>,
        default_locale: &Rc<Key>,
        path: &KeyPath,
    ) -> Result<()> {
        self.try_for_each_value(move |value| {
            value.resolve_foreign_key(values, top_locale, default_locale, path)
        })
    }

    pub fn get_keys_inner(
        &self,
        key_path: &mut KeyPath,
        keys: &mut Option<InterpolationKeys>,
    ) -> Result<()> {
        fn inner<T>(
            v: &RangesInner<T>,
            key_path: &mut KeyPath,
            keys: &mut Option<InterpolationKeys>,
        ) -> Result<()> {
            for (_, value) in v {
                value.get_keys_inner(key_path, keys)?;
            }
            Ok(())
        }
        match self {
            Ranges::I8(v) => inner(v, key_path, keys),
            Ranges::I16(v) => inner(v, key_path, keys),
            Ranges::I32(v) => inner(v, key_path, keys),
            Ranges::I64(v) => inner(v, key_path, keys),
            Ranges::U8(v) => inner(v, key_path, keys),
            Ranges::U16(v) => inner(v, key_path, keys),
            Ranges::U32(v) => inner(v, key_path, keys),
            Ranges::U64(v) => inner(v, key_path, keys),
            Ranges::F32(v) => inner(v, key_path, keys),
            Ranges::F64(v) => inner(v, key_path, keys),
        }
    }

    pub fn try_for_each_value<F, E>(&self, f: F) -> Result<(), E>
    where
        F: FnMut(&ParsedValue) -> Result<(), E>,
    {
        fn inner<T, F, E>(v: &RangesInner<T>, mut f: F) -> Result<(), E>
        where
            F: FnMut(&ParsedValue) -> Result<(), E>,
        {
            for (_, value) in v {
                f(value)?;
            }
            Ok(())
        }
        match self {
            Ranges::I8(v) => inner(v, f),
            Ranges::I16(v) => inner(v, f),
            Ranges::I32(v) => inner(v, f),
            Ranges::I64(v) => inner(v, f),
            Ranges::U8(v) => inner(v, f),
            Ranges::U16(v) => inner(v, f),
            Ranges::U32(v) => inner(v, f),
            Ranges::U64(v) => inner(v, f),
            Ranges::F32(v) => inner(v, f),
            Ranges::F64(v) => inner(v, f),
        }
    }

    pub fn try_for_each_value_mut<F, E>(&mut self, f: F) -> Result<(), E>
    where
        F: FnMut(&mut ParsedValue) -> Result<(), E>,
    {
        fn inner<T, F, E>(v: &mut RangesInner<T>, mut f: F) -> Result<(), E>
        where
            F: FnMut(&mut ParsedValue) -> Result<(), E>,
        {
            for (_, value) in v {
                f(value)?;
            }
            Ok(())
        }
        match self {
            Ranges::I8(v) => inner(v, f),
            Ranges::I16(v) => inner(v, f),
            Ranges::I32(v) => inner(v, f),
            Ranges::I64(v) => inner(v, f),
            Ranges::U8(v) => inner(v, f),
            Ranges::U16(v) => inner(v, f),
            Ranges::U32(v) => inner(v, f),
            Ranges::U64(v) => inner(v, f),
            Ranges::F32(v) => inner(v, f),
            Ranges::F64(v) => inner(v, f),
        }
    }

    pub const fn get_type(&self) -> RangeType {
        match self {
            Ranges::I8(_) => RangeType::I8,
            Ranges::I16(_) => RangeType::I16,
            Ranges::I32(_) => RangeType::I32,
            Ranges::I64(_) => RangeType::I64,
            Ranges::U8(_) => RangeType::U8,
            Ranges::U16(_) => RangeType::U16,
            Ranges::U32(_) => RangeType::U32,
            Ranges::U64(_) => RangeType::U64,
            Ranges::F32(_) => RangeType::F32,
            Ranges::F64(_) => RangeType::F64,
        }
    }

    fn to_tokens_integers<T: RangeInteger>(ranges: &[(Range<T>, ParsedValue)]) -> TokenStream {
        let match_arms = ranges.iter().map(|(range, value)| quote!(#range => #value));

        let mut captured_values = None;
        let mut key_path = KeyPath::new(None);

        for (_, value) in ranges {
            value
                .get_keys_inner(&mut key_path, &mut captured_values)
                .unwrap();
        }

        let captured_values = captured_values.map(|keys| {
            let keys = keys
                .iter_keys()
                .map(|key| quote!(let #key = core::clone::Clone::clone(&#key);));
            quote!(#(#keys)*)
        });
        let match_statement = quote! {
            {
                let plural_count = plural_count();
                match plural_count {
                    #(
                        #match_arms,
                    )*
                }
            }
        };

        quote! {
            leptos::IntoView::into_view(
                {
                    #captured_values
                    move || #match_statement
                },

            )
        }
    }

    fn to_tokens_integers_string<T: RangeInteger>(
        ranges: &[(Range<T>, ParsedValue)],
    ) -> TokenStream {
        let match_arms = ranges.iter().map(|(range, value)| {
            let value = value.as_string_impl();
            quote!(#range => #value)
        });

        quote! {
            {
                let plural_count = *plural_count;
                match plural_count {
                    #(
                        #match_arms,
                    )*
                }
            }
        }
    }

    fn to_condition<T: RangeFloats>(range: &Range<T>) -> Option<TokenStream> {
        match range {
            Range::Exact(exact) => Some(quote!(plural_count == #exact)),
            Range::Bounds { .. } => {
                Some(quote!(core::ops::RangeBounds::contains(&(#range), &plural_count)))
            }
            Range::Multiple(conditions) => {
                let mut conditions = conditions.iter().filter_map(Self::to_condition);
                let first = conditions.next();
                Some(quote!(#first #(|| #conditions)*))
            }
            Range::Fallback => None,
        }
    }

    fn to_tokens_floats<T: RangeFloats>(ranges: &[(Range<T>, ParsedValue)]) -> TokenStream {
        let mut ifs = ranges
            .iter()
            .map(|(range, value)| match Self::to_condition(range) {
                None => quote!({ #value }),
                Some(condition) => quote!(if #condition { #value }),
            });
        let first = ifs.next();
        let ifs = quote! {
            #first
            #(else #ifs)*
        };

        let mut captured_values = None;
        let mut key_path = KeyPath::new(None);

        for (_, value) in ranges {
            value
                .get_keys_inner(&mut key_path, &mut captured_values)
                .unwrap();
        }

        let captured_values = captured_values.map(|keys| {
            let keys = keys
                .iter_keys()
                .map(|key| quote!(let #key = core::clone::Clone::clone(&#key);));
            quote!(#(#keys)*)
        });

        quote! {
            leptos::IntoView::into_view(
                {
                    #captured_values
                    move || {
                        let plural_count = plural_count();
                        #ifs
                    }
                },

            )
        }
    }

    fn to_tokens_floats_string<T: RangeFloats>(ranges: &[(Range<T>, ParsedValue)]) -> TokenStream {
        let mut ifs = ranges.iter().map(|(range, value)| {
            let value = value.as_string_impl();
            match Self::to_condition(range) {
                None => quote!({ #value }),
                Some(condition) => quote!(if #condition { #value }),
            }
        });
        let first = ifs.next();
        let ifs = quote! {
            #first
            #(else #ifs)*
        };

        quote! {
            {
                let plural_count = *plural_count;
                #ifs
            }
        }
    }

    pub fn deserialize_inner<'a, 'de, A>(&mut self, seq: A, seed: A::Seed) -> A::Result<()>
    where
        A: ParseRanges<'a, 'de>,
    {
        match self {
            Ranges::I8(ranges) => ParseRanges::deserialize_all_pairs(seq, ranges, seed),
            Ranges::I16(ranges) => ParseRanges::deserialize_all_pairs(seq, ranges, seed),
            Ranges::I32(ranges) => ParseRanges::deserialize_all_pairs(seq, ranges, seed),
            Ranges::I64(ranges) => ParseRanges::deserialize_all_pairs(seq, ranges, seed),
            Ranges::U8(ranges) => ParseRanges::deserialize_all_pairs(seq, ranges, seed),
            Ranges::U16(ranges) => ParseRanges::deserialize_all_pairs(seq, ranges, seed),
            Ranges::U32(ranges) => ParseRanges::deserialize_all_pairs(seq, ranges, seed),
            Ranges::U64(ranges) => ParseRanges::deserialize_all_pairs(seq, ranges, seed),
            Ranges::F32(ranges) => ParseRanges::deserialize_all_pairs(seq, ranges, seed),
            Ranges::F64(ranges) => ParseRanges::deserialize_all_pairs(seq, ranges, seed),
        }
    }

    pub fn from_type(range_type: RangeType) -> Self {
        match range_type {
            RangeType::I8 => Self::I8(vec![]),
            RangeType::I16 => Self::I16(vec![]),
            RangeType::I32 => Self::I32(vec![]),
            RangeType::I64 => Self::I64(vec![]),
            RangeType::U8 => Self::U8(vec![]),
            RangeType::U16 => Self::U16(vec![]),
            RangeType::U32 => Self::U32(vec![]),
            RangeType::U64 => Self::U64(vec![]),
            RangeType::F32 => Self::F32(vec![]),
            RangeType::F64 => Self::F64(vec![]),
        }
    }

    pub fn from_serde_seq<'de, A>(
        mut seq: A,
        parsed_value_seed: ParsedValueSeed,
    ) -> Result<Self, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let type_or_range = seq
            .next_element_seed(TypeOrRangeSeed(parsed_value_seed))?
            .ok_or_else(|| Error::EmptyRange)
            .map_err(serde::de::Error::custom)?;

        let mut ranges = match type_or_range {
            TypeOrRange::Type(range_type) => Self::from_type(range_type),
            TypeOrRange::Range(range) => Ranges::I32(vec![range]),
        };

        ranges.deserialize_inner(seq, parsed_value_seed)?;
        Ok(ranges)
    }

    fn check_de_inner<T: RangeNumber>(ranges: &[(Range<T>, ParsedValue)]) -> (bool, usize, bool) {
        // easy to avoid compile warning, check if a fallback is not at the end position
        let invalid_fallback = ranges.iter().rev().skip(1).any(|(range, _)| match range {
            Range::Fallback => true,
            // "n | _" is kind of pointless but still supported, but still check if a fallback is put outside the end
            Range::Multiple(multi) => multi.iter().any(|range| matches!(range, Range::Fallback)),
            _ => false,
        });
        // also check if multiple fallbacks exist
        let fallback_count = ranges
            .iter()
            .filter(|(range, _)| matches!(range, Range::Fallback))
            .count();

        (
            invalid_fallback,
            fallback_count,
            T::TYPE.should_have_fallback(),
        )
    }

    pub fn check_deserialization(&self) -> (bool, usize, bool) {
        match self {
            Ranges::I8(ranges) => Self::check_de_inner(ranges),
            Ranges::I16(ranges) => Self::check_de_inner(ranges),
            Ranges::I32(ranges) => Self::check_de_inner(ranges),
            Ranges::I64(ranges) => Self::check_de_inner(ranges),
            Ranges::U8(ranges) => Self::check_de_inner(ranges),
            Ranges::U16(ranges) => Self::check_de_inner(ranges),
            Ranges::U32(ranges) => Self::check_de_inner(ranges),
            Ranges::U64(ranges) => Self::check_de_inner(ranges),
            Ranges::F32(ranges) => Self::check_de_inner(ranges),
            Ranges::F64(ranges) => Self::check_de_inner(ranges),
        }
    }
}

impl ToTokens for Ranges {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Ranges::I8(ranges) => Self::to_tokens_integers(ranges).to_tokens(tokens),
            Ranges::I16(ranges) => Self::to_tokens_integers(ranges).to_tokens(tokens),
            Ranges::I32(ranges) => Self::to_tokens_integers(ranges).to_tokens(tokens),
            Ranges::I64(ranges) => Self::to_tokens_integers(ranges).to_tokens(tokens),
            Ranges::U8(ranges) => Self::to_tokens_integers(ranges).to_tokens(tokens),
            Ranges::U16(ranges) => Self::to_tokens_integers(ranges).to_tokens(tokens),
            Ranges::U32(ranges) => Self::to_tokens_integers(ranges).to_tokens(tokens),
            Ranges::U64(ranges) => Self::to_tokens_integers(ranges).to_tokens(tokens),
            Ranges::F32(ranges) => Self::to_tokens_floats(ranges).to_tokens(tokens),
            Ranges::F64(ranges) => Self::to_tokens_floats(ranges).to_tokens(tokens),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Range<T> {
    Exact(T),
    Bounds { start: Option<T>, end: Bound<T> },
    Multiple(Vec<Self>),
    Fallback,
}

pub trait RangeNumber: FromStr + ToTokens + PartialOrd + Copy {
    const TYPE: RangeType;

    fn range_end_bound(self) -> Option<Bound<Self>>;

    fn from_u64(v: u64) -> Option<Self>;
    fn from_i64(v: i64) -> Option<Self>;
    fn from_f64(v: f64) -> Option<Self>;
}

pub trait RangeInteger: RangeNumber {}

pub trait RangeFloats: RangeNumber {}

impl<T: RangeNumber> Range<T> {
    fn flatten(self) -> Self {
        let Range::Multiple(ranges) = self else {
            return self;
        };
        if ranges.contains(&Range::Fallback) {
            return Range::Fallback;
        }
        Range::Multiple(ranges)
    }

    pub fn new(s: &str) -> Result<Self> {
        let parse = |s: &str| {
            s.parse::<T>().map_err(|_| Error::RangeParse {
                range: s.to_string(),
                range_type: T::TYPE,
            })
        };
        let s = s.trim();
        if matches!(s, "_" | "..") {
            return Ok(Self::Fallback);
        };

        if s.contains('|') {
            return s
                .split('|')
                .map(|s| Self::new(s))
                .collect::<Result<_>>()
                .map(Self::Multiple)
                .map(Self::flatten);
        }

        if let Some((start, end)) = s.split_once("..") {
            let start = start.trim();
            let start = start.is_empty().not().then(|| parse(start)).transpose()?;
            let end = end.trim();
            let end = if end.is_empty() {
                Bound::Unbounded
            } else if let Some(end) = end.strip_prefix('=').map(str::trim_start) {
                Bound::Included(parse(end)?)
            } else {
                let end = parse(end)?;

                end.range_end_bound()
                    .ok_or_else(|| Error::InvalidBoundEnd {
                        range: s.to_string(),
                        range_type: T::TYPE,
                    })?
            };

            if let Some(start) = start {
                match end {
                    Bound::Excluded(end) if end <= start => {
                        return Err(Error::ImpossibleRange(s.to_string()))
                    }
                    Bound::Included(end) if end < start => {
                        return Err(Error::ImpossibleRange(s.to_string()))
                    }
                    _ => {}
                }
            }

            Ok(Self::Bounds { start, end })
        } else {
            parse(s).map(Self::Exact)
        }
    }
}

impl<T: RangeNumber> ToTokens for Range<T> {
    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            Range::Exact(num) => quote!(#num),
            Range::Bounds {
                start,
                end: Bound::Included(end),
            } => {
                quote!(#start..=#end)
            }
            Range::Bounds {
                start,
                end: Bound::Unbounded,
            } => {
                quote!(#start..)
            }
            Range::Bounds {
                start,
                end: Bound::Excluded(end),
            } => {
                quote!(#start..#end)
            }
            Range::Fallback => quote!(_),
            Range::Multiple(matchs) => {
                let mut matchs = matchs.iter().map(Self::to_token_stream);
                if let Some(first) = matchs.next() {
                    quote!(#first #(| #matchs)*)
                } else {
                    quote!()
                }
            }
        }
    }

    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        Self::to_token_stream(self).to_tokens(tokens)
    }
}

#[derive(Debug, Clone, Copy)]
struct RangeSeed<T>(PhantomData<T>);

impl<'de, T: RangeNumber> serde::de::DeserializeSeed<'de> for RangeSeed<T> {
    type Value = Range<T>;
    fn deserialize<D>(self, deserializer: D) -> std::result::Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }
}

impl<'de, T: RangeNumber> serde::de::Visitor<'de> for RangeSeed<T> {
    type Value = Range<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "a string representing a range or a sequence of string representing a range"
        )
    }

    fn visit_f64<E>(self, v: f64) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        T::from_f64(v)
            .map(Range::Exact)
            .ok_or(Error::RangeNumberType {
                found: RangeType::F64,
                expected: T::TYPE,
            })
            .map_err(serde::de::Error::custom)
    }

    fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        T::from_i64(v)
            .map(Range::Exact)
            .ok_or(Error::RangeNumberType {
                found: RangeType::I64,
                expected: T::TYPE,
            })
            .map_err(serde::de::Error::custom)
    }

    fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        T::from_u64(v)
            .map(Range::Exact)
            .ok_or(Error::RangeNumberType {
                found: RangeType::U64,
                expected: T::TYPE,
            })
            .map_err(serde::de::Error::custom)
    }

    fn visit_seq<A>(self, mut seq: A) -> std::result::Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let Some(first) = seq.next_element_seed(self)? else {
            return Ok(Range::Fallback);
        };
        let mut ranges = vec![];

        while let Some(range) = seq.next_element_seed(self)? {
            ranges.push(range)
        }

        if ranges.is_empty() {
            Ok(first)
        } else {
            ranges.push(first);
            Ok(Range::Multiple(ranges))
        }
    }

    fn visit_str<E>(self, s: &str) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Range::new(s).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Copy)]
struct RangeStructSeed<'a, T>(pub ParsedValueSeed<'a>, PhantomData<T>);

impl<'de, T: RangeNumber> serde::de::DeserializeSeed<'de> for RangeStructSeed<'_, T> {
    type Value = (Range<T>, ParsedValue);
    fn deserialize<D>(self, deserializer: D) -> std::result::Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }
}

impl<'de, T: RangeNumber> serde::de::Visitor<'de> for RangeStructSeed<'_, T> {
    type Value = (Range<T>, ParsedValue);

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "either a struct representing a range with the count and the value, or a sequence with the first element being the value and the other elements being the count"
        )
    }

    fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        fn deser_field<'de, A, S, T>(
            option: &mut Option<T>,
            map: &mut A,
            seed: S,
            field_name: &'static str,
        ) -> Result<(), A::Error>
        where
            A: serde::de::MapAccess<'de>,
            S: serde::de::DeserializeSeed<'de, Value = T>,
        {
            if option.replace(map.next_value_seed(seed)?).is_some() {
                Err(serde::de::Error::duplicate_field(field_name))
            } else {
                Ok(())
            }
        }
        fn unwrap_field<T, E>(field: Option<T>, field_name: &'static str) -> Result<T, E>
        where
            E: serde::de::Error,
        {
            field.ok_or_else(|| serde::de::Error::missing_field(field_name))
        }
        let mut range = None;
        let mut value = None;
        while let Some(field) = map.next_key()? {
            match field {
                RangeField::Range => {
                    deser_field(&mut range, &mut map, RangeSeed(PhantomData), "count")?
                }
                RangeField::Value => deser_field(&mut value, &mut map, self.0, "count")?,
            }
        }

        let range = range.unwrap_or(Range::Fallback); // if no count, fallback
        let value = unwrap_field(value, "value")?;

        Ok((range, value))
    }

    fn visit_seq<A>(self, mut seq: A) -> std::result::Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let Some(value) = seq.next_element_seed(self.0)? else {
            return Err(serde::de::Error::invalid_length(0, &"at least 1 element"));
        };
        let range = RangeSeed(PhantomData).visit_seq(seq)?;

        Ok((range, value))
    }
}

enum RangeField {
    Range,
    Value,
}

impl RangeField {
    pub const FIELDS: &'static [&'static str] = &["count", "value"];
}

struct RangeFieldVisitor;

impl<'de> serde::de::Deserialize<'de> for RangeField {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_identifier(RangeFieldVisitor)
    }
}

impl<'de> serde::de::Visitor<'de> for RangeFieldVisitor {
    type Value = RangeField;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "an identifier for fields {:?}",
            RangeField::FIELDS
        )
    }

    fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "count" => Ok(RangeField::Range),
            "value" => Ok(RangeField::Value),
            _ => Err(serde::de::Error::unknown_field(v, RangeField::FIELDS)),
        }
    }
}

pub enum TypeOrRange {
    Type(RangeType),
    Range((Range<DefaultRangeType>, ParsedValue)),
}

struct TypeOrRangeSeed<'a>(pub ParsedValueSeed<'a>);

impl TypeOrRange {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim() {
            "i8" => Some(TypeOrRange::Type(RangeType::I8)),
            "i16" => Some(TypeOrRange::Type(RangeType::I16)),
            "i32" => Some(TypeOrRange::Type(RangeType::I32)),
            "i64" => Some(TypeOrRange::Type(RangeType::I64)),
            "u8" => Some(TypeOrRange::Type(RangeType::U8)),
            "u16" => Some(TypeOrRange::Type(RangeType::U16)),
            "u32" => Some(TypeOrRange::Type(RangeType::U32)),
            "u64" => Some(TypeOrRange::Type(RangeType::U64)),
            "f32" => Some(TypeOrRange::Type(RangeType::F32)),
            "f64" => Some(TypeOrRange::Type(RangeType::F64)),
            _ => None,
        }
    }
}

impl<'de> serde::de::DeserializeSeed<'de> for TypeOrRangeSeed<'_> {
    type Value = TypeOrRange;

    fn deserialize<D>(self, deserializer: D) -> std::result::Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }
}

impl<'de> serde::de::Visitor<'de> for TypeOrRangeSeed<'_> {
    type Value = TypeOrRange;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "either a string describing a numerical type or a range"
        )
    }

    fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        TypeOrRange::from_str(v)
            .ok_or_else(|| serde::de::Error::custom(Error::InvalidRangeType(v.to_string())))
    }

    fn visit_map<A>(self, map: A) -> std::result::Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let range_seed = RangeStructSeed::<DefaultRangeType>(self.0, PhantomData);
        range_seed.visit_map(map).map(TypeOrRange::Range)
    }

    fn visit_seq<A>(self, seq: A) -> std::result::Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let range_seed = RangeStructSeed::<DefaultRangeType>(self.0, PhantomData);
        range_seed.visit_seq(seq).map(TypeOrRange::Range)
    }
}

pub trait ParseRanges<'a, 'de> {
    type Result<O>
    where
        O: 'de + 'a;

    type Seed;

    fn deserialize_all_pairs<T: RangeNumber>(
        self,
        ranges: &mut RangesInner<T>,
        seed: Self::Seed,
    ) -> Self::Result<()>;
}

impl<'a, 'de, A: serde::de::SeqAccess<'de>> ParseRanges<'a, 'de> for A {
    type Result<O> = Result<O, A::Error> where O: 'de + 'a;

    type Seed = ParsedValueSeed<'a>;

    fn deserialize_all_pairs<T: RangeNumber>(
        mut self,
        ranges: &mut RangesInner<T>,
        seed: Self::Seed,
    ) -> Self::Result<()> {
        let range_seed = RangeStructSeed::<T>(seed, PhantomData);
        while let Some(pair) = self.next_element_seed(range_seed)? {
            ranges.push(pair)
        }
        Ok(())
    }
}

pub struct RangeParseBuffer<'de>(pub ParseBuffer<'de>);

impl<'a, 'de> ParseRanges<'a, 'de> for RangeParseBuffer<'de> {
    type Result<O> = syn::Result<O> where O: 'de + 'a;

    type Seed = super::declare_locales::ParseRangeSeed<'a>;

    fn deserialize_all_pairs<T: RangeNumber>(
        self,
        ranges: &mut RangesInner<T>,
        seed: Self::Seed,
    ) -> Self::Result<()> {
        parse_range_pairs(&self.0, ranges, seed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact() {
        let range = Range::new("0").unwrap();

        assert_eq!(range, Range::Exact(0));
    }

    #[test]
    fn test_fallback() {
        let range = Range::<i32>::new("_").unwrap();

        assert_eq!(range, Range::Fallback);
    }

    #[test]
    fn test_range() {
        let range = Range::new("0..6").unwrap();

        assert_eq!(
            range,
            Range::Bounds {
                start: Some(0),
                end: Bound::Included(5)
            }
        );
    }

    #[test]
    fn test_range_unbounded_end() {
        let range = Range::new("0..").unwrap();

        assert_eq!(
            range,
            Range::Bounds {
                start: Some(0),
                end: Bound::Unbounded
            }
        );
    }

    #[test]
    fn test_range_included_end() {
        let range = Range::new("0..=6").unwrap();

        assert_eq!(
            range,
            Range::Bounds {
                start: Some(0),
                end: Bound::Included(6)
            }
        );
    }

    #[test]
    fn test_range_unbounded_start() {
        let range = Range::new("..=6").unwrap();

        assert_eq!(
            range,
            Range::Bounds {
                start: None,
                end: Bound::Included(6)
            }
        );
    }

    #[test]
    fn test_range_full() {
        let range = Range::<i32>::new("..").unwrap();

        assert_eq!(range, Range::Fallback);
    }

    #[test]
    fn test_multiple() {
        let range = Range::<i32>::new("5 | 5..8 | 70..=80").unwrap();

        assert_eq!(
            range,
            Range::Multiple(vec![
                Range::Exact(5),
                Range::Bounds {
                    start: Some(5),
                    end: Bound::Included(7)
                },
                Range::Bounds {
                    start: Some(70),
                    end: Bound::Included(80)
                }
            ])
        );
    }

    #[test]
    fn test_multiple_with_fallback() {
        let range = Range::<i32>::new("5 | 5..8 | 70..=80 | _").unwrap();

        assert_eq!(range, Range::Fallback);
    }
}

mod range_number_impl {
    use super::{Bound, RangeFloats, RangeInteger, RangeNumber, RangeType};
    macro_rules! impl_num {
        ($(($num_type:ty, $range_type:ident))*) => {
            $(
                impl RangeNumber for $num_type {
                    const TYPE: RangeType = RangeType::$range_type;

                    fn range_end_bound(self) -> Option<Bound<Self>> {
                        self.checked_sub(1).map(Bound::Included)
                    }

                    fn from_i64(v: i64) -> Option<Self> {
                        <$num_type>::try_from(v).ok()
                    }

                    fn from_u64(v: u64) -> Option<Self> {
                        <$num_type>::try_from(v).ok()
                    }

                    fn from_f64(_v: f64) -> Option<Self> {
                        None
                    }
                }

                impl RangeInteger for $num_type {}
            )*
        };
    }

    macro_rules! impl_floats {
        ($(($num_type:ty, $range_type:ident))*) => {
            $(
                impl RangeNumber for $num_type {
                    const TYPE: RangeType = RangeType::$range_type;

                    fn range_end_bound(self) -> Option<Bound<Self>> {
                        Some(Bound::Excluded(self))
                    }

                    fn from_i64(v: i64) -> Option<Self> {
                        Some(v as $num_type)
                    }

                    fn from_u64(v: u64) -> Option<Self> {
                        Some(v as $num_type)
                    }

                    fn from_f64(v: f64) -> Option<Self> {
                        Some(v as $num_type)
                    }
                }

                impl RangeFloats for $num_type {}
            )*
        };
    }

    impl_num!((i8, I8)(i16, I16)(i32, I32)(i64, I64)(u8, U8)(u16, U16)(
        u32, U32
    )(u64, U64));

    impl_floats!((f32, F32)(f64, F64));
}
