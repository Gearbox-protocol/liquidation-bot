// SPDX-License-Identifier: MIT
pragma solidity ^0.7.4;
pragma abicoder v2;

import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";
import { ERC20 } from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/SafeERC20.sol";
import { SafeMath } from "@openzeppelin/contracts/math/SafeMath.sol";

import { Constants } from "./Constants.sol";
import { IUniswapV2Router02 } from "./integrations/IUniswapV2Router02.sol";
import { IYVault } from "./integrations/IYVault.sol";

import { ICreditManager } from "./interfaces/ICreditManager.sol";
import { ICreditFilter } from "./interfaces/ICreditFilter.sol";

import "hardhat/console.sol";

contract Terminator is Ownable {
  using SafeMath for uint256;
  using SafeERC20 for IERC20;

  address[] public yearn;

  // KOVAN
//  address constant YEARN_DAI_KOVAN_MOCK = 0xe5267045739E4d6FcA15BB4a79190012F146893b;
//  address constant YEARN_USDC_KOVAN_MOCK = 0x980E4d8A22105c2a2fA2252B7685F32fc7564512;


  // MAINNET
//address constant YEARN_DAI_KOVAN_MOCK =
//0xdA816459F1AB5631232FE5e97a05BBBb94970c95;
//  address constant YEARN_USDC_KOVAN_MOCK =
//0x5f18C75AbDAe578b483E5F43f12a39cF75b973a9;

  address public wethToken;

  mapping(address => bool) public executors;

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

  function allowExecutor(address _executor) external onlyOwner {
    executors[_executor] = true;
  }

  function forbidExecutor(address _executor) external onlyOwner {
    executors[_executor] = false;
  }


  function addYearn(address _yearn) external onlyOwner {
    yearn.push(_yearn);
  }

  function liquidateAndSellOnV2(
    address _creditManager,
    address _borrower,
    address _router,
    UniV2Params[] calldata _paths
  ) external executorOnly {
    // Getting creditManager, creditFilter and underlyingToken
    ICreditManager creditManager = ICreditManager(_creditManager);
    ICreditFilter creditFilter = ICreditFilter(creditManager.creditFilter());

    // Provides address of credit account of reverts. It'll revert if someone already liquidate it
    address creditAccount = creditManager.getCreditAccountOrRevert(_borrower);

    uint256 allowedTokenQty = creditFilter.allowedTokensCount();

    // Storing balances, they will not be available after liquidation
    uint256[] memory balancesBefore = new uint256[](allowedTokenQty);

    // Getting enabledTokens - token mask which represents non-zero balances
    uint256 enabledTokens = creditFilter.enabledTokens(creditAccount);

    uint256 tokenMask;

    require(_paths.length == allowedTokenQty, "Incorrect path length");

    for (uint256 i = 1; i < allowedTokenQty; i++) {
      tokenMask = 1 << i;
      if (enabledTokens & tokenMask > 0) {
        address token = creditFilter.allowedTokens(i);
        balancesBefore[i] = IERC20(token).balanceOf(address(this));
      }
    }
    // Providing allowance for creditManager to withdraw liquidation amount
    _provideAllowance(address(creditManager), creditManager.underlyingToken());
    creditManager.liquidateCreditAccount(_borrower, address(this), false);

    for(uint i=0; i<yearn.length; i++) {
      if (IERC20(yearn[i]).balanceOf(address(this)) > 1) {
        IYVault(yearn[i]).withdraw();
      }

    }

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

  function sellTokensOnV2(address _router, uint256 amount, UniV2Params calldata paths) internal {
    _provideAllowance(_router, paths.path[0]);


      uint256 amountOut = paths.amountOutMin.mul(amount).div(paths.amountIn);

      if (paths.path[0] == wethToken) {
        IUniswapV2Router02(_router).swapExactETHForTokens{ value: amount }(
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

  // @dev sends tokens back
  function transferToOwner(address token, uint256 amount) external onlyOwner {
    IERC20(token).safeTransfer(msg.sender, amount);
  }

  function _provideAllowance(address externalContract, address token) internal {
    console.log(ERC20(token).allowance(address(this), externalContract));
    if (
      ERC20(token).allowance(address(this), externalContract) <
      Constants.MAX_INT_4
    ) {
      ERC20(token).approve(externalContract, Constants.MAX_INT);
    }
  }

  receive() external payable {}
}
