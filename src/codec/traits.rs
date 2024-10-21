use super::codec::UnknownType;
use super::{decoder, encoder};
use crate::codec::Id;
use crate::Codec;

pub trait Decoder<T> {
    fn decoder(self) -> Option<Codec<T>>;
}

impl<'a> Decoder<UnknownType> for &'a str {
    fn decoder(self) -> Option<Codec<UnknownType>> {
        decoder::find_by_name(self)
    }
}

impl Decoder<UnknownType> for Id {
    fn decoder(self) -> Option<Codec<UnknownType>> {
        decoder::find(self)
    }
}

impl<T> Decoder<T> for Codec<T> {
    fn decoder(self) -> Option<Codec<T>> {
        if self.is_decoder() {
            Some(self)
        } else {
            None
        }
    }
}

impl<T> Decoder<T> for Option<Codec<T>> {
    fn decoder(self) -> Option<Codec<T>> {
        self.and_then(|c| c.decoder())
    }
}

pub trait Encoder<T> {
    fn encoder(self) -> Option<Codec<T>>;
}

impl<'a> Encoder<UnknownType> for &'a str {
    fn encoder(self) -> Option<Codec<UnknownType>> {
        encoder::find_by_name(self)
    }
}

impl Encoder<UnknownType> for Id {
    fn encoder(self) -> Option<Codec<UnknownType>> {
        encoder::find(self)
    }
}

impl<T> Encoder<T> for Codec<T> {
    fn encoder(self) -> Option<Codec<T>> {
        if self.is_encoder() {
            Some(self)
        } else {
            None
        }
    }
}

impl<T> Encoder<T> for Option<Codec<T>> {
    fn encoder(self) -> Option<Codec<T>> {
        self.and_then(|c| c.encoder())
    }
}
