fn arithmetic(m: i8, n: i8) {
    // 加法运算,有溢出风险
    println!("{}", m + n);
}

fn main() {
    let elem = 5u8;//由后缀推断类型
    let mut vec = Vec::new();
    vec.push(elem);
    println!("{:?}",vec);//由上下文推断类型
    let player_scores = [
        ("Jack",20),("Jane",23),("Jill",18),("John",19),
        ];
    //players 动态数组,内部成员类型由编译器自动推导
    let players : Vec<_> = player_scores
        .iter()
        .map(|&(player,_score)|{player})
        .collect();
    println!("players{:?}",players);

    //全局变量
    static G1 : i32 = 3;
    println!("G1:{}",G1);
    //可变全局变量读写
    static mut G2 : i32 = 4;
    unsafe {
        G2 = 5;
        println!("G2:{}",G2);
    }

    //let booltest = false;
    //let cahrtest = ' ';
    let var1 : i32 = 32; // 十进制表示
    let var2 : i32 = 0xFF; // 以0x开头代表十六进制表示
    let var3 : i32 = 0o55; // 以0o开头代表八进制表示
    let var4 : i32 = 0b1001; // 以0b开头代表二进制表示
    let var5 = 0x_1234_ABCD;
    let var6 = 123usize; // i6变量是usize类型
    let var7 = 0x_ff_u8; // i7变量是u8类型
    let var8 = 32;// 不写类型,默认为 i32 类型
    println!("var1:{}",var1);
    println!("var2:{}",var2);
    println!("var3:{}",var3);
    println!("var4:{}",var4);
    println!("var5:{}",var5);
    println!("var6:{}",var6);
    println!("var7:{}",var7);
    println!("var8:{}",var8);

    //溢出检查
    //let m : i8 = 120;
    //let n : i8 = 120;
    //arithmetic(m,n);

    // 变量 small 初始化为一个非常小的浮点数
    let mut small = std::f32::EPSILON;
    // 不断循环,让 small 越来越趋近于 0,直到最后等于0的状态
    while small > 0.0 {
        small = small / 2.0;
        println!("{} {:?}", small, small.classify());//Subnormal状态
    }
    let nan = std::f32::NAN;
    println!("{} {} {}", nan < nan, nan > nan, nan == nan);//NAN不具备"全序"

}
