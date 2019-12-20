# TODO

Models:

* Budgets
    * Settings
        * ISO Code: str
        * Decimal Digits: i32
        * Decimal Separator: str
        * Symbol First: bool
        * Group Separator: str
        * Currency Symbol: str
        * Display Symbol: bool
    * Accounts
        * Name: str
        * Type: "checking" | ...
        * On Budget: bool
        * Closed: bool
        * Note: str
        * Balance: ap_int
        * Cleared Balance: ap_int
        * Uncleared Balance: ap_int
        * Transfer Payee: TransferPayeeId
        * Deleted: bool
    * Category Groups
        * Name: str
        * Hidden: bool
        * Deleted: bool
        * Categories
            * Name: str
            * Hidden: bool
            * Original Category Group: CategoryGroupId
            * Note: str
            * Budgeted: ap_int
            * Activity: ap_int
            * Balance: ap_int
            * Goal Type: TB=Target Category Balance | TBD=Target Category Balance by Date | MF=Monthly Funding
            * Goal Creation Month: Month
            * Goal Target: ap_int
            * Goal Target Month: Month
            * Goal Percentage Complete: ap_int
            * Deleted: bool
* Payees
* Months
* Transactions
* Scheduled Transactions
*
