// ---------------------------------------------------------
// This file was generated by parol.
// It is not intended for manual editing and changes will be
// lost after next build.
// ---------------------------------------------------------

use id_tree::Tree;
use parol_runtime::lexer::{TokenStream, Tokenizer};
use parol_runtime::parser::errors::*;
use parol_runtime::parser::{
    DFATransition, LLKParser, LookaheadDFA, ParseTreeType, ParseType, Production, UserActionsTrait,
};
use std::cell::RefCell;

use parol_runtime::lexer::tokenizer::{
    ERROR_TOKEN, NEW_LINE_TOKEN, UNMATCHABLE_TOKEN, WHITESPACE_TOKEN,
};

pub const TERMINALS: &[&str; 23] = &[
    /*  0 */ UNMATCHABLE_TOKEN,
    /*  1 */ NEW_LINE_TOKEN,
    /*  2 */ WHITESPACE_TOKEN,
    /*  3 */ r###"//.*(\r\n|\r|\n|$)"###,
    /*  4 */ r###"(?ms)/\*.*?\*/"###,
    /*  5 */ r###";"###,
    /*  6 */ r###"==|!="###,
    /*  7 */ r###"(\+|-|\*|/|%|<<|>>|&|^|\|)?="###,
    /*  8 */ r###"\|\|"###,
    /*  9 */ r###"&&"###,
    /* 10 */ r###"\|"###,
    /* 11 */ r###"&"###,
    /* 12 */ r###"<<|>>"###,
    /* 13 */ r###"<=|<|>=|>"###,
    /* 14 */ r###"\+"###,
    /* 15 */ r###"-"###,
    /* 16 */ r###"\*\*"###,
    /* 17 */ r###"\*|/|%"###,
    /* 18 */ r###"\("###,
    /* 19 */ r###"\)"###,
    /* 20 */ r###"\d+"###,
    /* 21 */ r###"[a-zA-Z_]\w*"###,
    /* 22 */ ERROR_TOKEN,
];

pub const TERMINAL_NAMES: &[&str; 23] = &[
    /*  0 */ "EndOfInput",
    /*  1 */ "Newline",
    /*  2 */ "Whitespace",
    /*  3 */ "LineComment",
    /*  4 */ "BlockComment",
    /*  5 */ "Semicolon",
    /*  6 */ "EqualityOp",
    /*  7 */ "AssignOp",
    /*  8 */ "LogicalOrOp",
    /*  9 */ "LogicalAndOp",
    /* 10 */ "BitwiseOrOp",
    /* 11 */ "BitwiseAndOp",
    /* 12 */ "BitwiseShiftOp",
    /* 13 */ "RelationalOp",
    /* 14 */ "Plus",
    /* 15 */ "Minus",
    /* 16 */ "PowOp",
    /* 17 */ "MultOp",
    /* 18 */ "LParen",
    /* 19 */ "RParen",
    /* 20 */ "Number",
    /* 21 */ "Id",
    /* 22 */ "Error",
];

const MAX_K: usize = 2;

pub const NON_TERMINALS: &[&str; 65] = &[
    /*  0 */ "add_op",
    /*  1 */ "assign_item",
    /*  2 */ "assign_op",
    /*  3 */ "assignment",
    /*  4 */ "assignment_lst1",
    /*  5 */ "assignment_lst1_itm1",
    /*  6 */ "bitwise_and",
    /*  7 */ "bitwise_and_item",
    /*  8 */ "bitwise_and_lst1",
    /*  9 */ "bitwise_and_lst1_itm1",
    /* 10 */ "bitwise_and_op",
    /* 11 */ "bitwise_or",
    /* 12 */ "bitwise_or_item",
    /* 13 */ "bitwise_or_lst1",
    /* 14 */ "bitwise_or_lst1_itm1",
    /* 15 */ "bitwise_or_op",
    /* 16 */ "bitwise_shift",
    /* 17 */ "bitwise_shift_item",
    /* 18 */ "bitwise_shift_lst1",
    /* 19 */ "bitwise_shift_lst1_itm1",
    /* 20 */ "bitwise_shift_op",
    /* 21 */ "calc",
    /* 22 */ "calc_lst1",
    /* 23 */ "calc_lst1_itm1",
    /* 24 */ "equality",
    /* 25 */ "equality_item",
    /* 26 */ "equality_lst1",
    /* 27 */ "equality_lst1_itm1",
    /* 28 */ "equality_op",
    /* 29 */ "factor",
    /* 30 */ "id",
    /* 31 */ "idref",
    /* 32 */ "instruction",
    /* 33 */ "logical_and",
    /* 34 */ "logical_and_item",
    /* 35 */ "logical_and_lst1",
    /* 36 */ "logical_and_lst1_itm1",
    /* 37 */ "logical_and_op",
    /* 38 */ "logical_or",
    /* 39 */ "logical_or_item",
    /* 40 */ "logical_or_lst1",
    /* 41 */ "logical_or_lst1_itm1",
    /* 42 */ "logical_or_op",
    /* 43 */ "minus",
    /* 44 */ "mult",
    /* 45 */ "mult_item",
    /* 46 */ "mult_lst1",
    /* 47 */ "mult_lst1_itm1",
    /* 48 */ "mult_op",
    /* 49 */ "negate",
    /* 50 */ "number",
    /* 51 */ "plus",
    /* 52 */ "pow_op",
    /* 53 */ "power",
    /* 54 */ "power_lst1",
    /* 55 */ "power_lst1_itm1",
    /* 56 */ "relational",
    /* 57 */ "relational_item",
    /* 58 */ "relational_lst1",
    /* 59 */ "relational_lst1_itm1",
    /* 60 */ "relational_op",
    /* 61 */ "summ",
    /* 62 */ "summ_item",
    /* 63 */ "summ_lst1",
    /* 64 */ "summ_lst1_itm1",
];

pub const LOOKAHEAD_AUTOMATA: &[LookaheadDFA; 65] = &[
    /* 0 - "add_op" */
    LookaheadDFA {
        states: &[None, Some(60), Some(61)],
        transitions: &[DFATransition(0, 14, 1), DFATransition(0, 15, 2)],
        k: 1,
    },
    /* 1 - "assign_item" */
    LookaheadDFA {
        states: &[Some(8)],
        transitions: &[],
        k: 0,
    },
    /* 2 - "assign_op" */
    LookaheadDFA {
        states: &[Some(7)],
        transitions: &[],
        k: 0,
    },
    /* 3 - "assignment" */
    LookaheadDFA {
        states: &[Some(9)],
        transitions: &[],
        k: 0,
    },
    /* 4 - "assignment_lst1" */
    LookaheadDFA {
        states: &[None, None, Some(10), None, None, None, Some(12)],
        transitions: &[
            DFATransition(0, 15, 3),
            DFATransition(0, 18, 4),
            DFATransition(0, 20, 5),
            DFATransition(0, 21, 1),
            DFATransition(1, 5, 6),
            DFATransition(1, 6, 6),
            DFATransition(1, 7, 2),
            DFATransition(1, 8, 6),
            DFATransition(1, 9, 6),
            DFATransition(1, 10, 6),
            DFATransition(1, 11, 6),
            DFATransition(1, 12, 6),
            DFATransition(1, 13, 6),
            DFATransition(1, 14, 6),
            DFATransition(1, 15, 6),
            DFATransition(1, 16, 6),
            DFATransition(1, 17, 6),
            DFATransition(3, 15, 6),
            DFATransition(3, 18, 6),
            DFATransition(3, 20, 6),
            DFATransition(3, 21, 6),
            DFATransition(4, 15, 6),
            DFATransition(4, 18, 6),
            DFATransition(4, 20, 6),
            DFATransition(4, 21, 6),
            DFATransition(5, 5, 6),
            DFATransition(5, 6, 6),
            DFATransition(5, 8, 6),
            DFATransition(5, 9, 6),
            DFATransition(5, 10, 6),
            DFATransition(5, 11, 6),
            DFATransition(5, 12, 6),
            DFATransition(5, 13, 6),
            DFATransition(5, 14, 6),
            DFATransition(5, 15, 6),
            DFATransition(5, 16, 6),
            DFATransition(5, 17, 6),
        ],
        k: 2,
    },
    /* 5 - "assignment_lst1_itm1" */
    LookaheadDFA {
        states: &[Some(11)],
        transitions: &[],
        k: 0,
    },
    /* 6 - "bitwise_and" */
    LookaheadDFA {
        states: &[Some(31)],
        transitions: &[],
        k: 0,
    },
    /* 7 - "bitwise_and_item" */
    LookaheadDFA {
        states: &[Some(36)],
        transitions: &[],
        k: 0,
    },
    /* 8 - "bitwise_and_lst1" */
    LookaheadDFA {
        states: &[None, Some(32), Some(34)],
        transitions: &[
            DFATransition(0, 5, 2),
            DFATransition(0, 8, 2),
            DFATransition(0, 9, 2),
            DFATransition(0, 10, 2),
            DFATransition(0, 11, 1),
            DFATransition(0, 19, 2),
        ],
        k: 1,
    },
    /* 9 - "bitwise_and_lst1_itm1" */
    LookaheadDFA {
        states: &[Some(33)],
        transitions: &[],
        k: 0,
    },
    /* 10 - "bitwise_and_op" */
    LookaheadDFA {
        states: &[Some(35)],
        transitions: &[],
        k: 0,
    },
    /* 11 - "bitwise_or" */
    LookaheadDFA {
        states: &[Some(25)],
        transitions: &[],
        k: 0,
    },
    /* 12 - "bitwise_or_item" */
    LookaheadDFA {
        states: &[Some(30)],
        transitions: &[],
        k: 0,
    },
    /* 13 - "bitwise_or_lst1" */
    LookaheadDFA {
        states: &[None, Some(26), Some(28)],
        transitions: &[
            DFATransition(0, 5, 2),
            DFATransition(0, 8, 2),
            DFATransition(0, 9, 2),
            DFATransition(0, 10, 1),
            DFATransition(0, 19, 2),
        ],
        k: 1,
    },
    /* 14 - "bitwise_or_lst1_itm1" */
    LookaheadDFA {
        states: &[Some(27)],
        transitions: &[],
        k: 0,
    },
    /* 15 - "bitwise_or_op" */
    LookaheadDFA {
        states: &[Some(29)],
        transitions: &[],
        k: 0,
    },
    /* 16 - "bitwise_shift" */
    LookaheadDFA {
        states: &[Some(49)],
        transitions: &[],
        k: 0,
    },
    /* 17 - "bitwise_shift_item" */
    LookaheadDFA {
        states: &[Some(53)],
        transitions: &[],
        k: 0,
    },
    /* 18 - "bitwise_shift_lst1" */
    LookaheadDFA {
        states: &[None, Some(50), Some(52)],
        transitions: &[
            DFATransition(0, 5, 2),
            DFATransition(0, 6, 2),
            DFATransition(0, 8, 2),
            DFATransition(0, 9, 2),
            DFATransition(0, 10, 2),
            DFATransition(0, 11, 2),
            DFATransition(0, 12, 1),
            DFATransition(0, 13, 2),
            DFATransition(0, 19, 2),
        ],
        k: 1,
    },
    /* 19 - "bitwise_shift_lst1_itm1" */
    LookaheadDFA {
        states: &[Some(51)],
        transitions: &[],
        k: 0,
    },
    /* 20 - "bitwise_shift_op" */
    LookaheadDFA {
        states: &[Some(42)],
        transitions: &[],
        k: 0,
    },
    /* 21 - "calc" */
    LookaheadDFA {
        states: &[Some(0)],
        transitions: &[],
        k: 0,
    },
    /* 22 - "calc_lst1" */
    LookaheadDFA {
        states: &[None, Some(1), Some(3)],
        transitions: &[
            DFATransition(0, 0, 2),
            DFATransition(0, 15, 1),
            DFATransition(0, 18, 1),
            DFATransition(0, 20, 1),
            DFATransition(0, 21, 1),
        ],
        k: 1,
    },
    /* 23 - "calc_lst1_itm1" */
    LookaheadDFA {
        states: &[Some(2)],
        transitions: &[],
        k: 0,
    },
    /* 24 - "equality" */
    LookaheadDFA {
        states: &[Some(37)],
        transitions: &[],
        k: 0,
    },
    /* 25 - "equality_item" */
    LookaheadDFA {
        states: &[Some(41)],
        transitions: &[],
        k: 0,
    },
    /* 26 - "equality_lst1" */
    LookaheadDFA {
        states: &[None, Some(38), Some(40)],
        transitions: &[
            DFATransition(0, 5, 2),
            DFATransition(0, 6, 1),
            DFATransition(0, 8, 2),
            DFATransition(0, 9, 2),
            DFATransition(0, 10, 2),
            DFATransition(0, 11, 2),
            DFATransition(0, 19, 2),
        ],
        k: 1,
    },
    /* 27 - "equality_lst1_itm1" */
    LookaheadDFA {
        states: &[Some(39)],
        transitions: &[],
        k: 0,
    },
    /* 28 - "equality_op" */
    LookaheadDFA {
        states: &[Some(6)],
        transitions: &[],
        k: 0,
    },
    /* 29 - "factor" */
    LookaheadDFA {
        states: &[None, Some(75), Some(76), Some(77), Some(78)],
        transitions: &[
            DFATransition(0, 15, 3),
            DFATransition(0, 18, 4),
            DFATransition(0, 20, 1),
            DFATransition(0, 21, 2),
        ],
        k: 1,
    },
    /* 30 - "id" */
    LookaheadDFA {
        states: &[Some(81)],
        transitions: &[],
        k: 0,
    },
    /* 31 - "idref" */
    LookaheadDFA {
        states: &[Some(80)],
        transitions: &[],
        k: 0,
    },
    /* 32 - "instruction" */
    LookaheadDFA {
        states: &[None, None, Some(4), None, None, None, Some(5)],
        transitions: &[
            DFATransition(0, 15, 3),
            DFATransition(0, 18, 4),
            DFATransition(0, 20, 5),
            DFATransition(0, 21, 1),
            DFATransition(1, 5, 6),
            DFATransition(1, 6, 6),
            DFATransition(1, 7, 2),
            DFATransition(1, 8, 6),
            DFATransition(1, 9, 6),
            DFATransition(1, 10, 6),
            DFATransition(1, 11, 6),
            DFATransition(1, 12, 6),
            DFATransition(1, 13, 6),
            DFATransition(1, 14, 6),
            DFATransition(1, 15, 6),
            DFATransition(1, 16, 6),
            DFATransition(1, 17, 6),
            DFATransition(3, 15, 6),
            DFATransition(3, 18, 6),
            DFATransition(3, 20, 6),
            DFATransition(3, 21, 6),
            DFATransition(4, 15, 6),
            DFATransition(4, 18, 6),
            DFATransition(4, 20, 6),
            DFATransition(4, 21, 6),
            DFATransition(5, 5, 6),
            DFATransition(5, 6, 6),
            DFATransition(5, 8, 6),
            DFATransition(5, 9, 6),
            DFATransition(5, 10, 6),
            DFATransition(5, 11, 6),
            DFATransition(5, 12, 6),
            DFATransition(5, 13, 6),
            DFATransition(5, 14, 6),
            DFATransition(5, 15, 6),
            DFATransition(5, 16, 6),
            DFATransition(5, 17, 6),
        ],
        k: 2,
    },
    /* 33 - "logical_and" */
    LookaheadDFA {
        states: &[Some(19)],
        transitions: &[],
        k: 0,
    },
    /* 34 - "logical_and_item" */
    LookaheadDFA {
        states: &[Some(24)],
        transitions: &[],
        k: 0,
    },
    /* 35 - "logical_and_lst1" */
    LookaheadDFA {
        states: &[None, Some(20), Some(22)],
        transitions: &[
            DFATransition(0, 5, 2),
            DFATransition(0, 8, 2),
            DFATransition(0, 9, 1),
            DFATransition(0, 19, 2),
        ],
        k: 1,
    },
    /* 36 - "logical_and_lst1_itm1" */
    LookaheadDFA {
        states: &[Some(21)],
        transitions: &[],
        k: 0,
    },
    /* 37 - "logical_and_op" */
    LookaheadDFA {
        states: &[Some(23)],
        transitions: &[],
        k: 0,
    },
    /* 38 - "logical_or" */
    LookaheadDFA {
        states: &[Some(13)],
        transitions: &[],
        k: 0,
    },
    /* 39 - "logical_or_item" */
    LookaheadDFA {
        states: &[Some(18)],
        transitions: &[],
        k: 0,
    },
    /* 40 - "logical_or_lst1" */
    LookaheadDFA {
        states: &[None, Some(14), Some(16)],
        transitions: &[
            DFATransition(0, 5, 2),
            DFATransition(0, 8, 1),
            DFATransition(0, 19, 2),
        ],
        k: 1,
    },
    /* 41 - "logical_or_lst1_itm1" */
    LookaheadDFA {
        states: &[Some(15)],
        transitions: &[],
        k: 0,
    },
    /* 42 - "logical_or_op" */
    LookaheadDFA {
        states: &[Some(17)],
        transitions: &[],
        k: 0,
    },
    /* 43 - "minus" */
    LookaheadDFA {
        states: &[Some(59)],
        transitions: &[],
        k: 0,
    },
    /* 44 - "mult" */
    LookaheadDFA {
        states: &[Some(64)],
        transitions: &[],
        k: 0,
    },
    /* 45 - "mult_item" */
    LookaheadDFA {
        states: &[Some(69)],
        transitions: &[],
        k: 0,
    },
    /* 46 - "mult_lst1" */
    LookaheadDFA {
        states: &[None, Some(65), Some(67)],
        transitions: &[
            DFATransition(0, 5, 2),
            DFATransition(0, 6, 2),
            DFATransition(0, 8, 2),
            DFATransition(0, 9, 2),
            DFATransition(0, 10, 2),
            DFATransition(0, 11, 2),
            DFATransition(0, 12, 2),
            DFATransition(0, 13, 2),
            DFATransition(0, 14, 2),
            DFATransition(0, 15, 2),
            DFATransition(0, 17, 1),
            DFATransition(0, 19, 2),
        ],
        k: 1,
    },
    /* 47 - "mult_lst1_itm1" */
    LookaheadDFA {
        states: &[Some(66)],
        transitions: &[],
        k: 0,
    },
    /* 48 - "mult_op" */
    LookaheadDFA {
        states: &[Some(68)],
        transitions: &[],
        k: 0,
    },
    /* 49 - "negate" */
    LookaheadDFA {
        states: &[Some(74)],
        transitions: &[],
        k: 0,
    },
    /* 50 - "number" */
    LookaheadDFA {
        states: &[Some(79)],
        transitions: &[],
        k: 0,
    },
    /* 51 - "plus" */
    LookaheadDFA {
        states: &[Some(58)],
        transitions: &[],
        k: 0,
    },
    /* 52 - "pow_op" */
    LookaheadDFA {
        states: &[Some(63)],
        transitions: &[],
        k: 0,
    },
    /* 53 - "power" */
    LookaheadDFA {
        states: &[Some(70)],
        transitions: &[],
        k: 0,
    },
    /* 54 - "power_lst1" */
    LookaheadDFA {
        states: &[None, Some(71), Some(73)],
        transitions: &[
            DFATransition(0, 5, 2),
            DFATransition(0, 6, 2),
            DFATransition(0, 8, 2),
            DFATransition(0, 9, 2),
            DFATransition(0, 10, 2),
            DFATransition(0, 11, 2),
            DFATransition(0, 12, 2),
            DFATransition(0, 13, 2),
            DFATransition(0, 14, 2),
            DFATransition(0, 15, 2),
            DFATransition(0, 16, 1),
            DFATransition(0, 17, 2),
            DFATransition(0, 19, 2),
        ],
        k: 1,
    },
    /* 55 - "power_lst1_itm1" */
    LookaheadDFA {
        states: &[Some(72)],
        transitions: &[],
        k: 0,
    },
    /* 56 - "relational" */
    LookaheadDFA {
        states: &[Some(43)],
        transitions: &[],
        k: 0,
    },
    /* 57 - "relational_item" */
    LookaheadDFA {
        states: &[Some(48)],
        transitions: &[],
        k: 0,
    },
    /* 58 - "relational_lst1" */
    LookaheadDFA {
        states: &[None, Some(44), Some(46)],
        transitions: &[
            DFATransition(0, 5, 2),
            DFATransition(0, 6, 2),
            DFATransition(0, 8, 2),
            DFATransition(0, 9, 2),
            DFATransition(0, 10, 2),
            DFATransition(0, 11, 2),
            DFATransition(0, 13, 1),
            DFATransition(0, 19, 2),
        ],
        k: 1,
    },
    /* 59 - "relational_lst1_itm1" */
    LookaheadDFA {
        states: &[Some(45)],
        transitions: &[],
        k: 0,
    },
    /* 60 - "relational_op" */
    LookaheadDFA {
        states: &[Some(47)],
        transitions: &[],
        k: 0,
    },
    /* 61 - "summ" */
    LookaheadDFA {
        states: &[Some(54)],
        transitions: &[],
        k: 0,
    },
    /* 62 - "summ_item" */
    LookaheadDFA {
        states: &[Some(62)],
        transitions: &[],
        k: 0,
    },
    /* 63 - "summ_lst1" */
    LookaheadDFA {
        states: &[None, Some(55), Some(57)],
        transitions: &[
            DFATransition(0, 5, 2),
            DFATransition(0, 6, 2),
            DFATransition(0, 8, 2),
            DFATransition(0, 9, 2),
            DFATransition(0, 10, 2),
            DFATransition(0, 11, 2),
            DFATransition(0, 12, 2),
            DFATransition(0, 13, 2),
            DFATransition(0, 14, 1),
            DFATransition(0, 15, 1),
            DFATransition(0, 19, 2),
        ],
        k: 1,
    },
    /* 64 - "summ_lst1_itm1" */
    LookaheadDFA {
        states: &[Some(56)],
        transitions: &[],
        k: 0,
    },
];

pub const PRODUCTIONS: &[Production; 82] = &[
    // 0 - calc: calc_lst1;
    Production {
        lhs: 21,
        production: &[ParseType::N(22)],
    },
    // 1 - calc_lst1: calc_lst1_itm1 calc_lst1;
    Production {
        lhs: 22,
        production: &[ParseType::N(22), ParseType::N(23)],
    },
    // 2 - calc_lst1_itm1: instruction ";";
    Production {
        lhs: 23,
        production: &[ParseType::T(5), ParseType::N(32)],
    },
    // 3 - calc_lst1: ;
    Production {
        lhs: 22,
        production: &[],
    },
    // 4 - instruction: assignment;
    Production {
        lhs: 32,
        production: &[ParseType::N(3)],
    },
    // 5 - instruction: logical_or;
    Production {
        lhs: 32,
        production: &[ParseType::N(38)],
    },
    // 6 - equality_op: "==|!=";
    Production {
        lhs: 28,
        production: &[ParseType::T(6)],
    },
    // 7 - assign_op: "(\+|-|\*|/|%|<<|>>|&|^|\|)?=";
    Production {
        lhs: 2,
        production: &[ParseType::T(7)],
    },
    // 8 - assign_item: id assign_op;
    Production {
        lhs: 1,
        production: &[ParseType::N(2), ParseType::N(30)],
    },
    // 9 - assignment: assign_item assignment_lst1 logical_or;
    Production {
        lhs: 3,
        production: &[ParseType::N(38), ParseType::N(4), ParseType::N(1)],
    },
    // 10 - assignment_lst1: assignment_lst1_itm1 assignment_lst1;
    Production {
        lhs: 4,
        production: &[ParseType::N(4), ParseType::N(5)],
    },
    // 11 - assignment_lst1_itm1: assign_item;
    Production {
        lhs: 5,
        production: &[ParseType::N(1)],
    },
    // 12 - assignment_lst1: ;
    Production {
        lhs: 4,
        production: &[],
    },
    // 13 - logical_or: logical_and logical_or_lst1;
    Production {
        lhs: 38,
        production: &[ParseType::N(40), ParseType::N(33)],
    },
    // 14 - logical_or_lst1: logical_or_lst1_itm1 logical_or_lst1;
    Production {
        lhs: 40,
        production: &[ParseType::N(40), ParseType::N(41)],
    },
    // 15 - logical_or_lst1_itm1: logical_or_item;
    Production {
        lhs: 41,
        production: &[ParseType::N(39)],
    },
    // 16 - logical_or_lst1: ;
    Production {
        lhs: 40,
        production: &[],
    },
    // 17 - logical_or_op: "\|\|";
    Production {
        lhs: 42,
        production: &[ParseType::T(8)],
    },
    // 18 - logical_or_item: logical_or_op logical_and;
    Production {
        lhs: 39,
        production: &[ParseType::N(33), ParseType::N(42)],
    },
    // 19 - logical_and: bitwise_or logical_and_lst1;
    Production {
        lhs: 33,
        production: &[ParseType::N(35), ParseType::N(11)],
    },
    // 20 - logical_and_lst1: logical_and_lst1_itm1 logical_and_lst1;
    Production {
        lhs: 35,
        production: &[ParseType::N(35), ParseType::N(36)],
    },
    // 21 - logical_and_lst1_itm1: logical_and_item;
    Production {
        lhs: 36,
        production: &[ParseType::N(34)],
    },
    // 22 - logical_and_lst1: ;
    Production {
        lhs: 35,
        production: &[],
    },
    // 23 - logical_and_op: "&&";
    Production {
        lhs: 37,
        production: &[ParseType::T(9)],
    },
    // 24 - logical_and_item: logical_and_op bitwise_or;
    Production {
        lhs: 34,
        production: &[ParseType::N(11), ParseType::N(37)],
    },
    // 25 - bitwise_or: bitwise_and bitwise_or_lst1;
    Production {
        lhs: 11,
        production: &[ParseType::N(13), ParseType::N(6)],
    },
    // 26 - bitwise_or_lst1: bitwise_or_lst1_itm1 bitwise_or_lst1;
    Production {
        lhs: 13,
        production: &[ParseType::N(13), ParseType::N(14)],
    },
    // 27 - bitwise_or_lst1_itm1: bitwise_or_item;
    Production {
        lhs: 14,
        production: &[ParseType::N(12)],
    },
    // 28 - bitwise_or_lst1: ;
    Production {
        lhs: 13,
        production: &[],
    },
    // 29 - bitwise_or_op: "\|";
    Production {
        lhs: 15,
        production: &[ParseType::T(10)],
    },
    // 30 - bitwise_or_item: bitwise_or_op bitwise_and;
    Production {
        lhs: 12,
        production: &[ParseType::N(6), ParseType::N(15)],
    },
    // 31 - bitwise_and: equality bitwise_and_lst1;
    Production {
        lhs: 6,
        production: &[ParseType::N(8), ParseType::N(24)],
    },
    // 32 - bitwise_and_lst1: bitwise_and_lst1_itm1 bitwise_and_lst1;
    Production {
        lhs: 8,
        production: &[ParseType::N(8), ParseType::N(9)],
    },
    // 33 - bitwise_and_lst1_itm1: bitwise_and_item;
    Production {
        lhs: 9,
        production: &[ParseType::N(7)],
    },
    // 34 - bitwise_and_lst1: ;
    Production {
        lhs: 8,
        production: &[],
    },
    // 35 - bitwise_and_op: "&";
    Production {
        lhs: 10,
        production: &[ParseType::T(11)],
    },
    // 36 - bitwise_and_item: bitwise_and_op equality;
    Production {
        lhs: 7,
        production: &[ParseType::N(24), ParseType::N(10)],
    },
    // 37 - equality: relational equality_lst1;
    Production {
        lhs: 24,
        production: &[ParseType::N(26), ParseType::N(56)],
    },
    // 38 - equality_lst1: equality_lst1_itm1 equality_lst1;
    Production {
        lhs: 26,
        production: &[ParseType::N(26), ParseType::N(27)],
    },
    // 39 - equality_lst1_itm1: equality_item;
    Production {
        lhs: 27,
        production: &[ParseType::N(25)],
    },
    // 40 - equality_lst1: ;
    Production {
        lhs: 26,
        production: &[],
    },
    // 41 - equality_item: equality_op relational;
    Production {
        lhs: 25,
        production: &[ParseType::N(56), ParseType::N(28)],
    },
    // 42 - bitwise_shift_op: "<<|>>";
    Production {
        lhs: 20,
        production: &[ParseType::T(12)],
    },
    // 43 - relational: bitwise_shift relational_lst1;
    Production {
        lhs: 56,
        production: &[ParseType::N(58), ParseType::N(16)],
    },
    // 44 - relational_lst1: relational_lst1_itm1 relational_lst1;
    Production {
        lhs: 58,
        production: &[ParseType::N(58), ParseType::N(59)],
    },
    // 45 - relational_lst1_itm1: relational_item;
    Production {
        lhs: 59,
        production: &[ParseType::N(57)],
    },
    // 46 - relational_lst1: ;
    Production {
        lhs: 58,
        production: &[],
    },
    // 47 - relational_op: "<=|<|>=|>";
    Production {
        lhs: 60,
        production: &[ParseType::T(13)],
    },
    // 48 - relational_item: relational_op bitwise_shift;
    Production {
        lhs: 57,
        production: &[ParseType::N(16), ParseType::N(60)],
    },
    // 49 - bitwise_shift: summ bitwise_shift_lst1;
    Production {
        lhs: 16,
        production: &[ParseType::N(18), ParseType::N(61)],
    },
    // 50 - bitwise_shift_lst1: bitwise_shift_lst1_itm1 bitwise_shift_lst1;
    Production {
        lhs: 18,
        production: &[ParseType::N(18), ParseType::N(19)],
    },
    // 51 - bitwise_shift_lst1_itm1: bitwise_shift_item;
    Production {
        lhs: 19,
        production: &[ParseType::N(17)],
    },
    // 52 - bitwise_shift_lst1: ;
    Production {
        lhs: 18,
        production: &[],
    },
    // 53 - bitwise_shift_item: bitwise_shift_op summ;
    Production {
        lhs: 17,
        production: &[ParseType::N(61), ParseType::N(20)],
    },
    // 54 - summ: mult summ_lst1;
    Production {
        lhs: 61,
        production: &[ParseType::N(63), ParseType::N(44)],
    },
    // 55 - summ_lst1: summ_lst1_itm1 summ_lst1;
    Production {
        lhs: 63,
        production: &[ParseType::N(63), ParseType::N(64)],
    },
    // 56 - summ_lst1_itm1: summ_item;
    Production {
        lhs: 64,
        production: &[ParseType::N(62)],
    },
    // 57 - summ_lst1: ;
    Production {
        lhs: 63,
        production: &[],
    },
    // 58 - plus: "\+";
    Production {
        lhs: 51,
        production: &[ParseType::T(14)],
    },
    // 59 - minus: "-";
    Production {
        lhs: 43,
        production: &[ParseType::T(15)],
    },
    // 60 - add_op: plus;
    Production {
        lhs: 0,
        production: &[ParseType::N(51)],
    },
    // 61 - add_op: minus;
    Production {
        lhs: 0,
        production: &[ParseType::N(43)],
    },
    // 62 - summ_item: add_op mult;
    Production {
        lhs: 62,
        production: &[ParseType::N(44), ParseType::N(0)],
    },
    // 63 - pow_op: "\*\*";
    Production {
        lhs: 52,
        production: &[ParseType::T(16)],
    },
    // 64 - mult: power mult_lst1;
    Production {
        lhs: 44,
        production: &[ParseType::N(46), ParseType::N(53)],
    },
    // 65 - mult_lst1: mult_lst1_itm1 mult_lst1;
    Production {
        lhs: 46,
        production: &[ParseType::N(46), ParseType::N(47)],
    },
    // 66 - mult_lst1_itm1: mult_item;
    Production {
        lhs: 47,
        production: &[ParseType::N(45)],
    },
    // 67 - mult_lst1: ;
    Production {
        lhs: 46,
        production: &[],
    },
    // 68 - mult_op: "\*|/|%";
    Production {
        lhs: 48,
        production: &[ParseType::T(17)],
    },
    // 69 - mult_item: mult_op power;
    Production {
        lhs: 45,
        production: &[ParseType::N(53), ParseType::N(48)],
    },
    // 70 - power: factor power_lst1;
    Production {
        lhs: 53,
        production: &[ParseType::N(54), ParseType::N(29)],
    },
    // 71 - power_lst1: power_lst1_itm1 power_lst1;
    Production {
        lhs: 54,
        production: &[ParseType::N(54), ParseType::N(55)],
    },
    // 72 - power_lst1_itm1: pow_op factor;
    Production {
        lhs: 55,
        production: &[ParseType::N(29), ParseType::N(52)],
    },
    // 73 - power_lst1: ;
    Production {
        lhs: 54,
        production: &[],
    },
    // 74 - negate: minus;
    Production {
        lhs: 49,
        production: &[ParseType::N(43)],
    },
    // 75 - factor: number;
    Production {
        lhs: 29,
        production: &[ParseType::N(50)],
    },
    // 76 - factor: idref;
    Production {
        lhs: 29,
        production: &[ParseType::N(31)],
    },
    // 77 - factor: negate factor;
    Production {
        lhs: 29,
        production: &[ParseType::N(29), ParseType::N(49)],
    },
    // 78 - factor: "\(" logical_or "\)";
    Production {
        lhs: 29,
        production: &[ParseType::T(19), ParseType::N(38), ParseType::T(18)],
    },
    // 79 - number: "\d+";
    Production {
        lhs: 50,
        production: &[ParseType::T(20)],
    },
    // 80 - idref: id;
    Production {
        lhs: 31,
        production: &[ParseType::N(30)],
    },
    // 81 - id: "[a-zA-Z_]\w*";
    Production {
        lhs: 30,
        production: &[ParseType::T(21)],
    },
];

lazy_static! {
    static ref TOKENIZER: Tokenizer = Tokenizer::build(TERMINALS).unwrap();
}

pub fn parse(
    input: &str,
    file_name: String,
    user_actions: &mut dyn UserActionsTrait,
) -> Result<Tree<ParseTreeType>> {
    let mut llk_parser = LLKParser::new(
        21,
        LOOKAHEAD_AUTOMATA,
        PRODUCTIONS,
        TERMINAL_NAMES,
        NON_TERMINALS,
    );
    let token_stream = RefCell::new(TokenStream::new(input, file_name, &TOKENIZER, MAX_K).unwrap());
    let result = llk_parser.parse(&token_stream, user_actions);
    match result {
        Ok(()) => Ok(llk_parser.parse_tree),
        Err(e) => Err(e),
    }
}
