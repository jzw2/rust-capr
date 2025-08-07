#import "@preview/linphon:0.1.0" as lp
#import "@preview/ascii-ipa:2.0.0": *

// #import "@preview/casson-uom-thesis:0.1.1": *
#import "template.typ" : *



#show: uom-thesis.with(
  title: "My Thesis",
  author: "John Wang",
  faculty: "Philosophische Fakultät",
  school: "Tübingen",
  departmentordivision: " Seminar für Sprachwissenschaft",
  //termsandabbreviations: [Enter terms and abbreviations as table or similar], // uncomment if want in thesis
  // layabstract: [Lay abstract goes here], // uncomment if want in thesis
  // theauthor: [If desired, a brief statement for External Examiners giving the candidate’s degree(s) and research experience, even if the latter consists only of the work done for this thesis.], // uncomment if want in thesis
  year: "2025",
  font: "times", // choices are: "times", "palatino", "roboto", "noto_sans" 
  fontsize: 11pt, // can be any reasonable value
)




#set heading(numbering: "1.1.1")
#import "@preview/frame-it:1.2.0": *




 #let (definition, statement ) = frames(
  definition: ("Definition",),
  // For each frame kind, you have to provide its supplement title to be displayed
  statement: ("Statement",),
  // You can provide a color or leave it out and it will be generated
)

// This is necessary. Don't forget this!
#show: frame-style(styles.boxy)


= Historical Linguistics


People have been noticing that language changes. Historical Linguistics is the study of change in language. @hock2021principles

The living pieces of evidence that language changes is simply examining the English language. The English language is notoriously known for its frequently arbitrary spelling rules. @silentk shows a list of _k_ words that do not have the _k_ sound. 


#figure(
  table(
    columns: 2,
    table.header([English Orthography], [IPA transcription]),
    [knee], [niːd],
    [knight], [naɪt],
    [knead], [niːd],
    [knuckle], [nʌkəl],
    [know], [noʊ]

  ), 
  caption: [Illogical English Spelling]
) <silentk>


A partial reason is because of the effects of sound change. English spelling often reflects earlier stage of the language, where things were pronounced differently. The spoken language changed, but the written language had failed to update to the newer pronunciation. 

This evidence is further supported when we look at some words from German, which happen to have similar meanings, shown in @silentkgerman. In the German words, the \<k\> is pronounced, unlike in English. 


#figure(
  table(
    columns: 3,
    table.header([English Word], [German Word], [Meaning]),
    [knee], [Knie], [knee],
    [knight], [Knecht], [Male Servant],
    [knead], [kneten], [to knead],
    [knuckle], [Knochen], [bone]
  ), 
  caption: [German Words]
) <silentkgerman>

It can be theorized that German and English were once the same language. However, through the many years of being apart, various changes in pronunciation, grammar, and meaning have effected both languages, making them non mutually intelligible. One of these changes that happened relatively recently was the change shown in @silentk, of deleting \<k\> when before \<n\>, which happened in English, but did not happen in German. 



== Neogrammarian Hypothesis


As a child learning how to read English, seeing the spelling of the words listed in @silentk for a general rule: \<kn\> gets pronounced as \<n\>. This generalization parallels the development of the development of English, so what initially seems to be a pronunciation rule may be more accurately described as a _sound change rule_ or a _sound law_, a term used by the Neogrammarians. 

Additionally, this particular sound law is observable because of the conservative orthography of English. But English has only (relativly speaking) recently been written down. If, for instance, English and German had written down their language back when they were one language, would it be possible for a learner of the language to similarly form a large list of rules to derive the English pronunciation, and a different set of rules to derive the German pronunciation?

The Neogrammarians had a similar thought, and believed not only that English and German could be derived from a common ancestor like this, but that they could project even further back, relating most language of Europe along with many languages of India. These languages included English, German, French, Spanish, Greek, Russian, Hindi, and more. 

The oldest literarly languages known to Neogrammarians: Latin, Greek, and Sanskrit, not only had correspondences in vocabulary, but also similarities in Grammar. This led them to postulate the existance of the Proto-Indo-European (PIE) language, which then diversified and split into various daughter languages.  Similar to the example of English and German, often one language might have had a sound law apply, but another language might have preserved a more archaic form.   @hock2021principles 

The Neogrammarians craeted these sound laws under the Neogrammarian Hypothesis, as shown in @neogrammarian

#definition[Neogrammarian Hypothesis][Sound Laws are regular] <neogrammarian>

In this, they tried to bring linguistics in line with the the laws of the natural world. This may be roughly equivalent to saying that all languages consist of a mathematical function taking a protoword and outputs a modern reflex. This function would then be composed of smaller functions, the sound laws, that could then be combined under function composition.

Like any science, the actual data will not 100%.  They believed what exception that they encountered were merely sound laws that had been not sufficiently generalized.



However this is probably not a very realistic model of language. Unlike the physical world, which has laws written in the language of mathematics that will predict exactly what will happen, languages are full of arbitrariness.

There are examples of changes that are _sporadic_ and effect only one word. Or maybe non determinsitcs behavior, where a single word splits. 



== Sound Laws <soundlaws>

Linguists have a conventionalized representation for laws.

Usually they write them in teh form $A > B$ where $A$ is a phoneme that gets changed, and $B$ is the phoneme that it gets changed into. Another common notation is shown in @contextfree.  It cal be chagined such as  $A > B > C$, which can be viewed as a short hand of two laws, $A > B$ and $B > C$. 



#figure(lp.rule([A], [B])) <contextfree>


@contextfree reprents a sound law that happens reguardless of the context it appears in. An example would be *whine-wine* merger, which can be formerly notated as @whinewine

#figure(lp.rule([ʍ], [w]), caption: [The Whine Wine merger]) <whinewine>


However, it is more common that sounds change in only restricted contexts. This is exemplified in @withcontext
@crowley2010introduction

#figure(lp.rule([A], [B], [X #lp.dash() Y]), caption: [Sound Law with context]) <withcontext>


where the variables $A, B, X, Y$ can represent strings, or potential features. The variables $X, Y$ represent the left hand side and right hand side contexts. 

Phonetic features sometimes allow for a more succinct pattern in expressing a sound law, and may also reveal a more general pattern. For instance, Grimm's law is a sound change encompasses 12 smaller sound laws as shown in @grimm


#figure(
  table(
    columns: 3,
lp.rule([p], [f]) ,  lp.rule([b], [p]), lp.rule([bʰ], [b]),
lp.rule([t], [#sym.theta]) ,  lp.rule([d], [t]), lp.rule([dʰ], [d]),
lp.rule([k], [x]) ,  lp.rule([g], [k]), lp.rule([gʰ], [g]),
lp.rule([kʷ], [xʷ]) ,  lp.rule([gʷ], [kʷ]), lp.rule([gʰʷ], [gʷ]),
  ), 
  caption: [Grimm's law]
) <grimm>

This can be rewritten in a more concisely using features as seen in @grimmfeature 

#figure(
  [
  + #lp.rule(
  lp.fmat(
    ("+", "stop"),
    ("-", "voice"),
  ),
  lp.fmat(
    ("+", "fric"),
  )
) 
  + #lp.rule(
  lp.fmat(
    ("+", "stop"),
    ("+", "voice"),
  ),
    lp.fmat(
      ("-", "voice"),
    )
  )
  
  + #lp.rule(
  lp.fmat(
    ("+", "stop"),
    ("+", "asp"),
  ),
  lp.fmat(
    ("+", "voice"),
    ("-", "asp"),
  )
)
]
,
  caption: [Grimm's law with features]
) <grimmfeature>

It is important to note that this a rough convention. The notation is inadequate to describe all types of sound law change. Authors will usually need to supplement this notation with english language explanations. However, for simple cases, it is possible to a sound change unambiguously. 


== Computational Linguistics

It would seem that this notation would easily be read by a computer. Indeed, it is not hard to write a computer program that, given a set of words, and the sound laws that are known, to then write a computer program that transduces this. 

There are many benefits of this doing this. Is that we now have _verification_ of the sound laws. A linguist would otherwise be the only source of trust. As the amount of sound changes is usually non trivial, it is not hard to accidentally forget to apply a law, or apply it in the wrong order. 

In theory, the idea that sound laws can be written in a formal notation allows for a sort of algorithm to be created that will automatically.

It is ironic that these sorts of explorations have not been popular with linguists. In fact, in terms of technology, the linguistic community has been relatively slow in adopting computer usage. For instance, 
it was discovered that linguists had been using a word processor to discover sound laws. @sims2018mechanising This very primative method could have been automated by a very simple program, and yet was used to discover a new sound law. Each transduction was more or less done by hand, except each phoneme was assisted using the word processor's Control-F functionality. 

In recent usage of historical linguistics 
more attention has been drawn to different aspects of computeratial linguistics, such as _phylogenetics_: quantifying how related languages or language familes are. And _cognate detection_: given a corpuses of languages, can it be used to detect find potential words that are cognates. @sims2018mechanising  Only recently has there been approaches that return back to the Neogrammarian hypothesis created a hundred years ago. 


== Reframing the Neogrammarian hypothesis


The Neogrammarian hypothesis: _all sound laws are regular_ is a strong statement. And to take it literally would unfortunately very hard, once a little time is spent working with sound laws. The main problem is that language is full of _irregularity_. There are very many ways in which sound laws will give the incorrect result when applying the soundlaws without some leeway for correction.

The Neogrammarians were had already known that _analogy_ was a common source of irregularities. For instance, English has leveled its verb conjugation for most verbs, only adding the _s_ to the third person singular: _I eat_, _They eat_, _We eat_,_He eats_. But the verb _to be_ has resisted this change just for the first person singular: _They are_, _We are_, but _*I am*_. 

In Old English, there would have been a different verb form for all verbs in the first person singular. However, through paradigm leveling, many of the verb forms are now just the stem. If the sound laws were applied to these word form directly, it would likely create many different outcomes instead of just 

// todo rewrite the above

Borrowing also creates some potential problems with the Neogrammarian hypothesis. When a word is borrowed, it lacks the sound laws that would otherwise apply to the rest of the lexicon. For instance the German word _Computer_ is borrowed from the English word _computer_ which looks like it has had no sound laws applied to it. 

Loan words can be especially troublesome with respect to the presence of _doublets_ which are words which essentially borrow from themselves. In Romance lanuages, the core vocabulary comes from Latin. In the natural process of time, have undergone various sound changes. However, due to Latin being used as a prestige language, the original latin word has been reborrowed along with the inherited word. An example is French _hotel_ and _hospital_ stemming from the same original Latin word _hospitalis_. Any sound change system that tries to mjodel this would require two different sound law paths. One which would require the inherited sound laws, another which would require less sound laws, that is more direct from the Latin. 

Similarly, in Japanese, many Chinese words were borrowed. However, Chinese words were continuously borrowed throughout the entire history. Sometimems the same Kanji would have multiple readings depend on the compound that it was in, and from what dialect of Chinese it came from. 

Even accounting for analogy and loans, sometimes sound changes do not have a predictable outcome. For instance, Mandarin Chinese is well known for not having stop codas, which used to constitute a tonal group. Due to Mandarin's phonology, where every syllable has to have a tone, the stop codas were distributed to other tones, but seemingly without any consistency. Thus, creating an algorithm to predict the outcome tone would sometimes be non deterministic. 

// todo maybe give an example for this

With all these exceptions, it's clear that any algorithm would potentially have problems even if the algorithm is otherwise completely accurate with respect to the sound laws. Due to the potentially large engineering feat that may be required, in additional to numerous exceptional cases, it is likely the apparent effort in constructing such a task was deemed to be too much. This expalains why many explorations into computational methods for historical linguistics have focused more tasks which require less supervision. They commonly use a general machine learning like workflow of data aggregations, followed by model that learns from the data and then compete with beating human annotated work. In contrast, building an algorithm for this requires not just a programmer, but someone who has linguistic expertise, and can examine each individual case. 

Despite these problems, there is an advantage to using the Neogrammarian hypothesis. Its easy to mistake the strictness of the Neogrammarian hypothesis as a weakness, but it is instead be viewed as a strength. Its strength is that it can be "wrong", but will provide a reasoning as to why this is wrong. In this, it is parallel to other sciences. Theories can be wrong and fail, but they will have a reasoning as to why it failed.

A common example is Grimm's law. The Latin word for _have_ is _habeo_. On the surface, these two words have simlar semantic meaning and phonetic strucuture. It might be logical to conclude that these two words are cognate. However, this fails to obey Grimm's law, which says that a germanic /h/ should be correspond with a Latin /k/. Indeed, the two are not cognates, but are just similar by coincidence. The true cognate of English _have_ is Latin _capio_, which has a meaning of take. 

Without this udnerstanding of sound laws, it is all too easy to assign these two as cognates. And this also explains the hesitancy of traditional historical linguistics in adopting computer based approaches. A system that uses a machine learning like approach will require a simpler model of language, meaning a full sound law system is too complicated to adopt requiring simplificatons such as context free changes. 
//cite a paper to give examples todo

Additionally, this approach has been popular because of similar work being done in computational biology where essentially all changes are sporadic. Work has been done, but explicitely goes again the Neogrammarian hypothesis. Also, this hypothesis makes it intrinsicially hard to verify if theory is correct. The Neogrammarian hypothesis in theory allows all the work to be done in the linguist's head. 

Most importantly, the Neogrammarian hyptothesis makes evidence explicit.  This is useful for other linguists. Meanwhile, previous approaches in autmatic reconstruction becomes not useful for linguists. The work that the autmatic constructions does is not useful for linguists. Any reconstructoin that the system does has to be reviewed by the linguist anyway. Additionally, the reasoning that the system used is not clear to the linguist. 

It could be better sumarizeed that the language itself is in general not completely regular. But that a model with higher regularity is easier for linguists to work with. A linguist will find it easier to model rules that are regular, becuase this can be viewed as evidence. Under this assumption, we can also view how regular a theory is. And using this we can also 

Additionally, this may also be used to better model wave based models. For instance, areal features, may also be viewed as similar as other approaches. The sound law may be applied without any dependency on the tree model. 



== Sound Changes as a study of itself

The use of sound laws appears to be understudied by the historical linguist. Instead, conlanguers seems to have taken a greaker interest in the study of sound changes. For instance, the Index Diachronica is a compilation of sound laws of languages that occurred throughout the world. @diachronica It's purpose is to provide conlangers with a place to view what real sound laws that have been documented to occur. This gives them a source of reference when making sound laws that occurred in their own languages. 

However, the diachronica is not a very formal piece of writing, in the sense that a computer program can be automatically made from it. It is essentially a list, that likely requires a human to make inferences. 

Other attempts at staying with a _sound law_ approach of historical linguistics include a reconstructing Proto-Algonquin from daughter languages using sound laws. This was made in the 1970s by John Henson. @sims2018mechanising. 

Additionally, conlangers have made a tool for forward applications, in order to verify thtier own laws would work properly, or at least regularly. This provides forward applications, but it does not provide backward application. As it is just for verification. @zompist 

Otherwise there have been various ad hoc tranducer, indeed there has also been various methods also using fsts. 


 = Finite State Trandsducers

 In this chapter, Finite State Tranducers (FSTs) are introduced. However, in order to define FSTs and motivate their usefulness, a some background information is required. FSTs can be viewed as a generlization of Finite State Auomata (FSAs), and have important properties about their usage. Only the most important results will be listed here, and will be presented without proof. 

Much of this chapter is adapted from #cite(<kaplan1994regular>, form: "prose") . 

 == Finite State Automata

 Finite state automata (or finite state machine) are a mathematical model basic form of autmata that can be used ot model a variety of phenomona. The most relvant usage is in string recognition.

 
 == Formal Definition

 === Strings
To talk about strings, a formal definition is required.

#definition[String][
Given a set $Sigma$ a _string_ can be defined recursively defined as being either
- $epsilon$, the empty string
- $c dot S$ where $c in Sigma$ and $S$ is a string
] <strings>
The set $Sigma$ is usually called the _alphabet_ and the members _characters_. The set of strings created from this $Sigma$ can be denoted as $Sigma^*$. The $dot$ operator can be extended to a concatenation operator of two strings where

- $epsilon dot S = S$
- $(c dot S_1) dot S_2 = c dot (S_1 dot S_2)$ 

A string is a common example of a _monoid_ because the $dot$ operator is associative, and $epsilon$ is the identity. 
By convention, strings are usually put in double quotes and the $dot$ operator commonly ommited. Examples: todo


#definition[Language][
A formal _language_ can be defined a (potentially infinite) set of strings. 
] <language>

Can be used to define natural language such as English, spansih, but can also be used to define more artifical lanuages, such as all prime numbers represented in decimal. 

=== Finite State Machine <fsm-section>

#definition[Deterministic Finite State Machine][
A deterministic finite state machine can be defined as  $(Q, s, Sigma, F, delta)$, where 

- $Q$ is the set of states
- $s in Q$ is the start state
- $Sigma$ is the _alphabet_, a set of characters
- $F subset Q$ the set of final states
- $delta: Q times Sigma -> Q$ the transition function that maps each state to its next state when given a character
]

A finite state machine $M$ can then be used to define a language $L$ by stating that a word is in $L$ if and only if $M$ accepts $L$.

// A finite state machine $(Q, s, Sigma, F, delta)$ accepts a string $S$ when one of the following conditions are met:

// - $S = epsilon$ and $s in F$ 
// - $S = c dot S'$ and $delta(q, c) = q'$ and $M' = (Q, q', Sigma, F, delta)$ accepts $S'$

Every $delta$ can be extended to $delta^* : Q times Sigma^* -> Q $ function by defining 

- $delta^*(q, epsilon) = q$
- $delta^*(q, c dot S) = delta^*(delta(q, c), S)$ for $c in Sigma$


Finally, a machine _accepts_ a string $S$ if and only if $delta^*(s, S) in F$. 


todo: add an example with a picture

A related construct is the nondeterministic finite state machine or automata, also known as a NFA.

#definition[Nondeterministic finite state machine][
A nondeterministic finite state machine can be defined as  $(Q, s, Sigma, F, delta)$, where 

- $Q$ is the set of states
- $s in Q$ is the start state
- $Sigma$ is the _alphabet_, a set of characters
- $F subset Q$ the set of final states
- $delta: Q times (Sigma union {epsilon}) -> P(Q)$, $P$ being the powerset function. The nondeterministic transition function that maps each state and character to possible states. 
]

Here, the transition function is different and can now map to multiple or zero states, in addition to just one state. Additionally, it can have $epsilon$ transitions, with the intuition of non consuming parts of the string, while moving to a different state. 

The non deterministic finite state machine also has a similar accepts function. 

Every $delta$ can be extended to $delta^* : Q times Sigma^* -> P(Q) $ function by defining 

- $delta^*(q, epsilon) = delta(q, epsilon)$
-  $ delta^*(q, c dot S) = union.big_(q' in delta(q, c)) delta^*(q', S) $ 

for each $c in Sigma union {epsilon }$


Finally, an NFA _accepts_ a string $S$ if and only if $delta^*(s, S) inter F eq.not emptyset$. 

// A finite state machine $(Q, s, Sigma, F, delta)$ accepts a string $S$ when one of the following conditions are met:

// - $S = epsilon$ and $s in F$ 
// - $S = c dot S'$ and there exists a $q$ such that $q' in delta(q, c)$ and $M' = (Q, q', Sigma, F, delta)$ accepts $S'$

A deterministic finite state machine is trivially a non deterministic finite state machine. However, it may be surprising to know the converse is also true: any non deterministic finite state machine can be turned into a deterministic finite state machine which accepts the same language, stated in @nfs-equals-dfa.

#statement[Deterministic and Nondeterministic Finite State Machine Equivalence][
  A language $L$ can be recognized by a a deterministic finite state mchine $F$ if and only if there exists there exists a nondeterinistic finite state mahine $F'$ that recognizes $L$.
] <nfs-equals-dfa>



 Due to @nfs-equals-dfa, it usually does not matter whether it is called a deterministic finite state machine or a non determinstic finite state machine, due the set of languages they recognize being the same. In general, they can be referred as (FSM) finite state machines.
 
When dealing with the theoretical properties of FSTs, whether the underlying machine is deterministic or nondeterministic can be relegated as an implementation detail. However, when dealing with real systems, the difference is more important, as determinizing a finite state machine can have a worst case performance of $O(2^n)$.
== Elementary Results in FSMs

=== Regular Expression


Many programming languges have some sort of library for _regexes_, often used for string processing. The term _regex_ is a shortening of the term _regular expression_, which @regular-expression defines. 



#definition[Regular Expression][
Given an alphabet $Sigma$, a _regular expression_ can be recursively defined as either, 


- $epsilon$ the empty regular expression.
- $c$ where $c in Sigma$ the singleton character
- $R_1 dot R_2$ where $R_1$ and $R_2$ are also regular expressions, concatenation.
- $R_1 | R_2$ where $R_1$ and $R_2$ are also regular expressions, disjunction.
- $R_1^*$ where $R_1$ is a regular expression, the Kleene Star.
] <regular-expression>

A regular expression is not useful unless its semantics are also defined, as shown in @regex-semantics.

#definition[Regex Matching][

Given a particular expression $R$, $R$ _matches_ a string $S$ under the following conditions

- $R = epsilon$ and $S = epsilon$
- $R = c$, where $c in Sigma$  and $S = c$
- $R = R_1 dot R_2$ and there exists $S_1$ and $S_2$ such that $S = S_1 dot S_2$ and $R_1$ matches $S_1$ and $R_2$ matches $S_2$
- $R = R_1 | R_2$ and $R_1$ matches $S$ or $R_2$ matches $S$
- $R = R_1^*$ and $S = epsilon$ or $S = S_1 dot S_2$ and $R_1$ matches $S_1$ and $R_1^*$ matches $S_2$.

] <regex-semantics>


Finally, the most important fact that relates regular expression with the FSMs in  @fsm-section.

#statement[FSM and Regular Expression equivalence][
Given a language $L$, there exists a regular expression $R$ whose set of strings that match is $L$ if and only if there exists an FST $M$ such that the set of strings that $M$ accepts is $L$.

] <fsm-regex-equal>

In this case, $L$ is called a _regular_ language. 

Some various properties can be proved for regular languages. 

- $Sigma^*$ is regular
- If $L$ is finite, then $L$ is regular. 
- If $L$ is regular under an alphabet $Sigma$, then the complement of $L$ under $Sigma^*$ is also regular, i.e. the _inverse_ is regular. 
- If $L$ is regular, then the reverse of $L$ i.e. $ {"rev"(x) | x in L}$ where every string is reversed is also regular. 


Special attention may be paid towards @regular-closure, which will come up again in @fst-section.

#statement[Regular Closure properties][
 If $L_1$ and $L_2$ are regular, then $L_1 union L_2$, $L_1 inter L_2$ and $L_1 \\ L_2$ are all regular. 
] <regular-closure>

A natural questions to ask is: are all languages regular? The answer is no. The pumping lemma describes the exact conditions in which a language is regular, but for brevity, it will not be described here. The most important consequences 


#statement[Non Regular Example][
Let the alphabet be the binary digits, i.e. $Sigma = {0, 1}$ , and let the langauge $L = { 0^x 1^x: x in bb(N) }$. It can be shown that $L$ is not regular.
] <simple-non-regular>

This result can be generalized to parentheses to argue that many programming lanugages are _syntactically_ not regular. 


== Finite State Tranducers <fst-section>

The previously mentioned FSMs provide a convenient way of modeling phonology. Words can be defined as strings over some alphabet, such as IPA or simple ASCII characters. Under the assumption that natural language is finite, these languages would be finite. 

However, _sound laws_ describe not just how the languages are, but how they _change_. This motivates finite state transducers, which will be able to model the laws shown in @soundlaws. 



#definition[Finite State Transducer][
A Finite State Tranducer (FST or transducer for short) can be defined as  
 $(Q, s, Sigma, F, delta)$, where 

- $Q$ is the set of states
- $s in Q$ is the start state
- $Sigma$ is the _alphabet_, a set of characters
- $F subset Q$ the set of final states
- $delta: Q times (Sigma union { epsilon }) times (Sigma union { epsilon }) -> P(Q)$. The transition function

] <fst-definition>


In @fst-definition, the $delta$ function now takes multple characters from the alphabet. One can be viewed as the input, the other the output. 
Like an FSM, a convenient way of visualizing an FST is by viewing it as a graph. The vertices are $Q$ and the edges are  $(Sigma union { epsilon }) times (Sigma union { epsilon })$. The edges are commonly notated using the notation $u:v$, where $u$ is the input and $v$ is the output. 

A $delta^*$ function can be defined as similarly, with todo figure this out
a

Each FST has two projections called the input and output projections by restricting the $delta_("input")(q, s) = union.big_(s' in Sigma union {epsilon}) delta(q, s, s')$ function and likewise for the oupput function, which can be used to create FSMs. Due to this, an FST can be thought of as a relation between two languages, i.e. a subset of $Sigma^* times Sigma^*$. An input string is related to the output string if and only if the input FSM accepts the input and the output FSM accepts the output strings. 


Such relation is called a _regular_ relation. 

Some properties of FSMs can be generalized to FSts. Concatentation, union, and reversal have corresponding constructoins in FSTs. Additionally, a very useful operator on two FSTs is the _composition_ operator. Given regular relations $R_1, R_2$, a pair $(x, y)$ is in the composition $R_1 compose R_2$ if and only if there is some $z$ such that $(x, z) in R_1$ and $(z,y) in R_2$. In terms of sound laws, this corresponds with combining one sound law after another. 

Note that the input and output constructions were chosen arbitrarily to be left and right. Indeed, it does not matter which is chosen to be which because given a regular relation $R$, there is the isomorphic construction $R^(-1)$, which switches the input and the output. Although the names input and output are similar to the terminology in a function, in general, a regular relations does not need to be function, and can display non determinsitics behavior in the sense that one input could map to multiple outputs.  

It was noted in @regular-closure that for regular languages, the compliment, union and intersection were all closed.  The _compliment_ construction and _intersection_, on the other hand, of two regular relations will not necessarily be regular.  @kaplan1994regular 

Consider the relation $R_1 = { angle.l a^n, b^n c^* angle.r | n >= 0}$ and $R_2 = { angle.l a^n, b^* c^n angle.r  | n >= 0}$. The intersection of $R_1$ and $R_2$ is ${ angle.l a^n, b^n c^n angle.r | n >= 0 }$. Doing an output projection gives $ { b^n c^n  | n >= 0 }$, and as stated in @simple-non-regular,  this language is known to not be regular. 

== The replace operator <replace-operator-section>

Using FSTs, it is relatively trivial to create a transducer that corresponds to an unconditional sound laws. However, creating conditional sound laws is surprisingly non trivial. A sound law describing #lp.rule([x], [y], [a #lp.dash() b]) is not as easy as it seems. The seemingly obvious approach. todo: fill this



#cite(<kaplan1994regular>, form: "prose") discuss an approach to implement this behavior. One of the key insights with this approach is to use first split it into smaller stages that are then composed together. One stage inserts _context markers_ next to the left and right contexts (_a_ and _b_ in the example), which have a special symbol. A separate stage then transduces by doing the replace only between context markers. Finally, these markers are then removed in a separate stage.  Additionally, they describe potential usages for these 

#cite(<karttunen1997replace>, form: "prose") uses a similar approach, and uses syntax that is also used in HFST @hfst as shown in @four-orient. This approach introduces four different ways of potentially disambiguating sound laws rewrites, since they can be ambiguous. 

#figure(
 table(
   columns: 2,
   [Upper Oriented ], [`UPPER -> LOWER || LEFT _ RIGHT`],
   [Right Oriented],[ `UPPER -> LOWER // LEFT _ RIGHT`],
   [Left Oriented:], [`UPPER -> LOWER \\ LEFT _ RIGHT`],
   [Downward Oriented], [ `UPPER -> LOWER \/ LEFT _ RIGHT`],
 )
,caption: [Types of Replace operators]
) <four-orient>

The show the difference and where the ambiguity occurs, consider the example `a b -> x || a b _ a `, and applying it to the string `abababa`. This would yield `abxxa`, because the upper oriented requires that both left and right contexts match before it can transduce.  However, the right oriented would yield `abxaba` and the left oriented would yield `ababxa`. The right oriented version requires the the right context to match first before transducing, and the left context to match after transducing. Likewise, the left oriented version must have the left context match before transducing, and the right context must match after transducing. Finally, the downward oriented version is actually has two outputs: `abxaba` and `ababxa`, since they both satisfy the condition that the left and right contexts match after transducing. 





== Applications for fst

The earliest approach for this 

A common library is openfst @openfst.  Hfst @hfst uses openfst @openfst as a potential backend.

Hfst itself is used as tool for morphological analysis. This was described using the xerox tools described in  #cite(<beesley2003finite>, form: "prose") using the 



= Infastructure

== CAPR

This project was initially inspired by #cite(<capr>, form: "prose"), in which the CAPR system was described. It provides an innovative approach to using FSTs to model historical linguistics.

#cite(<capr>, form: "prose") was to create a architecture that ultimately a useful tool for the linguist. The linguist has data and theorizes that some group of languages are related. The linguist then has to come up with protoforms and sound laws that would fit the data. The tool does not come up with theories, rather the linguist comes up with the idea, and the system tests to what extent the theory holds.

Unlike other approaches, the CAPR system allows a system closer to the traditional pen and paper process by developing a theory incrementally: initial hypothesis may be incorrect, but will be gradually refined.

More concretely, the system is a web application where the linguist provides the set of data cognates. The linguist then provides the the sound laws by inputting a the sound laws in the form of transducers. The sound laws need to specify which languages the laws to apply to, since CAPR requires that there be at least two descendant languages when constructing proto forms. The sound laws get fed into a backend server that takes the HFST rules and turns them into transducers. The backend then looks at each cognate set and performs the backward transduction according to the sound laws for the respective language. As mentioned ins @fst-section, the backward transduction creates potentially multiple protowords. If the protowrod appears in multiple reverse transductions for different descendant languages, then this shows that the sound laws are valid for word, and the words is displayed to the linguist. 

The core idea of CAPR is very promising, however there were some limitations that motivated this project. 

First was the difficulty in the installation. Although there was a docker container available, it was nevertheless difficult to get running. This could have come from a variety of sources: potentially, a bug on the developer end, or an error in the user configuration. This will likely be a barrier for linguists who may not be as technologically inclined or willing to invest time on setup. 

Second, the dependence on HFST potentially introduces inconviences. Since rule compilation is handled server-side, the linguist must show competence by downloading the full project locally and building, or there has to be a server running somewhere to handle all the requests. This comes with additionaly infastructure and maintenance costs. A simpler, self-contained web app that runs in the browser would be a lot more convenient for linguists. 


Additionally, the user interface requires the linguist to learn HFST. Although not particularly difficult, the syntax is somewhat different from the typical hand written form. Also, the linguist works mostly in typing in the edit box, which, although functional, is not as interactive as other interfaces. 

Additionally, it requires a strict adherance to the Neogrammarian Hypthesis. A potentially better system can be imagined that would be more accomodating to failures of the Neogrammarian hypthesis. In this sense, a system that allows the strictness for the Neogrammarian Hypothesis to be strict when necessary, but also to relax the hypothesis when necessary.

Despite these drawbacks, the CAPR project makes a lot of headway by opening a new direction in the field of computational historical linguistics that seems quite understudied. The decision to put the emphasis on sound laws, indeed is still a great in comparison to the previous approaches in which older studies were lacking. 


== Project Desciption

This project can be viewed as an exploration of lightweight transduction in the spirit of CAPR. It's primary goal was to explore more user friendly approaches for sound law transduction using finite state transducers. The end result is unfortunately in a more prototype state than the original CAPR, some expreience was gained in the direction of sound law transduction, and could potentially be a starting block for other approaches in sound law transduction. 

Like the CAPR predecessor, the aim of this project was not create a system automatically contruct or predict word forms. Instead, focus lies in being a tool to the linguist, while not infringing on the "job" of the linguist. The linguist does what they always do, but with assistance from the computer in doing the boring, mundane parts. This means the linguist still comes up theory of the proto language and how it changes. 

Instead the tool has two important roles ti plays. In addition to doing fulfilling the idea of making the linguist's life easier  of transducing sound laws, the idea behind it is potentially more general: which is verification of ideas. Frequently, words will have etymology entries in dictionary's, but rarely is there any explicite verfication on this. In some cases such as for the dictionaries of proto languages, a list of sound changes may be supplied to discuss general the history of the language and potentially the sound changes that occurred for various daughter languages. This is relatively rare when discussing every day dictionaries, and the user is often assumed to just believe the entry. Rarer still is for the user to be able to see some intermediate sound changes, which is occasionally seen in some Wiktionary entries. By making the tool explicitely see sound changes, it forces the process to be transparent to the reader along with teh author. This can be seen as also proof reading the author. The author makes claims, but the tool produces the evidence.

Following these ideas, the first main goal was to create a minimum working version of a transduction system to be used for historical linguisti construction. Tghis should improve upon the CAPR predecessor by reducing barriers for those without technical background. It also should try to create a simpler to user UI and provide a simpler syntax that is more familiar than teh HFST rule syntax. To accomplish this, it was orginally envisioned to be a standalone, browser based applicatin requiring minimal setup. Whiel the technical goals were only partially achieved, the effort has left some expierence t o reflect on.

The core functionality is opened in Rust. Rust is a relatively new programming language that aims to be as fast as C or C++. The language does not have to deal with the 40 years of languages features that C and C++ have to suupport in order to be backwards compatible, and as a result, has a very modern exprerience when programming in it, while also not sacrificing in performance.  It achieves this high performance by using a unique memory management system that does not require a garbage collector, but uses compile time checks to prevent memory errors. 

The library was built using rustfst @rustfst, a rust port of the OpenFst @openfst that is a C++ transduccer library. While rustfst provides many low level FST operations, it lacks some higher level operations that are included in HFST. 

As a result, a considerable amount of development was devoted to reimplementing HFST functionality. The most important is the _replace operator_ as discussed in @replace-operator-section, which is required in expressing context sensitive rewrite rules. As discussed in @karttunen1997replace, the replace operator is decnetly complex. The currently implementation is based off the HFST source code, but Most importantly, the _complement_ operator is a crucial operator that is missing in rustfst. Of the four types mentioned in @four-orient, the project only implements the upper oriented replace operator. 








= Case Study Celtic


== Background Information

=== Celtic

Celtic is an Indo-European language family. This includes members such as Irish, Scottish, Welsh, and Gaulish. @celtic The language which these language descended from is called Proto-Celtic, because it has not been directly attested, and has been reconstructed from it's living descendants.  

=== Proto-Indo-European

Proto-Indo-European (PIE) is the reconstructed ancestor of many languages of Europe. Its descendants include Germanic Languages (English, German, etc), Slavic Languages (Russian, Polish, Ukranian), Romance Languages (Spanish, French, Italian, etc. ), and many languages of India. 

The grammar of PIE is reconstructed to be complex. The scope is quite large. It has a vast history and to go into full detail would take too long. Only the most relavent information will be covered here. 

PIE was heavily declined. A root with a meaning could not be used by itself, but always had to be declined according its case. There were two broad declension paradigms used: the thematic and athematic declension paradigms.


== The Celtic Dictionary

The case study examines a Celtic dictionary by #cite(<celtic>, form: "prose"). The dictionary begins with a short introduction, followed by a description of sound laws, and then finally the rest of the book consists of Proto-Celtic dictionary entries.

To test the validity of the Celtic dictionary.

=== Sound Laws

// A > B \/ X #box(width: 0.5cm, clip: true, outset: (y: 1em), underline(extent: 1000cm, sym.space.nobreak)) Y 

#let law(A, B, X, Y) = [#A > #B \/ #X #box(width: 0.5cm, clip: true, outset: (y: 1em), underline(extent: 1000cm, sym.space.nobreak)) #Y ]


// #law([a],[b],[c],[d])
Given is a brief overview of the sound laws presented in the dictionary. They describe the changes from PIE to Proto-Celtic.


#let C = lp.fmat([consonant])
#let V = lp.fmat([vowel])
#let H = lp.fmat([laryngeal])

#figure(
  [
    + #lp.rule([h₁e], [e]), #lp.rule([h₂e], [a]), #lp.rule([h₃e], [o])
    + #lp.rule([eh₁], [ē]), #lp.rule([eh₂], [ā]), #lp.rule([eh₃], [ō])
    + #lp.rule(lp.fmat[laryngeal], [a], [#C #lp.dash() #lp.fmat([consonant]) ])
    + #lp.rule[#lp.fmat[laryngeal]][#sym.emptyset][#lp.fmat[stop] #lp.dash() #lp.fmat[stop]]
    + #lp.rule[#lp.fmat[coronal][stop]#lp.fmat[coronal][stop]][s s ]
    + #lp.rule[#sym.emptyset][a][#lp.fmat[consonant] #lp.fmat[resonant] #lp.dash() #lp.fmat[laryngeal] #lp.fmat[resonant] ]
    + #lp.rule[#H][#sym.emptyset][#V #lp.dash() #lp.fmat[laryngeal] #lp.fmat[resonant] ] // lookup how they did verners law in the book
    
  ],
  caption:[Dialectical Indo-European Changes]
)

=== Results

Once these sound laws were formalized and turned into FSTs, a set of PIE etyma were then selected as test cases to see if the transduced result was the expected Proto-Celtic form. These words were more or less selected arbitrarily. Many words also required some slight modification, due to the Proto-Celtic entry containing a vowel as part of the declension, when the PIE root mentioned did not have it. Some words were also modified due to them coming from a different ablaut grade than the cited PIE root, or had a different suffix appended to it.
 @transducetable shows the results. 



#show "true": [#emoji.checkmark.box]
#show "false": [❌]

#show figure: set block(breakable: true)

#figure(
 table(
  columns: 5,
  table.header[*Pie Stem*][*Transduced Result*][*Proto-Celtic Root*][*Correct*][*Notes*],
  [ h₃eyno], [oyno], [oyno], [true], [],
  [bʰero], [bero], [bero], [true], [],
  [bʰrso], [barso], [barso], [true], [],
  [deqno], [daːno], [daːno], [true], [],
  [dnt], [dant], [dant], [true], [original danto],
  [dʰɡʰesi], [xɡesi], [ɡdesi], [false], [metathesis],
  [dʰɡʰo:m], [xɡaːm], [ɡdon], [false], [metathesis],
  [geh₂r], [ɡaːr], [ɡaːr], [true], [original ga:ri],
  [genh₁os], [ɡenos], [ɡenos], [true], [],
  [genu], [ɡenu], [ɡenu], [true], [],
  [grh₁no], [ɡraːno], [ɡraːno], [true], [],
  [gʰyemo], [ɡyemo], [ɡyemo], [true], [],
  [h₁epirom], [efirom], [efirom], [true], [],
  [h₁ludʰ], [lud], [lud], [true], [],
  [h₁re:g], [riːɡ], [riːɡ], [true], [],
  [h₁rewdʰ], [rowd], [rowd], [true], [original rowdo],
  [h₁su], [su], [su], [true], [],
  [h₂mlgto], [amlixto], [mlixto], [false], [Incorrectly feeds _a_ insertion],
  [h₂ste:r], [stiːr], [steraː], [false], [Incorrect vowel and extra a],
  [h₂weh₁nto], [winto], [winto], [true], [],
  [h₂weh₁nto], [winto], [winto], [true], [],
  [h₃ektoh₁], [oxto], [oxtuː], [false], [Incorrect vowel],
  [kapr], [kabr], [ɡabro], [false], [initial consonant is irregular],
  [kewh₂ro], [kowaro], [kawaro], [false], [Incorrect vowel],
  [kleh₂ro], [klaːro], [klaːro], [true], [],
  [klewos], [klowos], [kluwos], [false], [Incorrect Vowel],
  [klh₁eto], [kleto], [kaleto], [false], [_a_ inserted],
  [kmti], [kanti], [kanti], [true], [],
  [krd], [krid], [krid], [true], [],
  [krdtu], [krissu], [krissu], [true], [],
  [krewh₂], [krow], [kruː], [false], [Incorrect vowel],
  [kwo:n], [kwaːn], [kʷon], [false], [Incorrect vowel],
  [kʷendʰ], [kʷend], [kʷend], [true], [so element at the end seems to be an addition kwendso],
  [kʷetwores], [kʷetwores], [kʷetwores], [true], [],
  [kʷey], [kʷeː], [kʷeː], [true], [original kwe:s _s_ includes the nominative ending],
  [kʷid], [kʷid], [kʷid], [true], [],
  [legʰ], [leɡ], [leɡ], [true], [original lego/ legyo],
  [lewko], [lowko], [lowko], [true], [],
  [leyd], [leːd], [loydo], [false], [incorrect vowel, may be from _o_ grade],
  [linkʷ], [linkʷ], [linkʷ], [true], [original linkʷo original leykʷ, but n infix added],
  [medʰyo], [medyo], [medyo], [true], [],
  [mel], [mel], [mall], [false], [Does not account for double consonant],
  [meh₂k], [maːk], [mak], [false], [original mako, incorrect vowel length],
  [meh₂te:r], [maːtiːr], [maːtiː], [false], [Somehow the _r_ is dropped],
  [mrgʷto], [mrixto], [mrixto], [true], [],
  [nebʰos], [nebos], [nemos], [false], [Alternation between _m_ and _b_ ],
  [nem], [nem], [nem], [true], [original nemo],
  [newos], [nowos], [nowyo], [false], [],
  [newyo], [nowyo], [nowyo], [true], [],
  [neh₂u], [naːu], [naːwaː], [false], [Due to switching from _u_ to an _a_ declension paradigm],
  [nokʷt], [noxt], [noxt], [true], [],
  [pelh₁u], [felu], [filu], [false], [says it preserves e grade, but don't know where i comes from],
  [ph₁te:r], [fatiːr], [fatiːr], [true], [],
  [piprqse], [fibraːse], [fibrase], [false], [Vowel length],
  [plew], [flow], [flow], [true], [],
  [plh₁no], [flaːno], [flaːno], [true], [],
  [potr], [fotr], [fatar], [false], [cannot account for a vocalism],
  [prptu], [brixtu], [frixtu], [false], [Wrong initial],
  [prptu], [frixtu], [frixtu], [true], [],
  [re:g], [riːɡ], [riːɡ], [true], [],
  [re:gnih₂], [riːɡni], [riːɡaniː], [false], [Extra _a_ epenthesis],
  [reyd], [reːd], [reːd], [true], [],
  [reydʰ], [reːd], [reːd], [true], [original re:do],
  [reyh₁], [reː], [reːno], [false], [_no_ suffix],
  [sed], [sed], [sed], [true], [original sedo],
  [seh₂l], [saːl], [salano], [false], [Incorrect vowel length along with _no_ suffix],
  [seh₂t], [saːt], [saːt], [true], [original sa:ti],
  [sekʷ], [sekʷ], [skʷetlo], [false], [Some sort of metathesis with _lo_ suffix],
  [seno], [seno], [seno], [true], [],
  [sent], [sent], [sent], [true], [original sentu],
  [septm], [sextm], [sextam], [false], [Fails to append add an _a_],
  [seh₂g], [saːɡ], [saɡyo], [false], [Wrong vowel length and _yo_ suffix],
  [sih₂mdo], [siando], [sindo], [false], [Law incorrectly feeds],
  [siskʷo], [siskʷo], [sisku], [false], [Failed to delabialize],
  [skeh₃t], [skaːt], [skaːt], [true], [original ska:to],
  [skribʰ], [skrib], [skriːbbaː], [false], [],
  [slunk], [slunk], [slunko], [false], [original slewk],
  [smh₂el], [smal], [samali], [false], [],
  [sneh₂], [snaː], [snaː], [true], [],
  [sney], [sneːb], [sniɡʷ], [false], [Incorrect feeding rule],
  [soru], [soru], [serwaː], [false], [Incorrect vowel],
  [spelh₁ɡʰ], [sfelaɡ], [sfelɡaː], [false], [Metathesis of vowel and consonant],
  [sperh₂g], [sferaɡ], [sfraxto], [false], [Metathesis and _to_ suffix],
  [srewm], [srowm], [srowman], [false], [n suffixed onto end],
  [sterk], [sterk], [stronko], [false], [_o_ grade and metatesis],
  [steh₂], [staː], [sista], [false], [],
  [stomn], [stomn], [stamnaː], [false], [Wrong vowel with long a at end],
  [supno], [sowno], [sowno], [true], [],
  [sweh₂du], [swaːdu], [swaːdu], [true], [],
  [sh₂l], [sal], [sal], [true], [original saltro],
  [tenh₁], [ten], [torano], [false], [A suffix of ro followed by a metathesis has been applied],
  [terh₁tro], [teratro], [taratro], [false], [Spurious _e_ to _a_],
  [treyes], [treːes], [triːs], [false], [Multiple vowels combining],
  [tuh], [tu], [tu], [true], [],
  [uper], [ufer], [ufor], [false], [Incorrect vowel, could be from _o_ grade],
  [widʰu], [widu], [widu], [true], [],
  [wirh₁o], [wiro], [wiro], [true], [],
  [wlh₁o], [walo], [walo], [true], [],
  [yemh₁o], [yemo], [yemono], [false], [_no_ suffix],
  [ɡʰeh₂ns], [ɡans], [ɡans], [true], [original gansi],
  [ɡʰelh₃], [ɡel], [ɡel], [true], [original gelo],
  [ɡʷow], [bow], [bow], [true], [],
),
caption: [Transduction table]
) <transducetable>




== Discussion

#bibliography(("bib.bib", "bib.yaml"))








