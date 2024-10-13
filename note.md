<text> ::= <b> "1"
<b> ::= "b" | <a1> ε | <a2>
<a1> ::= "a1"
<a2> ::= "a1" | "a"


(level) - choice_stack
0 - text.0.0 b.2
1 - b.2.0 a2.1

branch_stack
2 - a2.1.0

<text> ::= <a> "end"
<a> ::= <a_sub> <a> | <a_sub>
<a_sub> ::= "a" <b> | "a"
<b> ::= "b"

matchin aaa bend

there are two stacks :
    - the choices you made (which potentially are not correct)
    - where you are in the sequence


(level) - choice_stack
0 - text.0.0 a.0
1 - a.0.0 a_sub.1
1 - a.0.1 a.0
2 - a.0.0 a_sub.1
2 - a.0.1 a.1
3 - a.1.0 a_sub.0
4 - a_sub.0.1 b.0

branch_stack
0 - text.0.1

this algorithm is the following : 

you store currently:
- depth level
- exact position (label + alt index + seq index)
- position in string

in stack you store:
- depth level
- exact position of choice (label + alt index + seq index)
- position in string
- which option you chose (label + alt index)


when matching an element:
    if you encounter a reference, then:
        to the choice stack you add the reference of where you are, and where you are going
        (level + label+alt+seq + label+alt)
        you move to where you are going (seq = 0)
        you bump the depth
        next loop

    if you fail to match an element:
        you pop the last choice, bump the destination (b.0 -> b.1), and add it back
        you also move to the new destination (b.1.0)
        you bump the depth
        next loop

    if you are outside of alt bounds:
        you ditch the last choice stack
        then you act like you failed to match an element

    if you match a complete sequence (outside of sequence bounds):
        you go back one level (if you level 2, go back to closest level 1), and bump position
        you don't remove information from the choice stack
