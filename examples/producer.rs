use std::convert::Infallible;
use std::rc::Rc;
use std::str::FromStr;
use std::thread;

use unused::Unused;

trait Producer {
    type Output;

    fn produce(&self, data: &str) -> Self::Output;
}

struct FromStrProducer<T: FromStr> {
    _unused: Unused<T>,
}

impl<T: FromStr> Producer for FromStrProducer<T> {
    type Output = T;

    fn produce(&self, data: &str) -> Self::Output {
        data.parse().ok().unwrap()
    }
}

#[derive(Eq, PartialEq, Debug)]
struct RcString(Rc<String>);

impl FromStr for RcString {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Rc::new(s.to_owned())))
    }
}

fn main() {
    let producer = FromStrProducer::<RcString> {
        _unused: Unused::new(),
    };

    thread::spawn(move || {
        assert_eq!(producer.produce("a"), RcString(Rc::new("a".to_owned())));
    })
    .join()
    .unwrap();
}
