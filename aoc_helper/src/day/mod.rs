pub trait Day<D, O1, O2> {
    fn parse(&self, input: String) -> D;
    fn part_1(&self, input: &D) -> O1;
    fn part_2(&self, input: &D) -> O2;
}
