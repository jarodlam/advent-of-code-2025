pub trait Solution {
    type Output;
    fn part1(input: &str) -> Self::Output;
    fn part2(input: &str) -> Self::Output;
}

pub fn solve<T>(input: &str) -> (String, String)
where
    T: Solution,
    T::Output: ToString,
{
    let output1 = T::part1(input).to_string();
    let output2 = T::part2(input).to_string();
    (output1, output2)
}
