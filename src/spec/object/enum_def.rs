#[test]
fn def_enum() {
    #[derive(Debug)]
    #[allow(unused)]
    enum Number {
        Zero=0,
        One, // 默认1
        Two,
        Three,
    }

    #[allow(unused)]
    enum Number1 {
        Zero=0,
        One=1,
        Two=10,
    }
    fn print_suit(card: Number) {
        println!("{:?}", card);
    }
    let three = Number::Three;
    let two = Number::Two;
    print_suit(three);
    print_suit(two);

    // as convert
    assert_eq!(Number::One as u8, Number1::One as u8);
    assert_ne!(Number::Two as u8, Number1::Two as u8);

}

#[test]
fn def_enum_with_value(){
    // 任何类型的数据都可以放入枚举成员中
    #[allow(unused)]
    enum PokerCard {
        Spades(u8),
        Diamonds(String),
        Move { x: i32, y: i32 },//代表匿名struct

    }
    
    let _s1 = PokerCard::Spades(5);
    let _p1 = PokerCard::Diamonds("A".to_string());
    let _m = PokerCard::Move{x:1,y:1};
}

#[test]
fn move_enum_value(){
    #[derive(Debug)]
    enum PokerCard {
        Spades(u8),
        Diamonds(String),
    }
    
    let s1 = PokerCard::Spades(5);
    let p1 = PokerCard::Diamonds("A".to_string());
    dbg!(&s1);
    let _s2 = s1; //s1 is moved
    let _p2 = p1; //p1 is moved
    // dbg!(&s1);//error
}

#[test]
fn copy_enum_owner(){
    #[derive(Debug)]
    #[allow(unused)]
    enum Number {
        Zero=0,
    }

    let s1 = Number::Zero;//copied
    let s2 = Number::Zero;//copied
    dbg!(&s1);
    dbg!(&s2);
    dbg!(&Number::Zero);
}