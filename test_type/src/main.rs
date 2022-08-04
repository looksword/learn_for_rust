fn arithmetic(m: i8, n: i8) {
    // 加法运算,有溢出风险
    println!("{}", m + n);
}

struct Point {
    x:i32,
    y:i32
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


    //as
    let i = 42;
    //先转为 *const i32,再转为 *mut i32
    let p = &i as *const i32 as *mut i32;
    println!("{:p}",p);

    //tuple
    let p = (1i32,2i32);
    let (a,b) = p;//模式匹配
    let x = p.0;//数字索引
    let y = p.1;
    println!("{} {} {} {}",a,b,x,y);

    //struct
    let p = Point{x:1,y:2};
    println!("Point is at {},{}",p.x,p.y);
    let Point{x:px,y:py} = p;
    println!("Point is at {},{}",px,py);

    //tuple struct
    struct Color(i32,i32,i32);
    struct T1{v:i32}//define struct
    struct T2(i32);//define tuple struct
    let v1 = T1{v:1};
    let v2 = T2(2);
    let v3 = T2{0:3};
    println!("{},{},{}",v1.v,v2.0,v3.0);

    

}
