#[path = "../counting/count.rs"]
pub mod count;

pub fn add(left: u64, right: u64) -> u64 {
    let result = left + right;
    count::count_to(result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
