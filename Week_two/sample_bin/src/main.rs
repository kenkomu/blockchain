fn main() {
    fn sample(one:i32 , two:i32){
        println!("one is {} and two is {}", one, two)
    }
    fn sample_1(one:i32 , two:i32)-> i32{
        return one + two;
    }
    fn sample_2(one:i32 , two:i32)-> i32{
        one +two
    }
}
