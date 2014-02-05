// Copyright 2014 Yuri Kunde Schlesner
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A command-line argument parsing library very loosely inspired by Python's argparse.

#[crate_id = "argparse#0.1"];
#[crate_type = "lib"];
#[license = "ASL2"];

enum IsRequired {
    Required, Optional
}

struct Flag;

struct Positional;

struct Arguments {
    tmp: ~str,
}

pub fn flag(name: &str, description: &str) -> Flag {
    Flag
}

pub fn parameter(name: &str, required: IsRequired, description: &str) -> Positional {
    Positional
}

impl Flag {
    pub fn alias(&self, name: &str) -> Flag {
        *self
    }

    pub fn required(&self) -> Flag {
        *self
    }

    pub fn has_option(&self, required: IsRequired) -> Flag {
        *self
    }

    pub fn default(&self, value: &str) -> Flag {
        *self
    }

    pub fn multiple(&self) -> Flag {
        *self
    }
}

impl Positional {
    pub fn default(&self, value: &str) -> Positional {
        *self
    }

    pub fn multiple(&self) -> Positional {
        *self
    }
}

impl<'a> Arguments {
    pub fn present(&self, name: &str) -> bool {
        false
    }

    pub fn value(&'a self, name: &str) -> &'a str {
        self.tmp.as_slice()
    }

    pub fn values(&self, name: &str) -> &[&str] {
        &[]
    }
}

pub fn argparse(flags: &[Flag], params: &[Positional]) -> Option<Arguments> {
    Some(Arguments{tmp: ~"lol"})
}

pub fn handle_help(args: Option<Arguments>) -> Arguments {
    args.unwrap()
}

#[test]
fn test_func() {
    use std::io::println;

    let flags = [
        flag("--banana", "Desc").alias("-b").required().has_option(Required),
        flag("--012345", "Desc").alias("-a").has_option(Required).default("20"),
        flag("--kiwi", "Desc").alias("-k"),
        flag("-p", "Desc").has_option(Optional).default("30"),
        flag("-l", "Desc").has_option(Required).multiple(),
    ];
    let params = [
        parameter("foo", Required, "Desc"),
        parameter("bar", Optional, "Desc").default("foo"),
        parameter("baz", Required, "Desc").multiple(),
    ];

    // handle_help automatically unwraps the Option value and, if --help was passed or an usage
    // error was encountered, prints usage information and terminates the program.
    let args = handle_help(argparse(flags, params));

    if args.present("--banana") {
        println!("banana is {}", args.value("--banana"));
    }

    println!("012345 is (or default): {}", args.value("--012345"));

    for x in args.values("-l").iter() {
        println(*x);
    }

    // the interface for positional arguments is identical to that of options
    println!("foo is {}", args.value("foo"));
}
