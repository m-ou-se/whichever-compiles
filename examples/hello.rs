use whichever_compiles::whichever_compiles;

fn main() {
    whichever_compiles! {
        try { thisfunctiondoesntexist(); }
        try { invalid syntax 1 2 3 }
        try { println!("missing arg: {}"); }
        try { println!("hello {}", world()); }
        try { 1 + 2 }
    }
}

whichever_compiles! {
    try { }
    try { fn world() {} }
    try { fn world() -> &'static str { "world" } }
}
