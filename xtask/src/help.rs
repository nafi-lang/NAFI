use crate::*;

impl flags::Help {
    pub fn run(self) -> Result<()> {
        println!("{}", crate::flags::Xtask::HELP);
        Ok(())
    }
}
