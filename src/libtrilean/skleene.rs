// Copyright 2015 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// `SKleene` is a structure implementing the strong Kleene's three-valued logic (see README.md and the truth tables below).

use skleene::SKleene::*;
use core::ops::*;
use core::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SKleene
{
  False = 0,
  True = 1,
  Unknown = 2
}

const NOT_TABLE: [SKleene; 3] =
  // False, True, Unknown
     [True, False, Unknown];
const AND_TABLE: [[SKleene; 3]; 3] =
  /* False */   [[ False, False, False ],
  /* True  */    [ False, True, Unknown],
  /* Unknown */  [ False, Unknown, Unknown]
  ];

const OR_TABLE: [[SKleene; 3]; 3] =
  //               False, True, Unknown
  /* False */   [[ False, True, Unknown ],
  /* True  */    [ True, True, True],
  /* Unknown */  [ Unknown, True, Unknown]
  ];

impl SKleene
{
  pub fn and(self, rhs: SKleene)-> SKleene {
    AND_TABLE[self as usize][rhs as usize]
  }

  pub fn or(self, rhs: SKleene) -> SKleene {
    OR_TABLE[self as usize][rhs as usize]
  }

  pub fn from_bool(b: bool) -> SKleene {
    NOT_TABLE[(!b) as usize]
  }
}

impl Not for SKleene
{
  type Output = SKleene;

  fn not(self) -> SKleene {
    NOT_TABLE[self as usize]
  }
}

impl BitAnd for SKleene
{
  type Output = Self;
  fn bitand(self, rhs: Self) -> Self {
    self.and(rhs)
  }
}

impl BitOr for SKleene
{
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self {
    self.or(rhs)
  }
}


impl fmt::Display for SKleene
{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      False => write!(f, "false"),
      True => write!(f, "true"),
      Unknown => write!(f, "unknown"),
    }
  }
}

#[test]
fn test_not() {
  assert_eq!(!True, False);
  assert_eq!(!False, True);
  assert_eq!(!Unknown, Unknown);
}

#[test]
fn test_and() {
  let f = |a: SKleene, b, e| {
    assert_eq!(a.and(b), e);
    assert_eq!(b.and(a), e);
    assert_eq!(a & b, e);
  };
  f(True, True, True);
  f(True, False, False);
  f(True, Unknown, Unknown);
  f(False, False, False);
  f(False, Unknown, False);
  f(Unknown, Unknown, Unknown);
}

#[test]
fn test_or() {
  let f = |a: SKleene, b, e| {
    assert_eq!(a.or(b), e);
    assert_eq!(b.or(a), e);
    assert_eq!(a | b, e);
  };
  f(True, True, True);
  f(True, False, True);
  f(True, Unknown, True);
  f(False, False, False);
  f(False, Unknown, Unknown);
  f(Unknown, Unknown, Unknown);
}

#[test]
fn test_from_bool() {
  assert_eq!(SKleene::from_bool(true), True);
  assert_eq!(SKleene::from_bool(false), False);
}
