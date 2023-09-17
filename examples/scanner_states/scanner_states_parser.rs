// ---------------------------------------------------------
// This file was generated by parol.
// It is not intended for manual editing and changes will be
// lost after next build.
// ---------------------------------------------------------

use parol_runtime::once_cell::sync::Lazy;
#[allow(unused_imports)]
use parol_runtime::parser::{
    LLKParser, LookaheadDFA, ParseTreeType, ParseType, Production, Trans, UserActionsTrait,
};
use parol_runtime::{ParolError, ParseTree, TerminalIndex};
use parol_runtime::{TokenStream, Tokenizer};
use std::path::Path;

use parol_runtime::lexer::tokenizer::{
    ERROR_TOKEN, NEW_LINE_TOKEN, UNMATCHABLE_TOKEN, WHITESPACE_TOKEN,
};

pub const TERMINALS: &[&str; 11] = &[
    /*  0 */ UNMATCHABLE_TOKEN,
    /*  1 */ UNMATCHABLE_TOKEN,
    /*  2 */ UNMATCHABLE_TOKEN,
    /*  3 */ UNMATCHABLE_TOKEN,
    /*  4 */ UNMATCHABLE_TOKEN,
    /*  5 */ r"[a-zA-Z_]\w*",
    /*  6 */ r"\u{5c}[\u{22}\u{5c}bfnt]",
    /*  7 */ r"\u{5c}[\s^\n\r]*\r?\n",
    /*  8 */ r"[^\u{22}\u{5c}]+",
    /*  9 */ r"\u{22}",
    /* 10 */ ERROR_TOKEN,
];

pub const TERMINAL_NAMES: &[&str; 11] = &[
    /*  0 */ "EndOfInput",
    /*  1 */ "Newline",
    /*  2 */ "Whitespace",
    /*  3 */ "LineComment",
    /*  4 */ "BlockComment",
    /*  5 */ "Identifier",
    /*  6 */ "Escaped",
    /*  7 */ "EscapedLineEnd",
    /*  8 */ "NoneQuote",
    /*  9 */ "StringDelimiter",
    /* 10 */ "Error",
];

/* SCANNER_0: "INITIAL" */
const SCANNER_0: (&[&str; 5], &[TerminalIndex; 2]) = (
    &[
        /*  0 */ UNMATCHABLE_TOKEN,
        /*  1 */ NEW_LINE_TOKEN,
        /*  2 */ WHITESPACE_TOKEN,
        /*  3 */ r"(//.*(\r\n|\r|\n|$))",
        /*  4 */ r"((?ms)/\*.*?\*/)",
    ],
    &[5 /* Identifier */, 9 /* StringDelimiter */],
);

/* SCANNER_1: "String" */
const SCANNER_1: (&[&str; 5], &[TerminalIndex; 4]) = (
    &[
        /*  0 */ UNMATCHABLE_TOKEN,
        /*  1 */ UNMATCHABLE_TOKEN,
        /*  2 */ UNMATCHABLE_TOKEN,
        /*  3 */ UNMATCHABLE_TOKEN,
        /*  4 */ UNMATCHABLE_TOKEN,
    ],
    &[
        6, /* Escaped */
        7, /* EscapedLineEnd */
        8, /* NoneQuote */
        9, /* StringDelimiter */
    ],
);

const MAX_K: usize = 1;

pub const NON_TERMINALS: &[&str; 10] = &[
    /*  0 */ "Content",
    /*  1 */ "Escaped",
    /*  2 */ "EscapedLineEnd",
    /*  3 */ "Identifier",
    /*  4 */ "NoneQuote",
    /*  5 */ "Start",
    /*  6 */ "StartList",
    /*  7 */ "StringContent",
    /*  8 */ "StringDelimiter",
    /*  9 */ "StringElement",
];

pub const LOOKAHEAD_AUTOMATA: &[LookaheadDFA; 10] = &[
    /* 0 - "Content" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 5, 1, 3), Trans(0, 9, 2, 4)],
        k: 1,
    },
    /* 1 - "Escaped" */
    LookaheadDFA {
        prod0: 11,
        transitions: &[],
        k: 0,
    },
    /* 2 - "EscapedLineEnd" */
    LookaheadDFA {
        prod0: 12,
        transitions: &[],
        k: 0,
    },
    /* 3 - "Identifier" */
    LookaheadDFA {
        prod0: 10,
        transitions: &[],
        k: 0,
    },
    /* 4 - "NoneQuote" */
    LookaheadDFA {
        prod0: 13,
        transitions: &[],
        k: 0,
    },
    /* 5 - "Start" */
    LookaheadDFA {
        prod0: 0,
        transitions: &[],
        k: 0,
    },
    /* 6 - "StartList" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 0, 2, 2), Trans(0, 5, 1, 1), Trans(0, 9, 1, 1)],
        k: 1,
    },
    /* 7 - "StringContent" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[
            Trans(0, 6, 1, 5),
            Trans(0, 7, 1, 5),
            Trans(0, 8, 1, 5),
            Trans(0, 9, 2, 6),
        ],
        k: 1,
    },
    /* 8 - "StringDelimiter" */
    LookaheadDFA {
        prod0: 14,
        transitions: &[],
        k: 0,
    },
    /* 9 - "StringElement" */
    LookaheadDFA {
        prod0: -1,
        transitions: &[Trans(0, 6, 1, 7), Trans(0, 7, 2, 8), Trans(0, 8, 3, 9)],
        k: 1,
    },
];

pub const PRODUCTIONS: &[Production; 15] = &[
    // 0 - Start: StartList /* Vec */;
    Production {
        lhs: 5,
        production: &[ParseType::N(6)],
    },
    // 1 - StartList: Content StartList;
    Production {
        lhs: 6,
        production: &[ParseType::N(6), ParseType::N(0)],
    },
    // 2 - StartList: ;
    Production {
        lhs: 6,
        production: &[],
    },
    // 3 - Content: Identifier;
    Production {
        lhs: 0,
        production: &[ParseType::N(3)],
    },
    // 4 - Content: StringDelimiter Push(1) StringContent StringDelimiter Pop;
    Production {
        lhs: 0,
        production: &[
            ParseType::Pop,
            ParseType::N(8),
            ParseType::N(7),
            ParseType::Push(1),
            ParseType::N(8),
        ],
    },
    // 5 - StringContent: StringElement StringContent;
    Production {
        lhs: 7,
        production: &[ParseType::N(7), ParseType::N(9)],
    },
    // 6 - StringContent: ;
    Production {
        lhs: 7,
        production: &[],
    },
    // 7 - StringElement: Escaped;
    Production {
        lhs: 9,
        production: &[ParseType::N(1)],
    },
    // 8 - StringElement: EscapedLineEnd;
    Production {
        lhs: 9,
        production: &[ParseType::N(2)],
    },
    // 9 - StringElement: NoneQuote;
    Production {
        lhs: 9,
        production: &[ParseType::N(4)],
    },
    // 10 - Identifier: "[a-zA-Z_]\w*";
    Production {
        lhs: 3,
        production: &[ParseType::T(5)],
    },
    // 11 - Escaped: "\u{5c}[\u{22}\u{5c}bfnt]";
    Production {
        lhs: 1,
        production: &[ParseType::T(6)],
    },
    // 12 - EscapedLineEnd: "\u{5c}[\s^\n\r]*\r?\n";
    Production {
        lhs: 2,
        production: &[ParseType::T(7)],
    },
    // 13 - NoneQuote: "[^\u{22}\u{5c}]+";
    Production {
        lhs: 4,
        production: &[ParseType::T(8)],
    },
    // 14 - StringDelimiter: "\u{22}";
    Production {
        lhs: 8,
        production: &[ParseType::T(9)],
    },
];

static TOKENIZERS: Lazy<Vec<(&'static str, Tokenizer)>> = Lazy::new(|| {
    vec![
        (
            "INITIAL",
            Tokenizer::build(TERMINALS, SCANNER_0.0, SCANNER_0.1).unwrap(),
        ),
        (
            "String",
            Tokenizer::build(TERMINALS, SCANNER_1.0, SCANNER_1.1).unwrap(),
        ),
    ]
});

pub fn parse<'t, T>(
    input: &'t str,
    file_name: T,
    user_actions: &mut dyn UserActionsTrait<'t>,
) -> Result<ParseTree<'t>, ParolError>
where
    T: AsRef<Path>,
{
    let mut llk_parser = LLKParser::new(
        5,
        LOOKAHEAD_AUTOMATA,
        PRODUCTIONS,
        TERMINAL_NAMES,
        NON_TERMINALS,
    );
    llk_parser.parse(
        TokenStream::new(input, file_name, &TOKENIZERS, MAX_K).unwrap(),
        user_actions,
    )
}
