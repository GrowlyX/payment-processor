use std::io::stdin;
use account::{Account, AccountHolder};
use payment::{PaymentContainer, PaymentCurrency};

mod payment;
mod account;

fn main() {
    let mut payment_container = PaymentContainer {
        currency: PaymentCurrency::QRRiyal,
        account: Account {
            holder: AccountHolder {
                id: 0,
                username: String::from("GrowlyX")
            },
            balance: 10,
            limit: 3000
        }
    };

    println!("what number do u want get to");

    let mut buffer = String::new();

    stdin()
        .read_line(&mut buffer)
        .expect("something went wrong");

    let amount = buffer
        .trim()
        .parse::<i32>()
        .unwrap();

    while payment_container.account.balance < amount {
        payment_container
            .process_payment(500)
            .expect("you went over the limit of 3000 :(");
    }

    let mut buffer_withdrawal = String::new();

    println!("how much are you wanting to withdraw");

    stdin()
        .read_line(&mut buffer_withdrawal)
        .expect("something went wrong");

    let amount_withdrawal = buffer_withdrawal
        .trim()
        .parse::<i32>()
        .unwrap();

    payment_container
        .withdraw_money(amount_withdrawal)
        .expect("you are broke");
}
