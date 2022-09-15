use chrono::NaiveDate;
use rust_decimal::Decimal;
use std::fmt::Display;

#[derive(Debug)]
pub struct Entry {
    date: NaiveDate,
    secondary_date: Option<NaiveDate>,
    description: String,
    comments: Vec<String>,
    transactions: Vec<Transaction>,
}

impl Entry {
    pub fn new(date: NaiveDate, description: String, transactions: Vec<Transaction>) -> Self {
        Self {
            date,
            secondary_date: None,
            description,
            comments: Vec::new(),
            transactions,
        }
    }

    pub fn with_secondary_date(mut self, date: NaiveDate) -> Self {
        self.secondary_date = Some(date);
        self
    }

    pub fn with_comment(mut self, comment: String) -> Self {
        self.comments.push(comment);
        self
    }

    pub fn add_comment(&mut self, comment: String) {
        self.comments.push(comment);
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.date)?;
        if let Some(secondary_date) = self.secondary_date {
            write!(f, "={}", secondary_date)?;
        }
        writeln!(f, " {}", self.description)?;
        for comment in &self.comments {
            writeln!(f, "\t; {}", comment)?;
        }
        for transaction in &self.transactions {
            writeln!(f, "\t{}", transaction)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Transaction {
    account: String,
    amount: Decimal,
    commodity: String,
    comment: Option<String>,
}

impl Transaction {
    pub fn new(account: String, amount: Decimal, commodity: String) -> Self {
        Self {
            account,
            amount,
            commodity,
            comment: None,
        }
    }

    pub fn with_comment(mut self, comment: String) -> Self {
        self.comment = Some(comment);
        self
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\t{} {}", self.account, self.amount, self.commodity)?;
        if let Some(comment) = &self.comment {
            write!(f, " ; {}", comment)
        } else {
            Ok(())
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
