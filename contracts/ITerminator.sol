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

/// @dev Terminator is an example of smartcontract which is used for liquidation bots
/// Current version gets information from the bot, withdraw all YEARN tokens and then
/// convert everything on Uniswap V2 protocol into underlying asset.
///
/// This is just an example contract, do not use it in productuion.
/// This constract is not audited, use it on your own risk.

interface ITerminator {
    function executors(address) external view returns (bool);

    // Parameters for Uniswap V2 which provides information
    struct UniV2Params {
        uint256 amountIn;
        address[] path;
        uint256 amountOutMin;
    }

    // Adds an executor to the set
    function allowExecutor(address _executor) external;

    // Removes executor from the set
    function forbidExecutor(address _executor) external;

    function provideAllowance(
        address[] calldata creditManagers,
        address[] calldata tokens
    ) external;

    function liquidate(
        address _creditManager,
        address _borrower,
        UniV2Params[] memory _routes,
        address[] memory _yearnTokens
    ) external;
}
