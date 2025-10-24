
## TODO for crate
- [ ] Ready for publish

## Fix StockSymbol:
Nasdaq currently restricts its symbol length to a maximum of 8 characters. 
For common stock issuances, Nasdaq, PSX and BX will only assign root symbols 
of 1 to 4 characters in length with possible fifth and or sixth character 
denoting a suffix. 
In certain instances, a dot “.” delimiter may be applied to symbols after the 
root and between the suffix e.g., XXXX.A. 
For subordinate securities, Nasdaq and BX will assign a 5 character symbol for
which the last character relays information about the issue class or issue type.
For the current list of fifth and or six character symbol suffixes, 
please refer to Ticker Symbol Convention page on the NasdaqTrader website.

For NYSE-, NYSE American- and NYSE Arca-listed securities with 
subordinate issue types, please refer to Ticker Symbol Convention page 
on the Nasdaq Trader website.

## enum macro
- [x] make docs optional?

