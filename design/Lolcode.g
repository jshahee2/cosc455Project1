grammar Lolcode;

// parser rules

lolcode        : HAI comment* head? variableDefine? body KBYE EOF ;

comment        : OBTW anyText* TLDR ;

head           : MAEK HEAD title MKAY ;
title          : GIMMEH TITLE anyText* OIC ;

variableDefine : IHAZ anyText ITIZ anyText+ MKAY ;
variableUse    : GIMMEH VAR anyText OIC ;

body           : innerBody* ;
innerBody      : paragraph | comment | bold | list | link | newline | variableUse | anyText ;

paragraph      : MAEK PARAGRAF variableDefine? innerText* MKAY ;
innerText      : variableUse | comment | bold | list | link | newline | anyText ;

bold           : GIMMEH BOLD anyText* OIC ;

list           : MAEK LIST listItem+ MKAY ;
listItem       : GIMMEH ITEM variableDefine? listContent* OIC ;
listContent    : bold | comment | link | newline | variableUse | anyText ;

link           : GIMMEH LINX anyText OIC ;
newline        : NEWLINE ;

anyText        : TEXT | HEAD | TITLE | PARAGRAF | BOLD | LIST | ITEM | LINX | VAR ;


// lexer rules (case insensitive)

HAI      : '#' [Hh][Aa][Ii] ;
KBYE     : '#' [Kk][Bb][Yy][Ee] ;
OBTW     : '#' [Oo][Bb][Tt][Ww] ;
TLDR     : '#' [Tt][Ll][Dd][Rr] ;
MAEK     : '#' [Mm][Aa][Ee][Kk] ;
OIC      : '#' [Oo][Ii][Cc] ;
GIMMEH   : '#' [Gg][Ii][Mm][Mm][Ee][Hh] ;
MKAY     : '#' [Mm][Kk][Aa][Yy] ;
NEWLINE  : '#' [Nn][Ee][Ww][Ll][Ii][Nn][Ee] ;
IHAZ     : '#' [Ii][Hh][Aa][Zz] ;
ITIZ     : '#' [Ii][Tt][Ii][Zz] ;

HEAD     : [Hh][Ee][Aa][Dd] ;
TITLE    : [Tt][Ii][Tt][Ll][Ee] ;
PARAGRAF : [Pp][Aa][Rr][Aa][Gg][Rr][Aa][Ff] ;
BOLD     : [Bb][Oo][Ll][Dd] ;
LIST     : [Ll][Ii][Ss][Tt] ;
ITEM     : [Ii][Tt][Ee][Mm] ;
LINX     : [Ll][Ii][Nn][Xx] ;
VAR      : [Vv][Aa][Rr] ;

TEXT     : [A-Za-z0-9,.":?!%=/]+ ;

WS       : [ \t\r\n]+ -> skip ;
