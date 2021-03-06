// rustfmt-normalize_comments: true
// rustfmt-error_on_line_overflow: false

// Imports.

// Long import.
use exceedingly::loooooooooooooooooooooooooooooooooooooooooooooooooooooooong::import::path::{ItemA,
                                                                                             ItemB};
use exceedingly::looooooooooooooooooooooooooooooooooooooooooooooooooooooooooong::import::path::{ItemA,
                                                                                                ItemB};
use syntax::ast::{ItemDefaultImpl, ItemForeignMod, ItemImpl, ItemMac, ItemMod, ItemStatic};

use list::{// Another item
           AnotherItem, // Another Comment
           // Last Item
           LastItem,
           // Some item
           SomeItem /* Comment */};

use test::{/* A */ self /* B */, Other /* C */};

use Foo::{Bar, Baz};
use syntax;
pub use syntax::ast::{Expr, ExprAssign, ExprCall, ExprMethodCall, ExprPath, Expr_};
use {Bar /* comment */, /* Pre-comment! */ Foo};

use self;
use std::io;
use std::io;

mod Foo {
    pub use syntax::ast::{ItemDefaultImpl, ItemForeignMod, ItemImpl, ItemMac, ItemMod, ItemStatic};

    mod Foo2 {
        pub use syntax::ast::{self, ItemDefaultImpl, ItemForeignMod, ItemImpl, ItemMac, ItemMod,
                              ItemStatic};
    }
}

fn test() {
    use Baz::*;
    use Qux;
}

// Simple imports
use bar::quux as kaas;
use foo;
use foo::bar::baz;

// With aliases.
use foo as bar;
use foo::qux as bar;
use foo::{self as bar, baz};
use foo::{baz, qux as bar};

// With absolute paths
use Foo;
use foo;
use foo::Bar;
use foo::{Bar, Baz};
use {Bar, Baz};

// Root globs
use ::*;
use *;

// spaces used to cause glob imports to disappear (#1356)
use super::*;
use foo::issue_1356::*;

// We shouldn't remove imports which have attributes attached (#1858)
#[cfg(unix)]
use self::unix::{};

// nested imports
use foo::{a, b, boo, c,
          bar::{baz, qux, xxxxxxxxxxx, yyyyyyyyyyyyy, zzzzzzzzzzzzzzzz,
                foo::{a, b, cxxxxxxxxxxxxx, yyyyyyyyyyyyyy, zzzzzzzzzzzzzzzz}}};

use fooo::{bar, x, y, z,
           baar::foobar::{xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx, yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy,
                          zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz},
           bar::*};

// nested imports with a single sub-tree.
use a::b::c::d;
use a::b::c::*;
use a::b::c::{xxx, yyy, zzz};
