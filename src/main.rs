use account::{Account, AccountHolder};
use payment::{PaymentContainer, PaymentCurrency};

mod payment;
mod account;

fn main() {
    let holder = AccountHolder {
        id: 0,
        username: String::from("GrowlyX")
    };

    let mut account = Account {
        holder,
        balance: 10,
        limit: 3000
    };

    let mut paymentContainer = PaymentContainer {
        currency: PaymentCurrency::QRRiyal,
        account
    };

    while &account.balance < &3000 {
        paymentContainer
            .process_payment(500)
            .expect("Wow, so you're literally full bro.");
    }

    paymentContainer
        .withdraw_money(200)
        .expect("you are broke");
}
