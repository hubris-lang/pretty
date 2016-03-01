//! This crate defines a
//! [Wadler-style](http://homepages.inf.ed.ac.uk/wadler/papers/prettier/prettier.pdf)
//! pretty-printing API.
#![feature(into_cow)]
use doc::{
    best,
};
use doc::Doc::{
    Append,
    Group,
    Nest,
    Newline,
    Nil,
    Text,
};

use std::io;
use std::borrow::{IntoCow};
use std::ops::{Add, Sub};
use std::fmt::{self, Display, Formatter};

mod doc;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Doc<'a>(doc::Doc<'a>);

impl<'a> Doc<'a> {
    #[inline]
    pub fn nil() -> Doc<'a> {
        Doc(Nil)
    }

    #[inline]
    pub fn append(self, that: Doc<'a>) -> Doc<'a> {
        let Doc(ldoc) = self;
        let Doc(rdoc) = that;
        let res = match ldoc {
            Nil  => rdoc,
            ldoc => match rdoc {
                Nil  => ldoc,
                rdoc => Append(Box::new(ldoc), Box::new(rdoc)),
            }
        };
        Doc(res)
    }

    #[inline]
    pub fn as_string<T: ToString>(t: T) -> Doc<'a> {
        Doc::text(t.to_string())
    }

    #[inline]
    pub fn concat(ds: &[Doc<'a>]) -> Doc<'a> {
        ds.iter().fold(Doc::nil(), |a, b| a.append(b.clone()))
    }

    #[inline]
    pub fn group(self) -> Doc<'a> {
        let Doc(doc) = self;
        Doc(Group(Box::new(doc)))
    }

    #[inline]
    pub fn nest(self, off: usize) -> Doc<'a> {
        let Doc(doc) = self;
        Doc(Nest(off, Box::new(doc)))
    }

    #[inline]
    pub fn newline() -> Doc<'a> {
        Doc(Newline)
    }

    #[inline]
    pub fn render<W: io::Write>(&self, width: usize, out: &mut W) -> io::Result<()> {
        let &Doc(ref doc) = self;
        best(doc, width, out)
    }

    #[inline]
    pub fn text<T: IntoCow<'a, str>>(data: T) -> Doc<'a> {
        Doc(Text(data.into_cow()))
    }
}

pub fn parens(doc: Doc) -> Doc {
    Doc::text("(") + doc + Doc::text(")")
}

pub fn braces(doc: Doc) -> Doc {
    Doc::text("{") + doc + Doc::text("}")
}

pub fn format<T: Pretty>(s: &T, formatter: &mut Formatter) -> Result<(), fmt::Error> {
    let mut v = Vec::new();
    try!(Doc::render(&s.pretty(), 80, &mut v).map_err(|_| fmt::Error));
    write!(formatter, "{}", try!(String::from_utf8(v).map_err(|_| fmt::Error)))
}

impl<'a> Add for Doc<'a> {
    type Output = Doc<'a>;
    fn add(self, other: Doc<'a>) -> Doc<'a> {
        self.append(other)
    }
}

// impl<'a, T: 'a + Pretty> Add<T> for Doc<'a> {
//     type Output = Doc<'a>;
//     fn add(self, other: T) -> Doc<'a> {
//         self.append(other.pretty())
//     }
// }

pub trait Pretty {
    fn pretty(&self) -> Doc;
}

impl<'a> Pretty for Doc<'a> {
    fn pretty(&self) -> Doc {
        self.clone()
    }
}

impl Pretty for String {
    fn pretty(&self) -> Doc {
        Doc::text(self.as_str())
    }
}

impl Pretty for str {
    fn pretty(&self) -> Doc {
        Doc::text(self)
    }
}

// impl<T: Pretty> Display for T {
//     fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
//         Doc::render(self.pretty(), 80, formatter)
//     }
// }
