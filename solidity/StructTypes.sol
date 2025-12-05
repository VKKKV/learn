// SPDX-License-Identifier: MIT

pragma solidity >=0.7.0;

contract StructTypes {
    // 我们定义一个结构体 Struct
    struct Student {
        uint256 id;
        uint256 score;
    }
    mapping(int => Student) public testVar;



}
