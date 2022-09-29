use crate::{Account};

#[derive(Debug)]
pub enum PaymentCurrency {
    USDollar,
    QRRiyal,
}

#[derive(Debug)]
pub enum PaymentResult {
    Completed,
    BalanceFull,
}

#[derive(Debug)]
pub enum WithdrawResult {
    Completed,
    BalanceInsufficient,
}

pub struct PaymentContainer {
    pub currency: PaymentCurrency,
    pub account: Account
}

impl PaymentContainer {

    pub fn process_payment(
        &mut self, amount: i32,
    ) -> Result<PaymentResult, PaymentResult> {
        if self.account.balance >= self.account.limit {
            return Err(PaymentResult::BalanceFull);
        }

        self.account.balance += amount;

        println!(
            "User {} paid {}, balance is now {}. used currency {:?}",
            self.account.holder.username, amount, self.account.balance, self.currency
        );

        Ok(PaymentResult::Completed)
    }

    pub fn withdraw_money(
        &mut self, amount: i32,
    ) -> Result<WithdrawResult, WithdrawResult> {
        if amount < self.account.balance {
            return Err(WithdrawResult::BalanceInsufficient);
        }

        self.account.balance -= amount;

        println!(
            "User {} withdrawed {}, balance is now {}. used currency {:?}",
            self.account.holder.username, amount, self.account.balance, self.currency
        );

        Ok(WithdrawResult::Completed)
    }
}
