pub trait Coord {
    fn to_num<T>(&self) -> T;
    fn to_xy_coord(&self, width: u16, height: u16) -> (u16, u16) {
        let n : u16 = self.to_num::<u16>();
        (n % width, n / height)
    }
}
