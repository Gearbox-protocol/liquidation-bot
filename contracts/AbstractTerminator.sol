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
import {ITerminator} from "./ITerminator.sol";

import "hardhat/console.sol";

/// @dev Terminator is an example of smartcontract which is used for liquidation bots
/// Current version gets information from the bot, withdraw all YEARN tokens and then
/// convert everything on Uniswap V2 protocol into underlying asset.
///
/// This is just an example contract, do not use it in productuion.
/// This constract is not audited, use it on your own risk.

abstract contract AbstractTerminator is Ownable, ITerminator {
    using SafeMath for uint256;
    using SafeERC20 for IERC20;

    // WETH token address
    address public immutable wethToken;

    address constant UNISWAP_V2_ROUTER =
        0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D;

    IUniswapV2Router02 public immutable uniV2Router;

    address public immutable beneficiary;

    // Set or executros. Executors could call liquidate function.
    // This role is design to make parallel liquidations from different liquidaiton
    // without waiting confirmations for your address
    mapping(address => bool) public override executors;

    modifier executorOnly() {
        require(executors[msg.sender], "For executors only");
        _;
    }

    constructor(address _wethToken, address _beneficiary) {
        wethToken = _wethToken;
        uniV2Router = IUniswapV2Router02(UNISWAP_V2_ROUTER);
        beneficiary = _beneficiary;
        allowExecutor(msg.sender);
    }

    // Adds an executor to the set
    function allowExecutor(address _executor) public override onlyOwner {
        executors[_executor] = true;
    }

    // Removes executor from the set
    function forbidExecutor(address _executor) external override onlyOwner {
        executors[_executor] = false;
    }

    function provideAllowance(
        address[] calldata creditManagers,
        address[] calldata tokens
    ) external override onlyOwner {
        for (uint256 i = 0; i < creditManagers.length; i++) {
            ICreditManager creditManager = ICreditManager(creditManagers[i]);
            // Providing allowance for creditManager to withdraw liquidation amount
            ERC20(creditManager.underlyingToken()).approve(
                creditManagers[i],
                Constants.MAX_INT
            );
        }

        for (uint256 i = 0; i < tokens.length; i++) {
            // Providing allowance for creditManager to withdraw liquidation amount
            ERC20(tokens[i]).approve(UNISWAP_V2_ROUTER, Constants.MAX_INT);
        }
    }

    /// @dev Internal function which facilitate swap assets on Uniswap V2
    function sellTokensOnV2(uint256 amount, UniV2Params memory paths) internal {
        // Computes amountOut based on rate provided as parameter
        uint256 amountOut = paths.amountOutMin.mul(amount).div(paths.amountIn);

        try uniV2Router.swapExactTokensForTokens(
                amount,
                amountOut,
                paths.path,
                address(this),
                block.timestamp
            )
        {} catch Error(string memory /*reason*/) {
            console.log("UNISWAP PROBLEM");
            console.log(amount);
            console.log(amountOut);
            console.log(uniV2Router.getAmountsOut(amount, paths.path)[0]);
            console.log(uniV2Router.getAmountsOut(amount, paths.path)[1]);
            address from = paths.path[0];
            address to = paths.path[paths.path.length - 1];
            console.log(from);
            console.log(to);
        }
    }

    function _unwrapYearnTokens(address[] memory yearnTokens) internal {
        // At first stage, we just withdraw all yearn assets we got during liquidation
        for (uint256 i = 0; i < yearnTokens.length; i++) {
            if (IERC20(yearnTokens[i]).balanceOf(address(this)) > 1) {
                IYVault(yearnTokens[i]).withdraw();
            }
        }
    }

    /// @dev Allows contract to get ETH during liquidation
    receive() external payable {
        IWETH(wethToken).deposit{value: msg.value}();
    }

    /// @dev Sends tokens back to owner. Helps to take the profits back
    function transferToOwner(address token, uint256 amount) external onlyOwner {
        IERC20(token).safeTransfer(beneficiary, amount);
    }
}
