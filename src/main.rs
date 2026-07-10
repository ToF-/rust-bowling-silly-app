
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    
    use speculoos::assert_that;

    #[test]
    fn dummy_test() {
        assert_that!(2+2 == 4);
    }
}
