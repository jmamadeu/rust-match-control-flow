#[derive(Clone, Debug)]
enum Currency {
    OneCent,
    FiveCents,
    TenCents,
    TwentyFiveCents,
}

impl Currency {
    fn value(&self) -> u8 {
        match self {
            Currency::OneCent => 1,
            Currency::FiveCents => 5,
            Currency::TenCents => 10,
            Currency::TwentyFiveCents => 25,
        }
    }
}

impl Into<u8> for Currency {
    fn into(self) -> u8 {
        self.value()
    }
}

impl Into<u16> for Currency {
    fn into(self) -> u16 {
        self.value() as u16
    }
}
impl TryFrom<u8> for Currency {
    type Error =  Box<dyn std::error::Error>;

    fn try_from(value: u8) -> Result<Currency, Self::Error> {
        match value {
            10 => Ok(Currency::TenCents),
            5 => Ok(Currency::FiveCents),
            25 => Ok(Currency::TwentyFiveCents),
            1 => Ok(Currency::OneCent),
            _ => Err("Invalid currency value".into())
        }
    }
}
let n1 = Coin::try_from(19).expect("Invalid currency value");
use std::fmt;

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn main() {
    let value = Currency::FiveCents;
    let number_u8: u8 = value.into();
    let n1 = Currency::try_from(19).expect("Invalid currency value");

    println!("Value: {}", value);
    println!("Converted to u8: {}", number_u8);
    println!("Try From 19: {}", n1);
}
