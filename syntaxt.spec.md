FSM ::= <HEADER>* <LOGIC>
<HEADER> ::= <NAME> ":" <NAME>
<LOGIC> :== "{" <TRANSITION>* "}"
<TRANSITION> ::= <STATE-SPEC> <SUBTRANSITION> | <STATE-SPEC> "{" <SUBTRANSITION>* "}"

<STATE-SPEC> ::= <STATE> <STATE-MODIFIER>*
<STATE> ::= <NAME> | "(" <NAME> ")"
<STATE-MODIFIER> ::= ">"<NAME> | "<"<NAME> | ":" <NAME>

<SUBTRANSITION> ::= <EVENT> <NEXT-STATE> <ACTION>

<EVENT> :== <NAME> | "-"
<NEXT-STATE> ::= <NAME> | "-"
<ACTION> ::= <NAME> | "-" | "{" <ACTION>* "}"
