// Privacy Rules
// Overall, these are the rules for item visibility:
//
// If an item is public, it can be accessed through any of its parent modules.
//
// If an item is private, it can be accessed only by its immediate parent
// module and any of the parent’s child modules.


// using semi-colon like this tells rust to
// look for the contents of mod client in
// client.rs
pub mod client;

// tells lib.rs to look to network.rs for the
// contents of the network module
// using "pub" allows this function
// to be used outside of the library crate
pub mod network;


// setup to explore private/public compiling
// failure/success situations 
// pub mod outermost {
//     pub fn middle_function() {}

//     fn middle_secret_function() {}

//     pub mod inside {
//         pub fn inner_function() {
//             ::outermost::middle_secret_function();
//         }

//         fn secret_function() {}
//     }
// }

// fn try_me() {
//     outermost::middle_function();
//     outermost::middle_secret_function();
//     outermost::inside::inner_function();
//     outermost::inside::secret_function();
// }

#[cfg(test)]
mod tests {
    // super goes up a one module
    // can be good for accessing siblings easily
    use super::client;
    #[test]
    fn it_works() {
        client::connect();
    }
}
