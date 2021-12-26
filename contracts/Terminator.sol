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

/// @dev Terminator is an example of smartcontract which is used for liquidation bots
/// Current version gets information from the bot, withdraw all YEARN tokens and then
/// convert everything on Uniswap V2 protocol into underlying asset.
///
/// This is just an example contract, do not use it in productuion.
/// This constract is not audited, use it on your own risk.

contract Terminator is Ownable {
    using SafeMath for uint256;
    using SafeERC20 for IERC20;

    // List of yearn vaults connected
    address[] public yearn;

    // WETH token address
    address public wethToken;

    // Set or executros. Executors could call liquidate function.
    // This role is design to make parallel liquidations from different liquidaiton
    // without waiting confirmations for your address
    mapping(address => bool) public executors;

    // Parameters for Uniswap V2 which provides information
    struct UniV2Params {
        uint256 amountIn;
        address[] path;
        uint256 amountOutMin;
    }

    modifier executorOnly() {
        require(executors[msg.sender], "For executors only");
        _;
    }

    constructor(address _wethToken) {
        wethToken = _wethToken;
    }

    // Adds an executor to the set
    function allowExecutor(address _executor) external onlyOwner {
        executors[_executor] = true;
    }

    // Removes executor from the set
    function forbidExecutor(address _executor) external onlyOwner {
        executors[_executor] = false;
    }

    // Adds yVault to the list
    function addYearn(address _yearn) external onlyOwner {
        yearn.push(_yearn);
    }

    /// @dev Liquidates account and sell all assets in Uniswap V2
    /// @param _creditManager address of credit manager
    /// @param _borrower address of borrower
    /// (the system finds credit account by these 2 params)
    /// @param _router Address of UniV2 router
    /// @param _paths Data for uniswap - how to swap all assets into underlying one
    function liquidateAndSellOnV2(
        address _creditManager,
        address _borrower,
        address _router,
        UniV2Params[] calldata _paths
    ) external executorOnly {
        // Getting creditManager, creditFilter and underlyingToken
        ICreditManager creditManager = ICreditManager(_creditManager);
        ICreditFilter creditFilter = ICreditFilter(
            creditManager.creditFilter()
        );

        // Provides address of credit account of reverts. It'll revert if someone already liquidate it
        address creditAccount = creditManager.getCreditAccountOrRevert(
            _borrower
        );

        // Gets the quantity of allowed tokens
        uint256 allowedTokenQty = creditFilter.allowedTokensCount();

        // Storing balances, they will not be available after liquidation
        uint256[] memory balancesBefore = new uint256[](allowedTokenQty);

        // Getting enabledTokens - token mask which represents non-zero balances
        uint256 enabledTokens = creditFilter.enabledTokens(creditAccount);

        uint256 tokenMask;

        // Checks that parameters array which provide params for swaping assets has the same size
        require(_paths.length == allowedTokenQty, "Incorrect path length");

        // Before executing liquidation method, we keep all balances of this smart contract to be able to
        // compute how many tokens of each asset we get during liquidation.
        // Terminator keeps different tokens to be able to liquidate different pools, it's why
        // it's crucial to distinguish balance we've already have and how many tokens we get.
        for (uint256 i = 1; i < allowedTokenQty; i++) {
            // Tokenmaks is a but mask which is used for gas saving during iteration for allowed tokens list
            tokenMask = 1 << i;
            if (enabledTokens & tokenMask > 0) {
                address token = creditFilter.allowedTokens(i);
                balancesBefore[i] = IERC20(token).balanceOf(address(this));
            }
        }
        // Providing allowance for creditManager to withdraw liquidation amount
        _provideAllowance(
            address(creditManager),
            creditManager.underlyingToken()
        );

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
        for (uint256 i = 0; i < yearn.length; i++) {
            if (IERC20(yearn[i]).balanceOf(address(this)) > 1) {
                IYVault(yearn[i]).withdraw();
            }
        }

        // The last thing is to swap all assets into underlying one
        // We compare current balace with stored one, and if it has difference
        // swap it into underlying account
        for (uint256 i = 1; i < allowedTokenQty; i++) {
            tokenMask = 1 << i;
            if (enabledTokens & tokenMask > 0) {
                address token = creditFilter.allowedTokens(i);
                uint256 balance = IERC20(token).balanceOf(address(this));

                if (balance > balancesBefore[i] + 1) {
                    balance = balance - balancesBefore[i] - 1;
                    require(token == _paths[i].path[0], "incorrect path");
                    sellTokensOnV2(_router, balance, _paths[i]);
                }
            }
        }
    }

    /// @dev Internal function which facilitate swap assets on Uniswap V2
    function sellTokensOnV2(
        address _router,
        uint256 amount,
        UniV2Params calldata paths
    ) internal {
        _provideAllowance(_router, paths.path[0]);

        // Computes amountOut based on rate provided as parameter
        uint256 amountOut = paths.amountOutMin.mul(amount).div(paths.amountIn);

        // By design, gearbox transfers eth if credit account has WETH position
        // So, we swap ETH using swapExactETHForTokens
        if (paths.path[0] == wethToken) {
            IUniswapV2Router02(_router).swapExactETHForTokens{value: amount}(
                amountOut,
                paths.path,
                address(this),
                block.timestamp
            );
        } else {
            IUniswapV2Router02(_router).swapExactTokensForTokens(
                amount,
                amountOut,
                paths.path,
                address(this),
                block.timestamp
            );
        }
    }

    /// @dev Sends tokens back to owner. Helps to take the profits back
    function transferToOwner(address token, uint256 amount) external onlyOwner {
        IERC20(token).safeTransfer(msg.sender, amount);
    }

    /// @dev Provides needed allowance
    function _provideAllowance(address externalContract, address token)
        internal
    {
        if (
            ERC20(token).allowance(address(this), externalContract) <
            Constants.MAX_INT_4
        ) {
            ERC20(token).approve(externalContract, Constants.MAX_INT);
        }
    }

    /// @dev Allows contract to get ETH during liquidation
    receive() external payable {}
}
