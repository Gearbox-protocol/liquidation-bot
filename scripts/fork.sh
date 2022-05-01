set -o allexport; source ./.env; set +o allexport;

export $(grep -v '^#' .env | xargs -d '\n')
npx hardhat node --fork $ETH_MAINNET_PROVIDER
