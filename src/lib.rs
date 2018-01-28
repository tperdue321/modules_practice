// using semi-colon like this tells rust to
// look for the contents of mod client in
// client.rs
mod client;

// tells lib.rs to look to network.rs for the
// contents of the network module
mod network;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
