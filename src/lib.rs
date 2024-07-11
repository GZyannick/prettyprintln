pub fn pretty_print<T>(content: T)
where
    T: std::fmt::Debug,
{
    println!("{:#?}", content);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        #[derive(Debug)]
        pub struct Pretty {
            str: String,
            nbr: usize,
        }
        let pretty = Pretty {
            str: "hellow".to_string(),
            nbr: 2,
        };

        pretty_print(pretty);
    }
}
