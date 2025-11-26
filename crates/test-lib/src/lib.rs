use midenc_harness_macros::miden_test;

fn main() {
    println!("Hello")
}

#[cfg(test)]
mod test {
    use midenc_harness_macros::miden_test;

    #[miden_test]
    pub fn ham() {
        println!("Ham")
    }
}
