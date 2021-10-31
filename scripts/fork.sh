set -o allexport; source ./.env; set +o allexport;
npx hardhat node --fork https://mainnet.infura.io/v3/$INFURA_API_KEY
