## Gearbox Protocol Liquidator

This repository contains an example of liquidation bot which could be used for Gearbox Protocol.  
This bot liquidates credit account and swap assets on Uniswap V2 compatible protocol.

### What is Gearbox protocol?

Gearbox is a generalized leverage protocol: it allows you to take leverage in one place and then use it across various 
DeFi protocols and platforms in a composable way. The protocol has two sides to it: passive liquidity providers who earn higher APY 
by providing liquidity; active traders, farmers, or even other protocols who can borrow those assets to trade or farm with x4+ leverage.

Gearbox protocol is Marketmake ETHGlobal hackathon finalist.

## How to install

1. Clone this repo: `git clone git@github.com:Gearbox-protocol/liquidation-bot.git`
2. Install npm packages: `yarn`
3. Copy `.env.example` to `.env`: ` cp .env.example .env`
4. Fill setting. Check `.env.kovan` as working example
5. Deploy `Terminator.sol` smart contract: `yarn deploy --network <NETWORK>`
6. Enter address of deployed contract as `BOT_ADDRESS` in `.env` file
7. Run the bot: `cargo run`

## Building and Running in Release mode

```
# Build in release mode
cargo build --release

#Run it
./target/release/liq_rs
```

## Useful links
Website: [https://gearbox.fi/](https://gearbox.fi/)  
Docs: [https://docs.gearbox.finance/](https://docs.gearbox.finance/)  
Forum: [https://gov.gearbox.fi/t/start-here-forum-rules/](https://gov.gearbox.fi/t/start-here-forum-rules/)  
Blog: [https://medium.com/@gearboxprotocol](https://medium.com/@gearboxprotocol)  
Twitter: [https://twitter.com/GearboxProtocol](https://twitter.com/GearboxProtocol)  
Snapshot page: [https://snapshot.org/#/gearbox.eth](https://snapshot.org/#/gearbox.eth)  
Developer Docs: [https://dev.gearbox.fi/](https://dev.gearbox.fi/)  

## Licensing

The primary license for this Liquidation bot is the AGPL-3.0.

## Disclaimer

This application is provided "as is" and "with all faults." Me as developer makes no representations or
warranties of any kind concerning the safety, suitability, lack of viruses, inaccuracies, typographical
errors, or other harmful components of this software. There are inherent dangers in the use of any software,
and you are solely responsible for determining whether this software product is compatible with your equipment and
other software installed on your equipment. You are also solely responsible for the protection of your equipment
and backup of your data, and THE PROVIDER will not be liable for any damages you may suffer in connection with using,
modifying, or distributing this software product.
