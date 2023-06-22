// edition:2021
// revisions: classic next
//[next] compile-flags: -Ztrait-solver=next
//[next] check-pass

#![feature(unboxed_closures)]

use std::future::Future;

trait Trait {
    fn func(&self, _: &str);
}

impl<T> Trait for T
where
    for<'a> T: Fn<(&'a str,)> + Send + Sync,
    for<'a> <T as FnOnce<(&'a str,)>>::Output: Future<Output = usize> + Send,
{
    fn func(&self, _: &str) {
        println!("hello!");
    }
}

async fn strlen(x: &str) -> usize {
    x.len()
}

fn main() {
    strlen.func("hi");
}
