
#[derive(Clone, Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Into<u8> for Coin {
    fn into(self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

impl Into<u16> for Coin {
    fn into(self) -> u16 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

impl TryFrom<u8> for Coin {
    type Error =  Box<dyn std::error::Error>;

    fn try_from(value: u8) -> Result<Coin, Self::Error> {
        match value {
            10 => Ok(Coin::Dime),
            5 => Ok(Coin::Nickel),
            25 => Ok(Coin::Quarter),
            1 => Ok(Coin::Penny),
            _ => Err("Error".into())
        }
    }
}

fn main() {
    let value = Coin::Nickel;
    let number_u8: u8 = value.into();
    let n1 = Coin::try_from(19).expect("Error");

    println!("{:?}", number_u8)
}
