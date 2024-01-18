use arbitrary::Arbitrary;

#[derive(Arbitrary, Debug, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
    pub name: String,
}


pub fn is_fuzz(color: Color) {
    
    #[cfg(not(fuzzing))]
    println!("{color:?}");
    
    if color.r == 22 {
        if color.g == 22 {
            if color.b == 24 {
                if color.a == 0 {
                    panic!("solved");
                }
            }
        }
    }
}
