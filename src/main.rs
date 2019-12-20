use chrono::{Date, Utc};

/// The date format setting for the budget.
struct DateFormat(String);

/// The currency format setting for the budget.
struct CurrencyFormat {
    iso_code: CurrencyISOCode,
    dec_digits: i32,
    dec_sep: Separator,
    symbol_first: bool,
    group_sep: Separator,
    currency_symbol: String,
    display_symbol: bool,
}

/// The supported currency ISO codes.
enum CurrencyISOCode {
    Eur,
    Brl,
    Usd,
}

/// The currency separators for decimals and groups.
enum Separator {
    Comma,
    Period,
}

/// The customizable options for the budget.
struct BudgetSettings {
    date_format: DateFormat,
    currency_format: CurrencyFormat,
}

/// An account holds a currency balance that may be used on a budget.
struct Account {
    /// A descriptive name for this account.
    name: String,
    /// The type of account, e.g. whether it's a checking, asset or liability
    /// kind of account.
    kind: AccountKind,
    /// Whether this account is on budget or not.
    on_budget: bool,
    /// Whether this account is closed or not.
    is_closed: bool,
    /// A description for this account.
    note: String,
    /// The current balance of the account in milliunits format.
    balance: i64,
    /// The current cleared balance of the account in milliunits format.
    cleared_balance: i64,
    /// The current uncleared balance of the account in milliunits format.
    uncleared_balance: i64,
    /// Whether or not this account has been deleted.
    is_deleted: bool,
}

/// The type of account.
enum AccountKind {
    Checking,
    Savings,
    Cash,
    CreditCard,
    LineOfCredit,
    OtherAsset,
    OtherLiability,
}

struct CategoryGroup {
    name: String,
    is_hidden: bool,
    is_deleted: bool,
}

struct Category {
    category_group: CategoryGroupId,
    name: String,
    is_hidden: bool,
    note: String,
    /// Budgeted amount in milliunits format.
    budgeted: i64,
    /// Activity amount in milliunits format.
    activity: i64,
    /// Balance in milliunits format.
    balance: i64,
    /// The category goal if any.
    goal: Option<Goal>,
}

struct Goal {
    kind: GoalKind,
    creation_month: Date<Utc>,
}

// TODO(feroldi): each of these goal types has some attributes to it, such as
// target balance or date. Find out which ones are still missing.
enum GoalKind {
    TargetCategoryBalance {
        target_balance: i64,
    },
    TargetCategoryBalanceByDate {
        target_balance: i64,
        target_date: Date<Utc>,
    },
    MonthlyFunding {
        funding_balance: i64,
    },
}

struct Payee {
    name: String,
    transfer_account: Option<AccountId>,
    is_deleted: bool,
}

struct AccountId(usize);
struct PayeeId(usize);
struct CategoryGroupId(usize);
struct CategoryId(usize);

struct Transaction {
    date: Date<Utc>,
    /// The transaction amount in milliunits format.
    amount: i64,
    account: AccountId,
    payee: PayeeId,
    category: CategoryId,
    transfer_account: Option<AccountId>,
    memo: String,
    cleared: ClearedStatus,
    approved: bool,
    flag_color: String,
}

enum ClearedStatus {
    Cleared,
    Uncleared,
    Reconciled,
}

struct Budget {
    name: String,
    first_month: Date<Utc>,
    last_month: Date<Utc>,
    settings: BudgetSettings,
    accounts: Vec<Account>,
    payees: Vec<Payee>,
    category_groups: Vec<CategoryGroup>,
    categories: Vec<Category>,
    months: Vec<Month>,
    transactions: Vec<Transaction>,
}

struct Month {
    month: Date<Utc>,
    note: String,
    /// The total amount in transactions categorized to 'Inflow: To be Budgeted' in the month.
    income: i64,
    /// The total amount budgeted in the month.
    budgeted: i64,
    /// The total amount in transactions in the month, excluding those categorized to 'Inflow: To be Budgeted'.
    activity: i64,
    /// The available amount for 'To be Budgeted'.
    to_be_budgeted: i64,
    /// The Age of Money as of the month.
    age_of_money: i64,
    /// Whether or not the month has been deleted.
    is_deleted: bool,
}

fn main() {
    println!("Hello, world!");
}
