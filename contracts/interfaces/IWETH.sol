// SPDX-License-Identifier: GPL-2.0-or-later

pragma solidity >=0.7.4;

interface IWETH {
    function deposit() external payable;
    function transfer(address to, uint value) external returns (bool);
    function withdraw(uint256) external;
}
