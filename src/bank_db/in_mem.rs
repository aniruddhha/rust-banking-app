use crate::bank_db::account;
use crate::bank_db::bank_account;

impl bank_account::BankAccount for account::Account {
    fn deposit(&mut self, amount: f64) {
        self.set_balance(self.balance() + amount);
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), &'static str> {
        if amount <= self.balance() {
            self.set_balance(self.balance() - amount);
            Ok(())
        } else {
            Err("Insufficient funds")
        }
    }

    fn balance(&self) -> f64 {
      self.get_balance()
    }
}