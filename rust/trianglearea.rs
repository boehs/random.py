#!/usr/bin/env rust-script
//! ```cargo
//! [package]
//! name = "trianglearea"
//! authors = ["Evan Boehs"]
//! version = "0.1.0"
//! license = "Commons Clause Apache-2.0"
//! edition = "2018"
//!
//! [dependencies]
//! 
//! ```
// Copyright 2021, Evan Boehs

#![forbid(unsafe_code)]
use std::io::stdin;

struct Triangle {
  lineone: f64,
  linetwo: f64,
  linethree: f64,
}

impl Triangle {
  fn area(&self) -> f64 {
    let s = (self.lineone+self.linetwo+self.linethree) / 2.0;
    (s * (s-self.lineone) * (s-self.linetwo) * (s-self.linethree)).sqrt()
  }
}

fn main() {
  fn gentri() -> f64 {
    let mut tri = String::new();
    stdin().read_line(&mut tri).ok().expect("Failed to read line");
    tri.trim().parse().unwrap()
  }
  let tri = Triangle {
    lineone: {
      println!("Enter the length of the first line");
      gentri()
    },
    linetwo: {
      println!("Enter the length of the second line");
      gentri()
    },
    linethree: {
      println!("Enter the length of the third line");
      gentri()
    }
  };
  println!("The result is {}",tri.area());
  
}

// vim: set ft=rust sw=4 sts=4 expandtab :