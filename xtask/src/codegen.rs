use crate::*;

mod syntax;

impl flags::Codegen {
    pub fn run(mut self) -> Result<()> {
        if !(self.syntax) {
            self.all = true;
        }

        if (self.syntax || self.all) && syntax::codegen()? {
            println!("  Recodegen'd syntax");
        }

        Ok(())
    }
}
