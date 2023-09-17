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

use crate::json_grammar::JsonGrammar;
use crate::json_grammar_trait::JsonGrammarAuto;

use parol_runtime::lexer::tokenizer::{
    ERROR_TOKEN, NEW_LINE_TOKEN, UNMATCHABLE_TOKEN, WHITESPACE_TOKEN,
};

pub const TERMINALS: &[&str; 17] = &[
    /*  0 */ UNMATCHABLE_TOKEN,
    /*  1 */ UNMATCHABLE_TOKEN,
    /*  2 */ UNMATCHABLE_TOKEN,
    /*  3 */ UNMATCHABLE_TOKEN,
    /*  4 */ UNMATCHABLE_TOKEN,
    /*  5 */ r"\{",
    /*  6 */ r"\}",
    /*  7 */ r",",
    /*  8 */ r":",
    /*  9 */ r"\[",
    /* 10 */ r"\]",
    /* 11 */ r"true",
    /* 12 */ r"false",
    /* 13 */ r"null",
    /* 14 */
    r"\u{0022}(?:\\[\u{0022}\\/bfnrt]|u[0-9a-fA-F]{4}|[^\u{0022}\\\u0000-\u001F])*\u{0022}",
    /* 15 */ r"-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][-+]?(?:0|[1-9][0-9]*)?)?",
    /* 16 */ ERROR_TOKEN,
];

pub const TERMINAL_NAMES: &[&str; 17] = &[
    /*  0 */ "EndOfInput",
    /*  1 */ "Newline",
    /*  2 */ "Whitespace",
    /*  3 */ "LineComment",
    /*  4 */ "BlockComment",
    /*  5 */ "LBrace",
    /*  6 */ "RBrace",
    /*  7 */ "Comma",
    /*  8 */ "Colon",
    /*  9 */ "LBracket",
    /* 10 */ "RBracket",
    /* 11 */ "True",
    /* 12 */ "False",
    /* 13 */ "Null",
    /* 14 */ "String",
    /* 15 */ "Number",
    /* 16 */ "Error",
];

/* SCANNER_0: "INITIAL" */
const SCANNER_0: (&[&str; 5], &[TerminalIndex; 11]) = (
    &[
        /*  0 */ UNMATCHABLE_TOKEN,
        /*  1 */ NEW_LINE_TOKEN,
        /*  2 */ WHITESPACE_TOKEN,
        /*  3 */ UNMATCHABLE_TOKEN,
        /*  4 */ UNMATCHABLE_TOKEN,
    ],
    &[
        5,  /* LBrace */
        6,  /* RBrace */
        7,  /* Comma */
        8,  /* Colon */
        9,  /* LBracket */
        10, /* RBracket */
        11, /* True */
        12, /* False */
        13, /* Null */
        14, /* String */
        15, /* Number */
    ],
);

const MAX_K: usize = 1;

pub const NON_TERMINALS: &[&str; 11] = &[
    /*  0 */ "Array",
    /*  1 */ "ArrayList",
    /*  2 */ "ArraySuffix",
    /*  3 */ "Json",
    /*  4 */ "Number",
    /*  5 */ "Object",
    /*  6 */ "ObjectList",
    /*  7 */ "ObjectSuffix",
    /*  8 */ "Pair",
    /*  9 */ "String",
    /* 10 */ "Value",
];

pub const LOOKAHEAD_AUTOMATA: &[LookaheadDFA; 11] = &[
    /* 0 - "Array" */
    LookaheadDFA {
        prod0: 7,
        transitions: &[],
        k: 0,
    },
    /* 1 - "ArrayList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 7, 1, 10), Trans(0, 10, 2, 11)],
        k: 1,
    },
    /* 2 - "ArraySuffix" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 5, 1, 8),
            Trans(0, 9, 1, 8),
            Trans(0, 10, 2, 9),
            Trans(0, 11, 1, 8),
            Trans(0, 12, 1, 8),
            Trans(0, 13, 1, 8),
            Trans(0, 14, 1, 8),
            Trans(0, 15, 1, 8),
        ],
        k: 1,
    },
    /* 3 - "Json" */
    LookaheadDFA {
        prod0: 0,
        transitions: &[],
        k: 0,
    },
    /* 4 - "Number" */
    LookaheadDFA {
        prod0: 20,
        transitions: &[],
        k: 0,
    },
    /* 5 - "Object" */
    LookaheadDFA {
        prod0: 1,
        transitions: &[],
        k: 0,
    },
    /* 6 - "ObjectList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 6, 2, 5), Trans(0, 7, 1, 4)],
        k: 1,
    },
    /* 7 - "ObjectSuffix" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 6, 2, 3), Trans(0, 14, 1, 2)],
        k: 1,
    },
    /* 8 - "Pair" */
    LookaheadDFA {
        prod0: 6,
        transitions: &[],
        k: 0,
    },
    /* 9 - "String" */
    LookaheadDFA {
        prod0: 19,
        transitions: &[],
        k: 0,
    },
    /* 10 - "Value" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 5, 3, 14),
            Trans(0, 9, 4, 15),
            Trans(0, 11, 5, 16),
            Trans(0, 12, 6, 17),
            Trans(0, 13, 7, 18),
            Trans(0, 14, 1, 12),
            Trans(0, 15, 2, 13),
        ],
        k: 1,
    },
];

pub const PRODUCTIONS: &[Production; 21] = &[
    // 0 - Json: Value;
    Production {
        lhs: 3,
        production: &[ParseType::N(10)],
    },
    // 1 - Object: "\{"^ /* Clipped */ ObjectSuffix;
    Production {
        lhs: 5,
        production: &[ParseType::N(7), ParseType::T(5)],
    },
    // 2 - ObjectSuffix: Pair ObjectList /* Vec */ "\}"^ /* Clipped */;
    Production {
        lhs: 7,
        production: &[ParseType::T(6), ParseType::N(6), ParseType::N(8)],
    },
    // 3 - ObjectSuffix: "\}"^ /* Clipped */;
    Production {
        lhs: 7,
        production: &[ParseType::T(6)],
    },
    // 4 - ObjectList: ","^ /* Clipped */ Pair ObjectList;
    Production {
        lhs: 6,
        production: &[ParseType::N(6), ParseType::N(8), ParseType::T(7)],
    },
    // 5 - ObjectList: ;
    Production {
        lhs: 6,
        production: &[],
    },
    // 6 - Pair: String ":"^ /* Clipped */ Value;
    Production {
        lhs: 8,
        production: &[ParseType::N(10), ParseType::T(8), ParseType::N(9)],
    },
    // 7 - Array: "\["^ /* Clipped */ ArraySuffix;
    Production {
        lhs: 0,
        production: &[ParseType::N(2), ParseType::T(9)],
    },
    // 8 - ArraySuffix: Value ArrayList /* Vec */ "\]"^ /* Clipped */;
    Production {
        lhs: 2,
        production: &[ParseType::T(10), ParseType::N(1), ParseType::N(10)],
    },
    // 9 - ArraySuffix: "\]"^ /* Clipped */;
    Production {
        lhs: 2,
        production: &[ParseType::T(10)],
    },
    // 10 - ArrayList: ","^ /* Clipped */ Value ArrayList;
    Production {
        lhs: 1,
        production: &[ParseType::N(1), ParseType::N(10), ParseType::T(7)],
    },
    // 11 - ArrayList: ;
    Production {
        lhs: 1,
        production: &[],
    },
    // 12 - Value: String;
    Production {
        lhs: 10,
        production: &[ParseType::N(9)],
    },
    // 13 - Value: Number;
    Production {
        lhs: 10,
        production: &[ParseType::N(4)],
    },
    // 14 - Value: Object;
    Production {
        lhs: 10,
        production: &[ParseType::N(5)],
    },
    // 15 - Value: Array;
    Production {
        lhs: 10,
        production: &[ParseType::N(0)],
    },
    // 16 - Value: "true"^ /* Clipped */;
    Production {
        lhs: 10,
        production: &[ParseType::T(11)],
    },
    // 17 - Value: "false"^ /* Clipped */;
    Production {
        lhs: 10,
        production: &[ParseType::T(12)],
    },
    // 18 - Value: "null"^ /* Clipped */;
    Production {
        lhs: 10,
        production: &[ParseType::T(13)],
    },
    // 19 - String: "\u{0022}(?:\\[\u{0022}\\/bfnrt]|u[0-9a-fA-F]{4}|[^\u{0022}\\\u0000-\u001F])*\u{0022}";
    Production {
        lhs: 9,
        production: &[ParseType::T(14)],
    },
    // 20 - Number: "-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][-+]?(?:0|[1-9][0-9]*)?)?";
    Production {
        lhs: 4,
        production: &[ParseType::T(15)],
    },
];

static TOKENIZERS: Lazy<Vec<(&'static str, Tokenizer)>> = Lazy::new(|| {
    vec![(
        "INITIAL",
        Tokenizer::build(TERMINALS, SCANNER_0.0, SCANNER_0.1).unwrap(),
    )]
});

pub fn parse<'t, T>(
    input: &'t str,
    file_name: T,
    user_actions: &mut JsonGrammar<'t>,
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
    );
    llk_parser.trim_parse_tree();
    // Initialize wrapper
    let mut user_actions = JsonGrammarAuto::new(user_actions);

    llk_parser.parse(
        TokenStream::new(input, file_name, &TOKENIZERS, MAX_K).unwrap(),
        &mut user_actions,
    )
}
