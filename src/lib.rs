use proc_macro::{Delimiter::Brace, TokenStream, TokenTree::Group};
use std::{fs::File, io::Read, os::unix::io::FromRawFd};

#[proc_macro]
pub fn whichever_compiles(input: TokenStream) -> TokenStream {
    let mut input = input.into_iter();
    let mut alternatives = Vec::new();
    while let Some(t) = input.next() {
        assert_eq!(t.to_string(), "try");
        match input.next() {
            Some(Group(g)) if g.delimiter() == Brace => alternatives.push(g.stream()),
            _ => panic!("{}", "expected `{ .. }` after `try`"),
        }
    }
    let last_alternative = alternatives.pop().expect("missing `try { .. }`");
    for tokens in alternatives {
        unsafe {
            let mut pipe = [-1, -1];
            assert_eq!(libc::pipe(pipe.as_mut_ptr()), 0);
            if libc::fork() == 0 {
                libc::dup2(pipe[1], 1);
                libc::dup2(pipe[1], 2);
                libc::close(pipe[0]);
                libc::close(pipe[1]);
                return tokens;
            }
            libc::close(pipe[1]);
            let mut out = String::new();
            File::from_raw_fd(pipe[0]).read_to_string(&mut out).unwrap();
            if !out.contains("\"error\"") && !out.contains("error:") {
                libc::_exit(0);
            }
        }
    }
    last_alternative
}
