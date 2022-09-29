pub struct AccountHolder {
    pub id: isize,
    pub username: String
}

pub struct Account {
    pub holder: AccountHolder,
    pub balance: i32,
    pub limit: i32
}
