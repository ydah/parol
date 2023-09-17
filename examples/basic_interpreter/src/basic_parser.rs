// ---------------------------------------------------------
// This file was generated by parol.
// It is not intended for manual editing and changes will be
// lost after next build.
// ---------------------------------------------------------

use parol_runtime::once_cell::sync::Lazy;
#[allow(unused_imports)]
use parol_runtime::parser::{LLKParser, LookaheadDFA, ParseTreeType, ParseType, Production, Trans};
use parol_runtime::{ParolError, ParseTree, TerminalIndex};
use parol_runtime::{TokenStream, Tokenizer};
use std::path::Path;

use crate::basic_grammar::BasicGrammar;
use crate::basic_grammar_trait::BasicGrammarAuto;

use parol_runtime::lexer::tokenizer::{ERROR_TOKEN, UNMATCHABLE_TOKEN, WHITESPACE_TOKEN};

pub const TERMINALS: &[&str; 32] = &[
    /*  0 */ UNMATCHABLE_TOKEN,
    /*  1 */ UNMATCHABLE_TOKEN,
    /*  2 */ UNMATCHABLE_TOKEN,
    /*  3 */ UNMATCHABLE_TOKEN,
    /*  4 */ UNMATCHABLE_TOKEN,
    /*  5 */ r":",
    /*  6 */ r"[0 ]*[1-9] *(?:[0-9] *){1,4}|[0 ]+",
    /*  7 */ r"REM",
    /*  8 */ r",",
    /*  9 */ r"(?:\r?\n|\r)+",
    /* 10 */ r"(?:(?:[0-9] *)+)?\. *(?:(?:[0-9] *)+)? *(?:E *[-+]? *(?:[0-9] *)+)?",
    /* 11 */ r"(?:[0-9] *)+E *[-+]? *(?:[0-9] *)+",
    /* 12 */ r"(?:[0-9] *)+",
    /* 13 */ r"IF",
    /* 14 */ r"THEN",
    /* 15 */ r"GOTO",
    /* 16 */ r"LET",
    /* 17 */ r"PRINT|\?",
    /* 18 */ r"END",
    /* 19 */ r"=",
    /* 20 */ r"N?OR",
    /* 21 */ r"AND",
    /* 22 */ r"NOT",
    /* 23 */ r"<\s*>|<\s*=|<|>\s*=|>|=",
    /* 24 */ r"\+",
    /* 25 */ r"\-",
    /* 26 */ r"\*|\u{2F}",
    /* 27 */ r"\(",
    /* 28 */ r"\)",
    /* 29 */ r"[^\r\n]+",
    /* 30 */ r"[A-Z][0-9A-Z]*",
    /* 31 */ ERROR_TOKEN,
];

pub const TERMINAL_NAMES: &[&str; 32] = &[
    /*  0 */ "EndOfInput",
    /*  1 */ "Newline",
    /*  2 */ "Whitespace",
    /*  3 */ "LineComment",
    /*  4 */ "BlockComment",
    /*  5 */ "Colon",
    /*  6 */ "LineNumber",
    /*  7 */ "REM",
    /*  8 */ "Comma",
    /*  9 */ "EndOfLine",
    /* 10 */ "Float1",
    /* 11 */ "Float2",
    /* 12 */ "Integer",
    /* 13 */ "If",
    /* 14 */ "Then",
    /* 15 */ "Goto",
    /* 16 */ "Let",
    /* 17 */ "Print",
    /* 18 */ "End",
    /* 19 */ "AssignOp",
    /* 20 */ "LogicalOrOp",
    /* 21 */ "LogicalAndOp",
    /* 22 */ "LogicalNotOp",
    /* 23 */ "RelationalOp",
    /* 24 */ "Plus",
    /* 25 */ "Minus",
    /* 26 */ "MulOp",
    /* 27 */ "LParen",
    /* 28 */ "RParen",
    /* 29 */ "Comment",
    /* 30 */ "Variable",
    /* 31 */ "Error",
];

/* SCANNER_0: "INITIAL" */
const SCANNER_0: (&[&str; 5], &[TerminalIndex; 13]) = (
    &[
        /*  0 */ UNMATCHABLE_TOKEN,
        /*  1 */ UNMATCHABLE_TOKEN,
        /*  2 */ WHITESPACE_TOKEN,
        /*  3 */ UNMATCHABLE_TOKEN,
        /*  4 */ UNMATCHABLE_TOKEN,
    ],
    &[
        5,  /* Colon */
        6,  /* LineNumber */
        7,  /* REM */
        8,  /* Comma */
        9,  /* EndOfLine */
        13, /* If */
        14, /* Then */
        15, /* Goto */
        16, /* Let */
        17, /* Print */
        18, /* End */
        19, /* AssignOp */
        30, /* Variable */
    ],
);

/* SCANNER_1: "Cmnt" */
const SCANNER_1: (&[&str; 5], &[TerminalIndex; 1]) = (
    &[
        /*  0 */ UNMATCHABLE_TOKEN,
        /*  1 */ UNMATCHABLE_TOKEN,
        /*  2 */ WHITESPACE_TOKEN,
        /*  3 */ UNMATCHABLE_TOKEN,
        /*  4 */ UNMATCHABLE_TOKEN,
    ],
    &[29 /* Comment */],
);

/* SCANNER_2: "Expr" */
const SCANNER_2: (&[&str; 5], &[TerminalIndex; 18]) = (
    &[
        /*  0 */ UNMATCHABLE_TOKEN,
        /*  1 */ UNMATCHABLE_TOKEN,
        /*  2 */ WHITESPACE_TOKEN,
        /*  3 */ UNMATCHABLE_TOKEN,
        /*  4 */ UNMATCHABLE_TOKEN,
    ],
    &[
        5,  /* Colon */
        8,  /* Comma */
        9,  /* EndOfLine */
        10, /* Float1 */
        11, /* Float2 */
        12, /* Integer */
        14, /* Then */
        15, /* Goto */
        20, /* LogicalOrOp */
        21, /* LogicalAndOp */
        22, /* LogicalNotOp */
        23, /* RelationalOp */
        24, /* Plus */
        25, /* Minus */
        26, /* MulOp */
        27, /* LParen */
        28, /* RParen */
        30, /* Variable */
    ],
);

const MAX_K: usize = 2;

pub const NON_TERMINALS: &[&str; 58] = &[
    /*  0 */ "AssignOp",
    /*  1 */ "Assignment",
    /*  2 */ "AssignmentOpt",
    /*  3 */ "Basic",
    /*  4 */ "BasicList",
    /*  5 */ "BasicOpt",
    /*  6 */ "BasicOpt0",
    /*  7 */ "Comment",
    /*  8 */ "End",
    /*  9 */ "EndOfLine",
    /* 10 */ "EndStatement",
    /* 11 */ "Expression",
    /* 12 */ "Factor",
    /* 13 */ "Float",
    /* 14 */ "Float1",
    /* 15 */ "Float2",
    /* 16 */ "Goto",
    /* 17 */ "GotoStatement",
    /* 18 */ "If",
    /* 19 */ "IfBody",
    /* 20 */ "IfStatement",
    /* 21 */ "Integer",
    /* 22 */ "LParen",
    /* 23 */ "Let",
    /* 24 */ "Line",
    /* 25 */ "LineList",
    /* 26 */ "LineNumber",
    /* 27 */ "Literal",
    /* 28 */ "LogicalAnd",
    /* 29 */ "LogicalAndList",
    /* 30 */ "LogicalAndOp",
    /* 31 */ "LogicalNot",
    /* 32 */ "LogicalNotOp",
    /* 33 */ "LogicalNotOpt",
    /* 34 */ "LogicalOr",
    /* 35 */ "LogicalOrList",
    /* 36 */ "LogicalOrOp",
    /* 37 */ "Minus",
    /* 38 */ "MulOp",
    /* 39 */ "Multiplication",
    /* 40 */ "MultiplicationList",
    /* 41 */ "Number",
    /* 42 */ "Plus",
    /* 43 */ "Print",
    /* 44 */ "PrintStatement",
    /* 45 */ "PrintStatementList",
    /* 46 */ "RParen",
    /* 47 */ "Relational",
    /* 48 */ "RelationalList",
    /* 49 */ "RelationalOp",
    /* 50 */ "Remark",
    /* 51 */ "RemarkOpt",
    /* 52 */ "Statement",
    /* 53 */ "Summation",
    /* 54 */ "SummationList",
    /* 55 */ "SummationListGroup",
    /* 56 */ "Then",
    /* 57 */ "Variable",
];

pub const LOOKAHEAD_AUTOMATA: &[LookaheadDFA; 58] = &[
    /* 0 - "AssignOp" */
    LookaheadDFA {
        prod0: 46,
        transitions: &[],
        k: 0,
    },
    /* 1 - "Assignment" */
    LookaheadDFA {
        prod0: 22,
        transitions: &[],
        k: 0,
    },
    /* 2 - "AssignmentOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 16, 1, 23), Trans(0, 30, 2, 24)],
        k: 1,
    },
    /* 3 - "Basic" */
    LookaheadDFA {
        prod0: 0,
        transitions: &[],
        k: 0,
    },
    /* 4 - "BasicList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 0, 3, 2),
            Trans(0, 9, 1, -1),
            Trans(1, 0, 3, 2),
            Trans(1, 6, 2, 1),
        ],
        k: 2,
    },
    /* 5 - "BasicOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 6, 2, 6), Trans(0, 9, 1, 5)],
        k: 1,
    },
    /* 6 - "BasicOpt0" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 0, 2, 4), Trans(0, 9, 1, 3)],
        k: 1,
    },
    /* 7 - "Comment" */
    LookaheadDFA {
        prod0: 56,
        transitions: &[],
        k: 0,
    },
    /* 8 - "End" */
    LookaheadDFA {
        prod0: 45,
        transitions: &[],
        k: 0,
    },
    /* 9 - "EndOfLine" */
    LookaheadDFA {
        prod0: 31,
        transitions: &[],
        k: 0,
    },
    /* 10 - "EndStatement" */
    LookaheadDFA {
        prod0: 30,
        transitions: &[],
        k: 0,
    },
    /* 11 - "Expression" */
    LookaheadDFA {
        prod0: 58,
        transitions: &[],
        k: 0,
    },
    /* 12 - "Factor" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 10, 1, 79),
            Trans(0, 11, 1, 79),
            Trans(0, 12, 1, 79),
            Trans(0, 25, 3, 81),
            Trans(0, 27, 4, 82),
            Trans(0, 30, 2, 80),
        ],
        k: 1,
    },
    /* 13 - "Float" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 10, 1, 35), Trans(0, 11, 2, 36)],
        k: 1,
    },
    /* 14 - "Float1" */
    LookaheadDFA {
        prod0: 37,
        transitions: &[],
        k: 0,
    },
    /* 15 - "Float2" */
    LookaheadDFA {
        prod0: 38,
        transitions: &[],
        k: 0,
    },
    /* 16 - "Goto" */
    LookaheadDFA {
        prod0: 42,
        transitions: &[],
        k: 0,
    },
    /* 17 - "GotoStatement" */
    LookaheadDFA {
        prod0: 20,
        transitions: &[],
        k: 0,
    },
    /* 18 - "If" */
    LookaheadDFA {
        prod0: 40,
        transitions: &[],
        k: 0,
    },
    /* 19 - "IfBody" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 14, 1, 25), Trans(0, 15, 2, 26)],
        k: 1,
    },
    /* 20 - "IfStatement" */
    LookaheadDFA {
        prod0: 21,
        transitions: &[],
        k: 0,
    },
    /* 21 - "Integer" */
    LookaheadDFA {
        prod0: 39,
        transitions: &[],
        k: 0,
    },
    /* 22 - "LParen" */
    LookaheadDFA {
        prod0: 54,
        transitions: &[],
        k: 0,
    },
    /* 23 - "Let" */
    LookaheadDFA {
        prod0: 43,
        transitions: &[],
        k: 0,
    },
    /* 24 - "Line" */
    LookaheadDFA {
        prod0: 7,
        transitions: &[],
        k: 0,
    },
    /* 25 - "LineList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 0, 2, 9), Trans(0, 5, 1, 8), Trans(0, 9, 2, 9)],
        k: 1,
    },
    /* 26 - "LineNumber" */
    LookaheadDFA {
        prod0: 10,
        transitions: &[],
        k: 0,
    },
    /* 27 - "Literal" */
    LookaheadDFA {
        prod0: 32,
        transitions: &[],
        k: 0,
    },
    /* 28 - "LogicalAnd" */
    LookaheadDFA {
        prod0: 62,
        transitions: &[],
        k: 0,
    },
    /* 29 - "LogicalAndList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 0, 2, 64),
            Trans(0, 5, 2, 64),
            Trans(0, 8, 2, 64),
            Trans(0, 9, 2, 64),
            Trans(0, 14, 2, 64),
            Trans(0, 15, 2, 64),
            Trans(0, 20, 2, 64),
            Trans(0, 21, 1, 63),
            Trans(0, 28, 2, 64),
        ],
        k: 1,
    },
    /* 30 - "LogicalAndOp" */
    LookaheadDFA {
        prod0: 48,
        transitions: &[],
        k: 0,
    },
    /* 31 - "LogicalNot" */
    LookaheadDFA {
        prod0: 65,
        transitions: &[],
        k: 0,
    },
    /* 32 - "LogicalNotOp" */
    LookaheadDFA {
        prod0: 49,
        transitions: &[],
        k: 0,
    },
    /* 33 - "LogicalNotOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 10, 2, 67),
            Trans(0, 11, 2, 67),
            Trans(0, 12, 2, 67),
            Trans(0, 22, 1, 66),
            Trans(0, 25, 2, 67),
            Trans(0, 27, 2, 67),
            Trans(0, 30, 2, 67),
        ],
        k: 1,
    },
    /* 34 - "LogicalOr" */
    LookaheadDFA {
        prod0: 59,
        transitions: &[],
        k: 0,
    },
    /* 35 - "LogicalOrList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 0, 2, 61),
            Trans(0, 5, 2, 61),
            Trans(0, 8, 2, 61),
            Trans(0, 9, 2, 61),
            Trans(0, 14, 2, 61),
            Trans(0, 15, 2, 61),
            Trans(0, 20, 1, 60),
            Trans(0, 28, 2, 61),
        ],
        k: 1,
    },
    /* 36 - "LogicalOrOp" */
    LookaheadDFA {
        prod0: 47,
        transitions: &[],
        k: 0,
    },
    /* 37 - "Minus" */
    LookaheadDFA {
        prod0: 52,
        transitions: &[],
        k: 0,
    },
    /* 38 - "MulOp" */
    LookaheadDFA {
        prod0: 53,
        transitions: &[],
        k: 0,
    },
    /* 39 - "Multiplication" */
    LookaheadDFA {
        prod0: 76,
        transitions: &[],
        k: 0,
    },
    /* 40 - "MultiplicationList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 0, 2, 78),
            Trans(0, 5, 2, 78),
            Trans(0, 8, 2, 78),
            Trans(0, 9, 2, 78),
            Trans(0, 14, 2, 78),
            Trans(0, 15, 2, 78),
            Trans(0, 20, 2, 78),
            Trans(0, 21, 2, 78),
            Trans(0, 23, 2, 78),
            Trans(0, 24, 2, 78),
            Trans(0, 25, 2, 78),
            Trans(0, 26, 1, 77),
            Trans(0, 28, 2, 78),
        ],
        k: 1,
    },
    /* 41 - "Number" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 10, 1, 33),
            Trans(0, 11, 1, 33),
            Trans(0, 12, 2, 34),
        ],
        k: 1,
    },
    /* 42 - "Plus" */
    LookaheadDFA {
        prod0: 51,
        transitions: &[],
        k: 0,
    },
    /* 43 - "Print" */
    LookaheadDFA {
        prod0: 44,
        transitions: &[],
        k: 0,
    },
    /* 44 - "PrintStatement" */
    LookaheadDFA {
        prod0: 27,
        transitions: &[],
        k: 0,
    },
    /* 45 - "PrintStatementList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 0, 2, 29),
            Trans(0, 5, 2, 29),
            Trans(0, 8, 1, 28),
            Trans(0, 9, 2, 29),
        ],
        k: 1,
    },
    /* 46 - "RParen" */
    LookaheadDFA {
        prod0: 55,
        transitions: &[],
        k: 0,
    },
    /* 47 - "Relational" */
    LookaheadDFA {
        prod0: 68,
        transitions: &[],
        k: 0,
    },
    /* 48 - "RelationalList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 0, 2, 70),
            Trans(0, 5, 2, 70),
            Trans(0, 8, 2, 70),
            Trans(0, 9, 2, 70),
            Trans(0, 14, 2, 70),
            Trans(0, 15, 2, 70),
            Trans(0, 20, 2, 70),
            Trans(0, 21, 2, 70),
            Trans(0, 23, 1, 69),
            Trans(0, 28, 2, 70),
        ],
        k: 1,
    },
    /* 49 - "RelationalOp" */
    LookaheadDFA {
        prod0: 50,
        transitions: &[],
        k: 0,
    },
    /* 50 - "Remark" */
    LookaheadDFA {
        prod0: 17,
        transitions: &[],
        k: 0,
    },
    /* 51 - "RemarkOpt" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 0, 2, 19),
            Trans(0, 5, 2, 19),
            Trans(0, 9, 2, 19),
            Trans(0, 29, 1, 18),
        ],
        k: 1,
    },
    /* 52 - "Statement" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 7, 1, 11),
            Trans(0, 13, 3, 13),
            Trans(0, 15, 2, 12),
            Trans(0, 16, 4, 14),
            Trans(0, 17, 5, 15),
            Trans(0, 18, 6, 16),
            Trans(0, 30, 4, 14),
        ],
        k: 1,
    },
    /* 53 - "Summation" */
    LookaheadDFA {
        prod0: 71,
        transitions: &[],
        k: 0,
    },
    /* 54 - "SummationList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 0, 2, 75),
            Trans(0, 5, 2, 75),
            Trans(0, 8, 2, 75),
            Trans(0, 9, 2, 75),
            Trans(0, 14, 2, 75),
            Trans(0, 15, 2, 75),
            Trans(0, 20, 2, 75),
            Trans(0, 21, 2, 75),
            Trans(0, 23, 2, 75),
            Trans(0, 24, 1, 72),
            Trans(0, 25, 1, 72),
            Trans(0, 28, 2, 75),
        ],
        k: 1,
    },
    /* 55 - "SummationListGroup" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 24, 1, 73), Trans(0, 25, 2, 74)],
        k: 1,
    },
    /* 56 - "Then" */
    LookaheadDFA {
        prod0: 41,
        transitions: &[],
        k: 0,
    },
    /* 57 - "Variable" */
    LookaheadDFA {
        prod0: 57,
        transitions: &[],
        k: 0,
    },
];

pub const PRODUCTIONS: &[Production; 83] = &[
    // 0 - Basic: BasicOpt /* Option */ Line BasicList /* Vec */ BasicOpt0 /* Option */;
    Production {
        lhs: 3,
        production: &[
            ParseType::N(6),
            ParseType::N(4),
            ParseType::N(24),
            ParseType::N(5),
        ],
    },
    // 1 - BasicList: EndOfLine Line BasicList;
    Production {
        lhs: 4,
        production: &[ParseType::N(4), ParseType::N(24), ParseType::N(9)],
    },
    // 2 - BasicList: ;
    Production {
        lhs: 4,
        production: &[],
    },
    // 3 - BasicOpt0: EndOfLine;
    Production {
        lhs: 6,
        production: &[ParseType::N(9)],
    },
    // 4 - BasicOpt0: ;
    Production {
        lhs: 6,
        production: &[],
    },
    // 5 - BasicOpt: EndOfLine;
    Production {
        lhs: 5,
        production: &[ParseType::N(9)],
    },
    // 6 - BasicOpt: ;
    Production {
        lhs: 5,
        production: &[],
    },
    // 7 - Line: LineNumber Statement LineList /* Vec */;
    Production {
        lhs: 24,
        production: &[ParseType::N(25), ParseType::N(52), ParseType::N(26)],
    },
    // 8 - LineList: ':'^ /* Clipped */ Statement LineList;
    Production {
        lhs: 25,
        production: &[ParseType::N(25), ParseType::N(52), ParseType::T(5)],
    },
    // 9 - LineList: ;
    Production {
        lhs: 25,
        production: &[],
    },
    // 10 - LineNumber: /[0 ]*[1-9] *(?:[0-9] *){1,4}|[0 ]+/;
    Production {
        lhs: 26,
        production: &[ParseType::T(6)],
    },
    // 11 - Statement: Remark;
    Production {
        lhs: 52,
        production: &[ParseType::N(50)],
    },
    // 12 - Statement: GotoStatement;
    Production {
        lhs: 52,
        production: &[ParseType::N(17)],
    },
    // 13 - Statement: IfStatement;
    Production {
        lhs: 52,
        production: &[ParseType::N(20)],
    },
    // 14 - Statement: Assignment;
    Production {
        lhs: 52,
        production: &[ParseType::N(1)],
    },
    // 15 - Statement: PrintStatement;
    Production {
        lhs: 52,
        production: &[ParseType::N(44)],
    },
    // 16 - Statement: EndStatement;
    Production {
        lhs: 52,
        production: &[ParseType::N(10)],
    },
    // 17 - Remark: 'REM'^ /* Clipped */ Push(1) RemarkOpt /* Option */ Pop;
    Production {
        lhs: 50,
        production: &[
            ParseType::Pop,
            ParseType::N(51),
            ParseType::Push(1),
            ParseType::T(7),
        ],
    },
    // 18 - RemarkOpt: Comment;
    Production {
        lhs: 51,
        production: &[ParseType::N(7)],
    },
    // 19 - RemarkOpt: ;
    Production {
        lhs: 51,
        production: &[],
    },
    // 20 - GotoStatement: Goto LineNumber;
    Production {
        lhs: 17,
        production: &[ParseType::N(26), ParseType::N(16)],
    },
    // 21 - IfStatement: If Push(2) Expression Pop IfBody;
    Production {
        lhs: 20,
        production: &[
            ParseType::N(19),
            ParseType::Pop,
            ParseType::N(11),
            ParseType::Push(2),
            ParseType::N(18),
        ],
    },
    // 22 - Assignment: AssignmentOpt /* Option */ Variable AssignOp Push(2) Expression Pop;
    Production {
        lhs: 1,
        production: &[
            ParseType::Pop,
            ParseType::N(11),
            ParseType::Push(2),
            ParseType::N(0),
            ParseType::N(57),
            ParseType::N(2),
        ],
    },
    // 23 - AssignmentOpt: Let;
    Production {
        lhs: 2,
        production: &[ParseType::N(23)],
    },
    // 24 - AssignmentOpt: ;
    Production {
        lhs: 2,
        production: &[],
    },
    // 25 - IfBody: Then Statement;
    Production {
        lhs: 19,
        production: &[ParseType::N(52), ParseType::N(56)],
    },
    // 26 - IfBody: Goto LineNumber;
    Production {
        lhs: 19,
        production: &[ParseType::N(26), ParseType::N(16)],
    },
    // 27 - PrintStatement: Print Push(2) Expression PrintStatementList /* Vec */ Pop;
    Production {
        lhs: 44,
        production: &[
            ParseType::Pop,
            ParseType::N(45),
            ParseType::N(11),
            ParseType::Push(2),
            ParseType::N(43),
        ],
    },
    // 28 - PrintStatementList: ','^ /* Clipped */ Expression PrintStatementList;
    Production {
        lhs: 45,
        production: &[ParseType::N(45), ParseType::N(11), ParseType::T(8)],
    },
    // 29 - PrintStatementList: ;
    Production {
        lhs: 45,
        production: &[],
    },
    // 30 - EndStatement: End;
    Production {
        lhs: 10,
        production: &[ParseType::N(8)],
    },
    // 31 - EndOfLine: /(?:\r?\n|\r)+/^ /* Clipped */;
    Production {
        lhs: 9,
        production: &[ParseType::T(9)],
    },
    // 32 - Literal: Number;
    Production {
        lhs: 27,
        production: &[ParseType::N(41)],
    },
    // 33 - Number: Float;
    Production {
        lhs: 41,
        production: &[ParseType::N(13)],
    },
    // 34 - Number: Integer;
    Production {
        lhs: 41,
        production: &[ParseType::N(21)],
    },
    // 35 - Float: Float1;
    Production {
        lhs: 13,
        production: &[ParseType::N(14)],
    },
    // 36 - Float: Float2;
    Production {
        lhs: 13,
        production: &[ParseType::N(15)],
    },
    // 37 - Float1: /(?:(?:[0-9] *)+)?\. *(?:(?:[0-9] *)+)? *(?:E *[-+]? *(?:[0-9] *)+)?/;
    Production {
        lhs: 14,
        production: &[ParseType::T(10)],
    },
    // 38 - Float2: /(?:[0-9] *)+E *[-+]? *(?:[0-9] *)+/;
    Production {
        lhs: 15,
        production: &[ParseType::T(11)],
    },
    // 39 - Integer: /(?:[0-9] *)+/;
    Production {
        lhs: 21,
        production: &[ParseType::T(12)],
    },
    // 40 - If: 'IF'^ /* Clipped */;
    Production {
        lhs: 18,
        production: &[ParseType::T(13)],
    },
    // 41 - Then: 'THEN'^ /* Clipped */;
    Production {
        lhs: 56,
        production: &[ParseType::T(14)],
    },
    // 42 - Goto: 'GOTO'^ /* Clipped */;
    Production {
        lhs: 16,
        production: &[ParseType::T(15)],
    },
    // 43 - Let: 'LET'^ /* Clipped */;
    Production {
        lhs: 23,
        production: &[ParseType::T(16)],
    },
    // 44 - Print: /PRINT|\?/^ /* Clipped */;
    Production {
        lhs: 43,
        production: &[ParseType::T(17)],
    },
    // 45 - End: 'END'^ /* Clipped */;
    Production {
        lhs: 8,
        production: &[ParseType::T(18)],
    },
    // 46 - AssignOp: '='^ /* Clipped */;
    Production {
        lhs: 0,
        production: &[ParseType::T(19)],
    },
    // 47 - LogicalOrOp: /N?OR/;
    Production {
        lhs: 36,
        production: &[ParseType::T(20)],
    },
    // 48 - LogicalAndOp: 'AND';
    Production {
        lhs: 30,
        production: &[ParseType::T(21)],
    },
    // 49 - LogicalNotOp: 'NOT';
    Production {
        lhs: 32,
        production: &[ParseType::T(22)],
    },
    // 50 - RelationalOp: /<\s*>|<\s*=|<|>\s*=|>|=/;
    Production {
        lhs: 49,
        production: &[ParseType::T(23)],
    },
    // 51 - Plus: '+';
    Production {
        lhs: 42,
        production: &[ParseType::T(24)],
    },
    // 52 - Minus: '-';
    Production {
        lhs: 37,
        production: &[ParseType::T(25)],
    },
    // 53 - MulOp: /\*|\u{2F}/;
    Production {
        lhs: 38,
        production: &[ParseType::T(26)],
    },
    // 54 - LParen: '(';
    Production {
        lhs: 22,
        production: &[ParseType::T(27)],
    },
    // 55 - RParen: ')';
    Production {
        lhs: 46,
        production: &[ParseType::T(28)],
    },
    // 56 - Comment: /[^\r\n]+/;
    Production {
        lhs: 7,
        production: &[ParseType::T(29)],
    },
    // 57 - Variable: /[A-Z][0-9A-Z]*/;
    Production {
        lhs: 57,
        production: &[ParseType::T(30)],
    },
    // 58 - Expression: LogicalOr;
    Production {
        lhs: 11,
        production: &[ParseType::N(34)],
    },
    // 59 - LogicalOr: LogicalAnd LogicalOrList /* Vec */;
    Production {
        lhs: 34,
        production: &[ParseType::N(35), ParseType::N(28)],
    },
    // 60 - LogicalOrList: LogicalOrOp LogicalAnd LogicalOrList;
    Production {
        lhs: 35,
        production: &[ParseType::N(35), ParseType::N(28), ParseType::N(36)],
    },
    // 61 - LogicalOrList: ;
    Production {
        lhs: 35,
        production: &[],
    },
    // 62 - LogicalAnd: LogicalNot LogicalAndList /* Vec */;
    Production {
        lhs: 28,
        production: &[ParseType::N(29), ParseType::N(31)],
    },
    // 63 - LogicalAndList: LogicalAndOp LogicalNot LogicalAndList;
    Production {
        lhs: 29,
        production: &[ParseType::N(29), ParseType::N(31), ParseType::N(30)],
    },
    // 64 - LogicalAndList: ;
    Production {
        lhs: 29,
        production: &[],
    },
    // 65 - LogicalNot: LogicalNotOpt /* Option */ Relational;
    Production {
        lhs: 31,
        production: &[ParseType::N(47), ParseType::N(33)],
    },
    // 66 - LogicalNotOpt: LogicalNotOp;
    Production {
        lhs: 33,
        production: &[ParseType::N(32)],
    },
    // 67 - LogicalNotOpt: ;
    Production {
        lhs: 33,
        production: &[],
    },
    // 68 - Relational: Summation RelationalList /* Vec */;
    Production {
        lhs: 47,
        production: &[ParseType::N(48), ParseType::N(53)],
    },
    // 69 - RelationalList: RelationalOp Summation RelationalList;
    Production {
        lhs: 48,
        production: &[ParseType::N(48), ParseType::N(53), ParseType::N(49)],
    },
    // 70 - RelationalList: ;
    Production {
        lhs: 48,
        production: &[],
    },
    // 71 - Summation: Multiplication SummationList /* Vec */;
    Production {
        lhs: 53,
        production: &[ParseType::N(54), ParseType::N(39)],
    },
    // 72 - SummationList: SummationListGroup Multiplication SummationList;
    Production {
        lhs: 54,
        production: &[ParseType::N(54), ParseType::N(39), ParseType::N(55)],
    },
    // 73 - SummationListGroup: Plus;
    Production {
        lhs: 55,
        production: &[ParseType::N(42)],
    },
    // 74 - SummationListGroup: Minus;
    Production {
        lhs: 55,
        production: &[ParseType::N(37)],
    },
    // 75 - SummationList: ;
    Production {
        lhs: 54,
        production: &[],
    },
    // 76 - Multiplication: Factor MultiplicationList /* Vec */;
    Production {
        lhs: 39,
        production: &[ParseType::N(40), ParseType::N(12)],
    },
    // 77 - MultiplicationList: MulOp Factor MultiplicationList;
    Production {
        lhs: 40,
        production: &[ParseType::N(40), ParseType::N(12), ParseType::N(38)],
    },
    // 78 - MultiplicationList: ;
    Production {
        lhs: 40,
        production: &[],
    },
    // 79 - Factor: Literal;
    Production {
        lhs: 12,
        production: &[ParseType::N(27)],
    },
    // 80 - Factor: Variable;
    Production {
        lhs: 12,
        production: &[ParseType::N(57)],
    },
    // 81 - Factor: Minus Factor;
    Production {
        lhs: 12,
        production: &[ParseType::N(12), ParseType::N(37)],
    },
    // 82 - Factor: LParen Expression RParen;
    Production {
        lhs: 12,
        production: &[ParseType::N(46), ParseType::N(11), ParseType::N(22)],
    },
];

static TOKENIZERS: Lazy<Vec<(&'static str, Tokenizer)>> = Lazy::new(|| {
    vec![
        (
            "INITIAL",
            Tokenizer::build(TERMINALS, SCANNER_0.0, SCANNER_0.1).unwrap(),
        ),
        (
            "Cmnt",
            Tokenizer::build(TERMINALS, SCANNER_1.0, SCANNER_1.1).unwrap(),
        ),
        (
            "Expr",
            Tokenizer::build(TERMINALS, SCANNER_2.0, SCANNER_2.1).unwrap(),
        ),
    ]
});

pub fn parse<'t, T>(
    input: &'t str,
    file_name: T,
    user_actions: &mut BasicGrammar<'t>,
) -> Result<ParseTree<'t>, ParolError>
where
    T: AsRef<Path>,
{
    let mut llk_parser = LLKParser::new(
        3,
        LOOKAHEAD_AUTOMATA,
        PRODUCTIONS,
        TERMINAL_NAMES,
        NON_TERMINALS,
    ); // Initialize wrapper
    let mut user_actions = BasicGrammarAuto::new(user_actions);

    llk_parser.parse(
        TokenStream::new(input, file_name, &TOKENIZERS, MAX_K).unwrap(),
        &mut user_actions,
    )
}
