use codec::Codec;

pub mod codec;
pub mod parse;
pub mod state;
pub mod types;
pub mod utils;

struct List1OrNil<'a, T>(&'a Vec<T>, &'a [u8]);

impl<'a, T> Codec for List1OrNil<'a, T>
where
    T: Codec,
{
    fn serialize(&self) -> Vec<u8> {
        if let Some((last, head)) = self.0.split_last() {
            let mut out = b"(".to_vec();

            for item in head {
                out.extend(&item.serialize());
                out.extend_from_slice(self.1);
            }

            out.extend(&last.serialize());

            out.push(b')');
            out
        } else {
            b"NIL".to_vec()
        }
    }

    fn deserialize(_input: &[u8]) -> Result<(&[u8], Self), Self>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}
