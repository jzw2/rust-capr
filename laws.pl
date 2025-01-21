

:- use_module(library(lists)).
:- use_module(library(dcgs)).
:- use_module(library(dif)).
:- use_module(library(si)).
:- use_module(library(debug)).


lab("p").
lab("b_h").

dent("t").
dent("d").
dent("d_h").

vel("k").
vel("g").
vel("g_h").

pal("k_j").
pal("g_j").
pal("g_jh").

labvel("k_w").
labvel("g_w").
labvel("g_wh").

lar("h_1").
lar("h_2").
lar("h_3").
lar("h_h").

res("m").
res("n").
res("l").
res("r").


nas("m").
nas("n").

vow("o").
vow("a").
vow("i").
vow("e").
vow("u").

tonic(X) :-
  append(V, "_H", X), % represent 
  vow(V).

stop(X) :- lab(X).
stop(X) :- dent(X).
stop(X) :- vel(X).
stop(X) :- labvel(X).
stop(X) :- pal(X).

stop_or_s(X) :- stop(X) ; X = "s".

cons(X) :- stop_or_s(X).
cons(X) :- lar(X).
cons(X) :- res(X).


liq("l").
liq("r").

all_cons --> [].
all_cons --> seq(X), { cons(X)}, all_cons.

tonic_syl(X) :-
  append(Cons, Vow, X),
  phrase(all_cons, Cons),
  tonic(Vow).

ends_in(End, String) :-
  append(_, End, String).
  

match(String, Pattern) --> {
 dif(Pattern, []), atom(Pattern), call(Pattern, String)
}, seq(String).

match(Pattern, Pattern) --> seq(Pattern), { list_si(Pattern) }.

match(String, (P1, P2)) -->
   match(S1, P1),
   match(S2, P2),
   { append(S1, S2, String)}.

replace(law(From, To, Left, Right), New) -->  
  seq(NonContextL),
  match(LeftStr, Left),
  match(_, From),
    match(RightStr, Right),
  seq(NonContextR),
   {
    phrase(seqq([NonContextL, LeftStr, To, RightStr, NonContextR]), New)

  }.

context_match(law(_, _, Left, Right), Center) -->  
  seq(_),
  match(_, Left),
  seq(Center),
    match(_, Right),
  seq(_).

valid_length(FromStr, ToStr, Old, New) :-
  length(S1, L),
  length(S2, L),
  append(FromStr, New, S1),
  append(ToStr, Old, S2).

  
replace_sym(Law, Old, New) :-
  Law = law(From, To, Left, Right),
  valid_length(FromStr, ToStr, Old, New),
  phrase((seq(L), match(LC, Left), match(FromStr, From), match(RC, Right), seq(R)), Old ),
   phrase((seq(L), match(LC, Left), match(ToStr, To), match(RC, Right), seq(R)), New ).
  
replace_re(law(From, _, Left, Right), Old, true) :-
   phrase((..., match(_, Left), match(_, From), match(_, Right), ...), Old).
replace_re(Law, Old, false) :- \+ replace_re(Law, Old, true).
  

replace_once(Law, Old, New) :-
   phrase(replace(Law, New), Old).

replace_all(law(From, _, Left, Right), Old, Old) :-
   \+ phrase((..., match(_, Left), match(_, From), match(_, Right), ...), Old).
replace_all(Law, Old, New) :-
   phrase(replace(Law, Replaced), Old),
   replace_all(Law, Replaced, New).


laws([


% 1
law("h_1e", "e", "", ""), 
law("h_2e", "a", "", ""),
law("h_3e", "o", "", ""),

% 2
law("eh_1", "ee", "", ""),
law("eh_2", "aa", "", ""),
law("eh_3", "oo", "", ""),

% 3
law(lar, "a", cons, cons),


% 4
law(lar, "", stop, stop),


% 5
law((dent, dent), "ss", "", ""),


% 6
law("", "a", (cons, res), (lar, cons)), % contractits the previous laryngaial between consonants

% 7
law(lar, "", vow, (cons, tonic_syl)),

% 8
law(lar, a, ("#", res), cons),

% 9
law("k_j", "k", "",""),
law("g_j", "g", "",""),
law("g_jh", "g_h", "",""),



% B

% 1
law("g_w", "b", "",""),

% 2
law("b_h", "b", "",""),
law("d_h", "d", "",""),
law("g_h", "g", "",""),
law("g_wh", "g_w", "",""),


% 3
law("", "i", (cons, liq), stop),

% 4
law("e", "a", "", (liq, "a")),

% 5 
law("", "a", cons, (res, cons)),

% 6 % I assume this means taht when it is next to a vowel
law(lar, "", vow,""),
law(lar, "", "", vow),


% 7 
law("p", "k_w", "",ends_in("k_w")),


% 8
law("e:", "i:", "",""),

% 9
law("o:", "u:", "",""), % ignore final sylables


% 10 not here for some reason

% 11
law(":", "", vow, (res, cons)), 


% C

% 1

law(cons, "x", "",(cons_or_s)),

% 2
law("p", "b", "",liq),

% 3
law("p", "w", "",liq),

% 4
 law("p", "f", "",""),


% 5
law("o:", "a:", "",""),
% 6
law("ey", "e:", "",""),


% 7
law("ew", "ow", "",""),


% 8
law("uw", "ow", "",cons),


% end
 
law("xx", "x", "",""),
law("xx", "x", "","")
]).


% replace this with a fold
transduce_(Word, Word, [], [Word]).
transduce_(Word, NewWord, [Law | Rest], Changes) :-
  replace_all(Law, Word, Replaced),
  (Word = Replaced, Changes = NewChanges ;
  $ dif(Word, Replaced),  Changes = [(Word, Law) | NewChanges]),
  transduce_(Replaced, NewWord, Rest, NewChanges).

transduce(Word, NewWord, Changes) :-
  laws(Laws),
  transduce_(Word, NewWord, Laws, Changes).


transduce_re_(Word, Word, [], [Word]).
transduce_re_(Word, NewWord, [Law | Rest], Changes) :-
  replace_re(Law, Word, true),
  replace_sym(Law, Word, Replaced),
  $ Changes = [(Word, Law) | NewChanges],
  transduce_re_(Replaced, NewWord, Rest, NewChanges).
transduce_re_(Word, NewWord, [Law | Rest], Changes) :-
  replace_re(Law, Word, false),
  transduce_re_(Word, NewWord, Rest, Changes).

transduce_re(Word, NewWord, Changes) :-
  length(Word, _),
  length(NewWord, _),
  laws(Laws),
  transduce_re_(Word, NewWord, Laws, Changes).
examples(
  [
  % A
  "#ph_2te:r#", % 3 fati:r
  "#d_hugh_2te:r#", % 4 duxti:r
  "#krdtu#", % 5  krissu
   "#plh_1no#", % 6  gra:no
  "#g_jrHno#", % 6  fla:no
  "#wirHro_H#", % 7 wiro
  "#dek_jm#", % 9  

  % B
  "#g_wow#", % 1
  "#b_hero", % 2
  "#g_wher", % 2

  "cat"
]).

transduce_all(Words, Changes) :-
  examples(Ex),
maplist(transduce, Ex, Words, Changes).
