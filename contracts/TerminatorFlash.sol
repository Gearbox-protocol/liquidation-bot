// SPDX-License-Identifier: agpl-3.0
pragma solidity ^0.7.4;
pragma abicoder v2;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {ILendingPool} from "./interfaces/ILendingPool.sol";
import {ILendingPoolAddressesProvider} from "./interfaces/ILendingPoolAddressesProvider.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/SafeERC20.sol";
import {SafeMath} from "@openzeppelin/contracts/math/SafeMath.sol";
import {IFlashLoanReceiver} from "./interfaces/IFlashLoanReceiver.sol";
import {Constants} from "./Constants.sol";
import {IUniswapV2Router02} from "./integrations/IUniswapV2Router02.sol";
import {IYVault} from "./integrations/IYVault.sol";
import {IWETH} from "./interfaces/IWETH.sol";

import {ICreditManager} from "./interfaces/ICreditManager.sol";
import {ICreditFilter} from "./interfaces/ICreditFilter.sol";
import {AbstractTerminator} from "./AbstractTerminator.sol";
import {ITerminator} from "./ITerminator.sol";

import "hardhat/console.sol";

/**
    !!!
    Never keep funds permanently on your FlashLoanReceiverBase contract as they could be
    exposed to a 'griefing' attack, where the stored funds are used by an attacker.
    !!!
 */
contract TerminatorFlash is AbstractTerminator, IFlashLoanReceiver {
    using SafeERC20 for IERC20;
    using SafeMath for uint256;

    // Constants

    // Aave Address Provider of Mainnet
    address constant aaveAP = 0xB53C1a33016B2DC2fF3653530bfF1848a515c8c5;


    // Flashloan logic
    ILendingPoolAddressesProvider public immutable override ADDRESSES_PROVIDER;
    ILendingPool public immutable override LENDING_POOL;

    constructor(address _wethToken, address _beneficiary)
        AbstractTerminator(_wethToken, _beneficiary)
    {
        ADDRESSES_PROVIDER = ILendingPoolAddressesProvider(aaveAP);
        LENDING_POOL = ILendingPool(
            ILendingPoolAddressesProvider(aaveAP).getLendingPool()
        );


    }

    /**
        This function is called after your contract has received the flash loaned amount
     */
    function executeOperation(
        address[] calldata assets,
        uint256[] calldata amounts,
        uint256[] calldata premiums,
        address initiator,
        bytes calldata params
    ) external override returns (bool) {
        {
            //
            // This contract now has the funds requested.
            // Your logic goes here.
            //

            (
                address _creditManager,
                address _borrower,
                UniV2Params[] memory _routes,
                address[] memory _yearnTokens
            ) = abi.decode(
                    params,
                    (address, address, UniV2Params[], address[])
                );
            ICreditManager creditManager = ICreditManager(_creditManager);

            /**
             * @dev Liquidates credit account
             * - Transfers discounted total credit account value from liquidators account
             * - Pays borrowed funds + interest + fees back to pool, than transfers remaining funds to credit account owner
             * - Transfer all assets from credit account to liquidator ("to") account
             * - Returns credit account to factory
             * - Emits LiquidateCreditAccount event
             *
             * More info: https://dev.gearbox.fi/developers/credit/credit_manager#liquidate-credit-account
             *
             * @param borrower Borrower address
             * @param to Address to transfer all assets from credit account
             * @oaram force For force liquidation (ignore reverts for token transfers)
             */
            creditManager.liquidateCreditAccount(
                _borrower,
                address(this),
                false
            );

            _unwrapYearnTokens(_yearnTokens);
            // The last thing is to swap all assets into underlying one
            // We compare current balace with stored one, and if it has difference
            // swap it into underlying account
            for (uint256 i = 0; i < _routes.length; i++) {
                sellTokensOnV2(
                    IERC20(_routes[i].path[0]).balanceOf(address(this)),
                    _routes[i]
                );
            }
        }
        // At the end of your logic above, this contract owes
        // the flashloaned amounts + premiums.
        // Therefore ensure your contract has enough to repay
        // these amounts.

        uint256 balance = IERC20(assets[0]).balanceOf(address(this));

        // Approve the LendingPool contract allowance to *pull* the owed amount
        uint256 amountOwing = amounts[0].add(premiums[0]);

        console.log("AFTERSALE");
        console.log(balance);
        console.log(amountOwing);

        IERC20(assets[0]).approve(address(LENDING_POOL), amountOwing);
        IERC20(assets[0]).transfer(
            beneficiary,
            balance.sub(amountOwing).sub(1)
        );

        return true;
    }

    function liquidate(
        address _creditManager,
        address _borrower,
        UniV2Params[] memory _routes,
        address[] memory _yearnTokens
    ) external override executorOnly {

        console.log("===============================================================================");
        console.log("===============================   LIQUIDATION   ===============================");
        console.log("===============================================================================");

        ICreditManager creditManager = ICreditManager(_creditManager);

        uint256[] memory amounts = new uint256[](1);
        amounts[0] = creditManager.calcRepayAmount(_borrower, true);


        address[] memory assets = new address[](1);
        assets[0] = creditManager.underlyingToken();

        // 0 = no debt, 1 = stable, 2 = variable
        uint256[] memory modes = new uint256[](1);
        modes[0] = 0;

        bytes memory params = abi.encode(
            _creditManager,
            _borrower,
            _routes,
            _yearnTokens
        );

        LENDING_POOL.flashLoan(
            address(this),
            assets,
            amounts,
            modes,
            address(this),
            params,
            0
        );
    }
}
