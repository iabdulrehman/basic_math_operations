pub mod calculate
{
    pub fn addition(no1: u8, no2: u8)
    {
        let result:u16 = (no1 + no2).into();
        println!("Addition : {}",result);
    }

    pub fn subtraction(no1: u8, no2: u8)
    {
        let result:i16 = (no1 - no2).into();
        println!("Subtraction : {}",result);
    }

    pub fn multiplication(no1: u8, no2: u8)
    {
        let result:u16 = (no1 * no2).into();
        println!("Multiplication : {}",result);
    }

    pub fn division(no1: u8, no2: u8)
    {
        let result:i16 = (no1 / no2).into();
        println!("Division : {}",result);
    }
}
