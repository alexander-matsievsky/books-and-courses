#[cfg(test)]
mod tests {
    use std::mem::transmute;

    //noinspection SpellCheckingInspection
    #[test]
    fn main1() {
        let a: f32 = 42.42;
        let frankentype: u32 = unsafe { transmute(a) };
        println!("{:032b}", frankentype);
    }
}
