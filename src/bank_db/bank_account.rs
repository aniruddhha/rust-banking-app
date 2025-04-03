pub trait BankAccount {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64) -> Result<(), &'static str>;
    fn balance(&self) -> f64;
}
