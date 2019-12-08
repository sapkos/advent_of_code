fn main() {
    let memory: Vec<i32> = vec![1, 2, 3];
    let mut mem1 = memory.clone();
    mem1[1] = 2;
    let mut mem2 = memory.clone();
    println!("{}", mem2[1])
}
