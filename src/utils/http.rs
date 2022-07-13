use crate::error::Error;
use actix_web::web::{BytesMut, Payload};
use futures::{Future, Stream};

pub struct ParsedBody(pub Vec<u8>);

impl ParsedBody {
  pub fn parse(body: Payload) -> impl Future<Item = Self, Error = Error> {
    body.map_err(Error::from)
      .fold(BytesMut::new(), move |mut body, chunk| {
        body.extend_from_slice(&chunk);

        Ok::<_, Error>(body)
      }).and_then(|body| {
        Ok(Self(body.to_vec()))
      })
  }
}