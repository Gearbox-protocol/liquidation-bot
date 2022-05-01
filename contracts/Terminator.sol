// SPDX-License-Identifier: MIT
pragma solidity ^0.7.4;
pragma abicoder v2;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/SafeERC20.sol";
import {SafeMath} from "@openzeppelin/contracts/math/SafeMath.sol";

import {Constants} from "./Constants.sol";
import {IUniswapV2Router02} from "./integrations/IUniswapV2Router02.sol";
import {IYVault} from "./integrations/IYVault.sol";

import {ICreditManager} from "./interfaces/ICreditManager.sol";
import {ICreditFilter} from "./interfaces/ICreditFilter.sol";
import {IWETH} from "./interfaces/IWETH.sol";
import {AbstractTerminator} from "./AbstractTerminator.sol";
import {ITerminator} from "./ITerminator.sol";

import "hardhat/console.sol";

/// @dev Terminator is an example of smartcontract which is used for liquidation bots
/// Current version gets information from the bot, withdraw all YEARN tokens and then
/// convert everything on Uniswap V2 protocol into underlying asset.
///
/// This is just an example contract, do not use it in productuion.
/// This constract is not audited, use it on your own risk.

contract Terminator is AbstractTerminator {
    using SafeMath for uint256;
    using SafeERC20 for IERC20;

    constructor(address _wethToken, address _beneficiary)
        AbstractTerminator(_wethToken, _beneficiary)
    {}

    /// @dev Liquidates account and sell all assets in Uniswap V2

    function liquidate(
        address _creditManager,
        address _borrower,
        UniV2Params[] memory _routes,
        address[] memory _yearnTokens
    ) external override executorOnly {
        // Getting creditManager, creditFilter and underlyingToken
        ICreditManager creditManager = ICreditManager(_creditManager);

        // Storing balances, they will not be available after liquidation
        uint256[] memory balancesBefore = new uint256[](_routes.length);

        // Before executing liquidation method, we keep all balances of this smart contract to be able to
        // compute how many tokens of each asset we get during liquidation.
        // Terminator keeps different tokens to be able to liquidate different pools, it's why
        // it's crucial to distinguish balance we've already have and how many tokens we get.
        for (uint256 i = 0; i < _routes.length; i++) {
            address token = _routes[i].path[0];
            balancesBefore[i] = IERC20(token).balanceOf(address(this));
        }

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
        creditManager.liquidateCreditAccount(_borrower, address(this), false);

        // At first stage, we just withdraw all yearn assets we got during liquidation
        _unwrapYearnTokens(_yearnTokens);

        // The last thing is to swap all assets into underlying one
        // We compare current balace with stored one, and if it has difference
        // swap it into underlying account
        for (uint256 i = 0; i < _routes.length; i++) {
            address token = _routes[i].path[0];
            uint256 balance = IERC20(token).balanceOf(address(this));

            if (balance > balancesBefore[i] + 1) {
                balance = balance - balancesBefore[i] - 1;
                sellTokensOnV2(balance, _routes[i]);
            }
        }
    }
}
