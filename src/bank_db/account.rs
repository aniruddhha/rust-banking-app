#[derive(Debug, Clone)]
pub struct Account {
    id: u32,
    balance: f64,
} 

impl Account {
    pub fn new(id: u32, balance: f64) -> Self {
        Account { id, balance }
    }

    pub fn get_balance(&self) -> f64 {
        self.balance
    }

    pub fn set_balance(&mut self, balance: f64) {
        self.balance = balance;
    }
    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }
}