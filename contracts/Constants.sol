// SPDX-License-Identifier: MIT
pragma solidity ^0.7.4;

library Constants {
    uint256 constant MAX_INT =
        0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff;

    // 25% of MAX_INT
    uint256 constant MAX_INT_4 =
        0x3fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff;

    // Seconds in a year
    uint256 constant SECONDS_PER_YEAR = 365 days;

    // 1e18
    uint256 constant WAD = 1e18;

}
