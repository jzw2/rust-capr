

:- use_module(library(lists)).
:- use_module(library(dcgs)).
:- use_module(library(dif)).
:- use_module(library(si)).
:- use_module(library(debug)).
:- use_module(library(clpz)).
:- use_module(library(format)).
:- dynamic(call_stuff/2).
:- dynamic(stuff/2).
:- dynamic(test/2).


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
  vow(V),
  phrase((seq(V), "_H"), X).

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
  tonic(Vow),
  length(X, _),
  phrase((all_cons, seq(Vow)), X, _).

ends_in(End, String) :-
  append(_, End, String).
  

match(String, Pattern) --> {
 dif(Pattern, []), atom(Pattern), call(Pattern, String)
}, seq(String).

match(Pattern, Pattern) --> seq(Pattern), { list_si(Pattern) }.

match(String, (P1, P2)) -->
   { $ append(S1, S2, String)},
   match(S1, P1),
   match(S2, P2).

match_pred(S, P) :- phrase(match(S, P), S).

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

valid_length(From, To, Old, New) :-
  phrase(match(FromStr, From), FromStr),
  phrase(match(ToStr, To), ToStr),
  length(FromStr, Fl),
  length(ToStr, Tl),
  Fl + NL #= Tl + OL, !,
  length(Old, OL),
  length(New, NL).
  
replace_sym(Law, Old, New) :-
  Law = law(From, To, Left, Right),
  append(L, DifO1, Old),
  append(L, DifN1, New),
  append(DifO2, R, DifO1),
  $ append(DifN2, R, DifN1),
  phrase( (match(LC, Left), match(_, From), match(RC, Right)), DifO2 ),
   phrase( (match(LC, Left), match(_, To), match(RC, Right)), DifN2 ).
  
replace_re(law(From, _, Left, Right), Old, true) :-
   phrase((..., match(_, Left), match(_, From), match(_, Right), ...), Old).
replace_re(Law, Old, false) :- \+ replace_re(Law, Old, true).


replace_exact(Law, Old, New) :-
  Law = law(From, To, Left, Right),
  phrase( (match(LC, Left), match(_, From), match(RC, Right)), Old),
   phrase( (match(LC, Left), match(_, To), match(RC, Right)), New ).
  
double_nil([]-[]).
double_cons([X | R1]-[X | R2], X, R1-R2).

double_seq([], L1, L1).
double_seq([X | Rest], L1, L) :-
  double_cons(L1, X, L2),
  double_seq(Rest, L2, L).

double_match(Pattern, L1, L) :-
  $ double_seq(S, L1, L),
  $ match_pred(S, Pattern).

double_replace(law(From, To, Left, Right), L1, L) :-
  $ double_seq(_, L1, L2),
  $ double_match(Left, L2, L3),
  $ L3 = Old - New,
  $ phrase(match(_, From), Old, Old1),
  $ phrase(match(_, To), New, New1),
  $ L4 = Old1 - New1,
  $ double_match(Right, L4, L5),
  $ double_seq(_, L5, L).
  

  
  
  

replace_once_sym(Law, Old, New) :-
  $ replace_exact(Law, Old1, New1),
  append(Old1, S, Old),
  append(New1, S, New).
replace_once_sym(Law, [S | Old], [S | New]) :-
  replace_once_sym(Law, Old, New).
  
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

law(cons, "x", "",(stop_or_s)),

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
  replace_re(Law, Word),
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

transduce_exact_sym(Word, NewWord, Changes) :-
  length(Word, _),
  length(NewWord, _),
  laws(Laws),
  transduce_exact_sym_(Word, NewWord, Laws, Changes).

transduce_exact_sym_(Word, Word, [], [Word]).
transduce_exact_sym_(Word, NewWord, [Law | Rest], Changes) :-
  $ replace_once_sym(Law, Word, Replaced),
  $ Changes = [(Word, Law) | NewChanges],
  transduce_exact_sym_(Replaced, NewWord, Rest, NewChanges).
transduce_exact_sym_(Word, NewWord, [_ | Rest], Changes) :-
  $ transduce_exact_sym_(Word, NewWord, Rest, Changes).

transduce_double(Word, NewWord, Changes) :-
  laws(Laws),
  transduce_double(Word, NewWord, Laws, Changes).

transduce_double(Word, Word, [], [Word]).
transduce_double(Word, NewWord, [Law | Rest], Changes) :-
  $ double_replace(Law, Word-Replaced, []-[]),
  Changes = [(Word, Law) | NewChanges],
  transduce_double(Replaced, NewWord, Rest, NewChanges).
transduce_double(Word, NewWord, [_ | Rest], Changes) :-
  transduce_double(Word, NewWord, Rest, Changes).
examples(
  [
  % A
  "#ph_2te:r#", % 3 fati:r
  "#d_hugh_2te:r#", % 4 duxti:r
  "#krdtu#", % 5  krissu
   "#plh_1no#", % 6  gra:no
  "#g_jrh_hno#", % 6  fla:no
  "#wih_hro_H#", % 7 wiro
  "#dek_jm#", % 9  dekam

  % B
  "#g_wow#", % 1 bow
  "#b_hero", % 2  bero
  "#g_wher", % 2 g_wero
  "#k_jrd", % 3  kridyo
  "#k_wrmi", % 3  k_wrimi
  "#h_1lmo", % 3 limo
  "#terh_1tro", % 4 taratro

  "cat"
]).

transduce_all(Words, Changes) :-
  examples(Ex),
  maplist(transduce, Ex, Words, Changes).
transduce_all_double(Words, Changes) :-
  examples(Ex),
  maplist(transduce_double, Ex, Words, Changes).
