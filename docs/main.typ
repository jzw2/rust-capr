#import "@preview/linphon:0.1.0" as lp
#import "@preview/ascii-ipa:2.0.0": *

// #import "@preview/casson-uom-thesis:0.1.1": *
#import "template.typ" : *


#import "@preview/finite:0.5.0": automaton
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

#show "todo": set text(fill: red)



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


= Historical Linguistics <linguistics-section>


Language is not static: languages change.  Historical Linguistics is the study of change in language. @hock2021principles

Languages may change in their syntax, along with semantics, but the area most suited for study is the phonology changes. 

The living pieces of evidence that language changes is simply examining the English language. The English language is notoriously known for its frequently arbitrary spelling rules, that is, its _orthography_.  @silentk shows a list of words that do not have the _k_ sound, yet are spelled as if it began with a _k_. 


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


Learners of English orthography may have just accepted this as a fact of the English language. Some spellings just make no sense. However, it is possible to postulate reasons for such a spelling.  The most compelling is due to language change.  English spelling often reflects earlier stage of the language, where things were pronounced differently. The spoken language changed, but the written language had failed to update to the newer pronunciation. The orthography an been seen as a fossil of the earlier stage. 

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

Also, note that not only has the word initial \<\kn\> cluster been simplified in the examples of @silentk, but it has disappeared throughout the entire language. In fact, native English speakers may even have trouble pronouncing the cluster in isolation. This is because the change has effected the entire lexicon, leaving not a single exception behind, meaning that the change was systematic, or _regular_. 

This change was only one example of many that occurred in the history of the English language. English is not the only language that has changed, every living language has evolved from an earlier state. 


== Neogrammarian Hypothesis


As a child learning how to read English, seeing the spelling of the words listed in @silentk might be generalized into a rule: \<kn\> gets pronounced as \<n\>. This generalization parallels the development of the development of English, so what initially seems to be a pronunciation rule may be more accurately described as a _sound change rule_ or a _sound law_, a term used by the Neogrammarians, a group of German linguists from the 19th century. 

Additionally, this particular sound law is observable because of the conservative orthography of English. But English has only (relativly speaking) recently been written down. If, for instance, English and German had written down their language back when they were one language, would it be possible for a learner of the language to similarly form a large list of rules to derive the English pronunciation, and a different set of rules to derive the German pronunciation?

The Neogrammarians had a similar thought, and believed not only that English and German could be derived from a common ancestor like this, but that they could project even further back, relating most language of Europe along with many languages of India. These languages included English, German, French, Spanish, Greek, Russian, Hindi, and more. This is similar to Darwinian evolution. An ancient species, after millions of generations, can diversify into many different modern species. Similarly, many languages can be thought of as the descendants of an ancient language. Just as DNA mutations drive the change in species, sound laws drive the change in languages. 

The oldest literarly languages known to Neogrammarians: Latin, Greek, and Sanskrit, not only had correspondences in vocabulary, but also similarities in Grammar. This led them to postulate the existance of the Proto-Indo-European (PIE) language, which then diversified and split into various daughter languages.  Similar to the example of English and German, often one language might have had a sound law apply, but another language might have preserved a more archaic form.   @hock2021principles 

The Neogrammarians created these sound laws under the Neogrammarian Hypothesis, as shown in @neogrammarian

#definition[Neogrammarian Hypothesis][Sound Laws are regular] <neogrammarian>

In this, they tried to bring linguistics in line with the the laws of the natural world. Just as the natural world is modeled by the language of mathematics, linguistics may be modeled by sound laws.  Each sound law corresponds with a function from words to words. The full history of a language can be thought of a composition of these small functions. 


=== Limitations



The Neogrammarian Hypothesis sought to make linguistics a science by emphasizing predictability and regularity.  Like any science, however, the actual data will not be completely deterministic from the model.   They believed what exception that they encountered were merely sound laws that had been not sufficiently generalized.


However this is probably not a very realistic model of language. Unlike the physical world, which has laws written in the language of mathematics that will predict exactly what will happen, languages are full of arbitrariness.

There are examples of changes that are _sporadic_ and effect only one word. Other changes may display _nondeterministic behavior_ , where a law has seemingly applied in a context for one word, but does not apply in another word which has the same context. This may even happen when these two words are the exact same, resulting in two descendants. For instance, the word _curse_ has the descendant _cuss_ which deleted the final \<r\>, while also preserving the unchanged form _curse_.  Borrowings further complicate the issue, since they first follow the sound laws it was borrowed from, and then follows the sound laws it was borrowed to. 



== Sound Laws <soundlaws>

=== Definitions <soundlaws-definitions-section>

Linguists have a conventionalized representation for sound laws.

The most basic change is the arrow: $A > B$ where $A$ is a phoneme that gets changed, and $B$ is the phoneme that it gets changed into. Another common notation is shown in @contextfree.  It cal be chained such as  $A > B > C$, which can be viewed as a short hand of two laws, $A > B$ and $B > C$. 



#figure(lp.rule([A], [B])) <contextfree>


@contextfree represents a sound law that happens regardless of the context it appears in. An example would be *whine-wine* merger, which can be formerly notated as @whinewine

#figure(lp.rule([ʍ], [w]), caption: [The Whine Wine merger]) <whinewine>


However, it is more common that sounds change in only restricted contexts. This is exemplified in @withcontext
@crowley2010introduction

#figure(lp.rule([A], [B], [X #lp.dash() Y]), caption: [Sound Law with context]) <withcontext>


where the variables $A, B, X, Y$ can represent strings, or potential features. The dash can be viewed as a "hole" in which the change happens. The variables $X, Y$ represent the left hand side and right hand side contexts. These contexts frequently represent single phonemes, but could also represent classes of phonemes, or multiple phonemes concatenated together. 

Phonetic features sometimes allow for a more succinct pattern in expressing a sound law, and may also reveal a more general pattern. For instance, Grimm's law is a law that happened in Germanic languages. It is why the stop series in Germanic frequently do not match with the languages in the PIE family. Written out completely, it is a sound change encompasses 12 smaller sound laws as shown in @grimm


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

It is important to note that this a rough convention, and lacks a full, formal definition. The notation is inadequate to describe all types of sound law change. Authors will usually need to supplement this notation with English language explanations.  However, for simple cases, it is possible to describe a sound change unambiguously. 

=== Ordering

The laws mentioned in @soundlaws-definitions-section are each relatively simple to describe. However, a sound law does not just occur by itself. In the history of a single language, there are often many sound laws that have occurred.  These sound laws can interact with each other, which forces an ordering onto the sound laws. This is called the _relative chronology_. @hock2021principles 

There are two important terms that describe certain orders:

-  _Feeding_: This is when the application of one sound law causes a second one to apply. 
- _Bleeding_: This is when application of a first law causes the second to _not_ apply, that would have otherwise applied. 




As an example, the Great Vowel Shift in English included the law shown in @great-vowel-shift-law. 

#figure(
  lp.rule([a], [æ]),
  caption: [Great Vowel Shift for /a/]
) <great-vowel-shift-law>

Much later, the, /æ/ fronting occurred in (most dialects of American English) with the law shown in @ae-tensing, further changed the /æ/ vowel before nasals.



#figure(
  lp.rule([æ],  [ɛə], [#lp.dash() [+nasal]]),
  caption: [American /æ/ tensing ]
) <ae-tensing>

The relation between these two laws is a feeding relation, causing the phoneme /a/ to change due to @great-vowel-shift-law and then again from @ae-tensing. Thus, the word \<sand\> is pronounced [sɛənd].  If the ordering were reversed, the result would be different.  @ae-tensing would not apply and only @great-vowel-shift-law would,  making the outcome [sænd].

Feeding and bleeding make the historical linguist's job a bit more complicated. Not only must they provide proto forms and sound laws, but the order must be correct too. Also, when adding hypothesizing a new sound law, the linguist must be careful that adding a new sound law does not incorrectly feed or bleed words that were previously correct. 


== Computational Linguistics

At first glance, the formal notation looks like a perfect fit for a computer program. Indeed, it is not hard to write a computer program that, given a set of words, and the sound laws that are known, to then write a computer program that transduces this. 

There are many benefits of this modeling the sound laws with a computer. First, it provides _verification_ of the sound laws. Instead of relying solely on the linguist, who may make mistakes, the computer functions as a second eye to see whether the laws produce the expected outcomes.  As the amount of sound changes is usually non trivial, it is not hard to accidentally forget to apply a law, or apply it in the wrong order.  A computational model will always computer the laws correctly and in the correct order, revealing inconsistencies between the laws the linguist proposes and the actual data. 

In this light, the job of the linguist is like the job of a programmer. The sound laws are lines of code to be executed. The proto language is the input. The descendant language is the output. And the individual words are test cases. 

It is ironic that these sorts of explorations have not been popular with linguists. In fact, in terms of technology, the linguistic community has been relatively slow in adopting computer usage. For instance, 
it was discovered that linguists had been using a word processor and search functions to discover sound laws. @sims2018mechanising This very primitive method could have been automated by a very simple program, and yet was used to discover a new sound law. Each transduction was more or less done by hand, except each phoneme was assisted using the word processor's Control-F functionality. 

In recent usage of historical linguistics 
more attention has been drawn to different aspects of computational linguistics, such as _phylogenetics_: quantifying how related languages or language families are. Much methodology in this area was inspired by algorithms in evolutionary biology.  Another are is _cognate detection_: given a corpus of languages, can it be used to detect find potential words that are cognates. @sims2018mechanising  Only recently has there been approaches @capr that return back to the Neogrammarian hypothesis created a hundred years ago. 



== Reframing the Neogrammarian hypothesis


The Neogrammarian hypothesis: _all sound laws are regular_ is a strong statement. Taken literally, the claim is unfortunately hard to uphold, once a little time is spent working with sound laws. Language is full of _irregularity_. There are very many ways in which sound laws will give the incorrect result when applying the sound laws without some leeway for correction.

The Neogrammarians were had already known that _analogy_ was a common source of irregularities. An example of _analogy_ is found in _paradigm leveling_. English has leveled its verb conjugation for most verbs, only adding the _s_ to the third person singular:

+ _I eat_
+ _You eat_
+ _We eat_
+ _They eat_
+ _He *eats*_

But the verb _to be_ has resisted this change just for the first person singular: _They are_, _We are_, but _*I am*_ instead of the incorrect _I are_. 

In Old English, there would have been a different verb form for all verbs in the first person singular. However, through paradigm leveling, many of the verb forms have now been "simplified" are now just the stem. If the sound laws were applied to these word form directly, without taking analogy into account, it would likely create many different outcomes instead of the ones that actually occur in modern English. 


Borrowing also creates some potential problems with the Neogrammarian hypothesis. When a word is borrowed, it lacks the sound laws that would otherwise apply to the rest of the lexicon. For instance the German word _Computer_ is borrowed from the English word _computer_ which looks like it has had no sound laws applied to it, by passing other correspondence for native vocabulary. 

Loan words can be especially troublesome with respect to the presence of _doublets_, which are words that have the same origin that coexist. In Romance languages, the most of the core vocabulary comes from Latin. In the natural process of time, it has undergone various sound changes. However, due to Latin being used as a prestige language, the original Latin word has been reborrowed along with the inherited word. An example is French _hotel_ and _hospital_ stemming from the same original Latin word _hospitalis_, both of which were then borrowed into English.  Any sound change system that tries to mjodel this would require two different sound law paths. One which would require the inherited sound laws, another which would require less sound laws, that is more direct from the Latin. 

Japanese provides another instructive case. Similar to Latin in the west, Chinese was a literary language used by the elites in Japan. 
As a result, Chinese words were continuously borrowed throughout the entire history. A Chinese character not as transparent about its pronunciation as a Latin word, so the pronunciation was borrowed from various dialects and at different periods. As a result, the same Kanji would have multiple readings depending on the compound that it was in and how it was pronounced in the dialect it was borrowed from.  

Even accounting for analogy and loans, sometimes sound changes do not have a predictable outcome. For instance, Mandarin Chinese is well known for not having stop codas, which used to constitute a tonal group. Due to Mandarin's phonology, where every syllable has to have a tone, the stop codas were distributed to other tones. However, the mapping from coda to tone is not completely predictable. This is exemplified in the words 不， and 一，which had previously both ended in /t/. Their defaults tones are fourth tone and first tone, but depending on context, may switch to second tone and fourth tone. Thus, creating an algorithm to predict the outcome tone would sometimes be non deterministic. 

// todo maybe give an example for this
All of these exceptions pose challenges to the Neogrammarian Hypothesis. They imply that even if a program were designed so that all sound laws were applied faithfully, it would still fail in certain cases due to analogy, borrowing and sporadic changes. Due to the potentially large engineering feat that may be required, in additional to numerous exceptional cases, it is likely the apparent effort in modeling both the sound changes and their exceptions is too high.  This explains why many explorations into computational methods for historical linguistics have focused more tasks which require less supervision. They commonly use a general machine learning style workflow: data aggregations, train a model, then evaluate on human annotated work. This way, a machine learning model can be adapted from another field without a human having to learn the nuances of each particular language. 

In contrast, building an algorithm by hand requires programming ability and linguistic expertise. Each case needs to be looked over, and the user must be able to debug when things do not go as expected.  

Despite these problems, there is an advantage to using the Neogrammarian hypothesis. Its easy to mistake the strictness of the Neogrammarian hypothesis as a weakness, but it is instead be viewed as a strength. When a sound law fails to account for a form, there is reason for it. Sometimes it is due other phenomena such as analogy, borrowing or sporadic change. But sometimes it is because a theory needs to be refined. In this, it is parallel to other sciences. Theories can be wrong and fail, but they will have a reasoning as to why it failed. Contrast this to machine learning models, which are generally black box models. 

A common example is Grimm's law. The Latin word for _have_ is _habeo_. On the surface, these two words have similar semantic meaning and phonetic structure. It might be logical to conclude that these two words are cognate. However, this fails to obey Grimm's law, which says that a germanic /h/ should be correspond with a Latin /k/. Indeed, the two are not cognates, but are just similar by coincidence. The true cognate of English _have_ is Latin _capio_, which has a meaning of take. Grimm's law rules out the false cognate, and provides a path to a correct one. 

Without such laws, it is too easy to assign these two as cognates. And this also explains the hesitancy of traditional historical linguistics in adopting computer based approaches. A system that uses a machine learning like approach will require a simpler model of language, meaning a full sound law system is too complicated to adopt requiring simplifications such as context free changes. @bouchard2013automated
//cite a paper to give examples todo

Machine learning models have been popular in computational linguistics partially because of similar work being done in computational biology. Language change can also be thought of as _mutations_ similar to how DNA changes. These changes are essentially a model where all changes are sporadic. While this model may have been a valid assumption for DNA, but explicitly goes again the Neogrammarian hypothesis on regularity. Furthermore, purely statistical model often produce reconstructions that are opaque to linguists. Even if it correctly predicts the output, the reasoning may not be clear. In contrast to biology, in which changes are truly random, linguists need to justify their theories. 

The strength of the Neogrammarian hypothesis is its explicitness. Rules can be checked and revised, and exceptions can be categorized and explained.  
Linguists need to build upon previous theories. An automatic system that has a theory without explanation requires the linguist to give that explanation. In this sense, the linguist must then reverse engineer the system's work, duplicating effort.  


It could be better summarized that the language itself is in general not completely regular, but that a model with higher regularity is easier for linguists to work with. A linguist will find it easier to model rules that are regular, because this can be viewed as evidence. Under this assumption, we can also view how regular a theory is. For linguist, seeing where the sound law succeeds is just as important as seeing where it fails, and why it fails. 

The original neogrammarian hypothesis can be revised to @revised-neogrammarian. 

#definition[Revised Neogrammarian Hypothesis][
  A sound law's usefulness is proportional to its regularity. 
] <revised-neogrammarian>


By relaxing the condition, some of the exceptional cases can be accounted for. Instead of viewing them as exceptions that break the model, they are just viewed as a less useful sound laws, since they represent laws that can apply, but potentially do not. To account for this, any algorithm that models sound change will have to incorporate _non determinism_. Instead of saying that language change is a function that maps a word to a new word, it can be thought of as a function that maps a word to multiple words. 

This new adaptation allows sound laws to be modeled computationally in a transparent way to traditional linguistics.  By putting the focus back on sound laws, the focus of computational linguistics becomes realigned with traditional historical linguistics. 



The biological analogy of evolution only partially corresponds to languages. In biology, DNA has to be passed from a parent to a child. This ignores the geographical nature of linguistics, in which languages share features due them simply being in the same area as other language. As supposed to the biological model, known as the _tree model_, the _wave model_ linguistics suggest sound changes effect a geographical area, as opposed to assigning a parent and descendent. Previous computational historical linguistic work frequently assumes the tree model. The beauty of sound laws is that they do not depend on a tree or wave model: both can be modeled as just composition of sound laws. 

== Previous Study of Sound Changes 

Despite their traditional role in historical linguistics, sound laws have received surprisingly little systematic study in computerized format. Explicit algorithmic changes are relatively rare. 

One example of using computerized sound laws was explored by John Hewson in the 1970s. After finding the corresponding sound laws by the Algonquin languages, he encoded these laws into a program. He then used the program to reverse the laws, taking as input words from the descendant languages and then outputting a proto Algonquian word. This program was then used to publish a book for dictionary of proto Algonquian. @sims2018mechanising @hewson1993computer

In contrast, the constructed language (conlang) community seems to have taken a greater interest in the study of sound changes in comparison to historical linguistics. For instance, the Index Diachronica is a compilation of sound laws attested in natural languages around the world. @diachronica Its purpose is to provide conlangers with a reference for realistic sound changes to adapt for their artificial languages. 

However, the Diachronica is not a very formal piece of writing, in the sense that a computer program can be automatically made from it. It is essentially a list, that likely requires a human to make inferences. 


Conlangers have further developed a tool for forward applications, for example with the Sound Change applier by Zompist. @zompist These allow users to input a proto-form and observe the output under a set of sound laws, verifying that their sound laws produce consistent results. This provides forward applications, but it does not provide backward application. This is suitable when the language is artificial like with the conlang community, but the backwards application is of more practical use for linguists. 


Most recently is Capr @capr, which will be discussed more detail in @capr-section. Capr is a program that uses FSTs to encode the sound laws that happened in a language. The advantage of FSTs is that they are bidirectional: encoding a sound law from one proto language to a descendent gives the reverse direction for  free.  The linguist encodes the sound laws via the XFST tool syntax, which gets passed to HFST. 

Otherwise, there have been various ad hoc transducers, indeed there has also been various methods also using FSTs such as rsca @rsca , the reversable sound change applier, which was a C++ program that used FSTs to apply sound laws. Due to the FST nature, it naturally allows for both forward application and backward applications. However, it appears this was created independent of the XFST family tool chains, and therefore has different syntax. 



 = Finite State Trandsducers

 In this chapter, Finite State Transducers (FSTs) are introduced. However, in order to define FSTs and motivate their usefulness, a some background information is required. FSTs can be viewed as a generalization of Finite State Automata (FSAs), and have important properties about their usage. Only the most important results will be listed here, and will be presented without proof. 

Much of this chapter is adapted from #cite(<kaplan1994regular>, form: "prose") . 

 == Finite State Automata

 Finite state automata (or finite state machine) are a mathematical model basic form of automata that can be used ot model a variety of phenomena. The most relevant usage is in string recognition.

 

 === Preliminary Definitions
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
By convention, strings are usually put in double quotes and the $dot$ operator commonly ommited. For example, the strings "foo", and "bar" are strings under the Latin alphabet. The string "Ελληνικά" is a string under the Greek alphabet. And "129893" is a string under the Arabic numerals. A string such as "abc" is the shorthand for full $"a" dot ("b" dot ("c" dot epsilon))$.


Now that strings have been formally defined, @language is a formal definition of _languages_.


#definition[Language][
A formal _language_ is set of strings. 
] <language>

Note that @language does not specify that the set must be finite, and under this defintion, there are languages that have an infinite amount of strings. 

@language be used to define natural languages such as English, Spanish, and German but can also be used to define more artifical lanuages, such as all prime numbers represented in decimal. 

As an abuse of notation, the $dot$ operator can also be extended to languages as shown in @concat-strings

#definition[Concatentation of Strings][
  Let $L_1$ and $L_2$ be languages. Then $L_1 dot L_2$ is defined as 
  $ { x dot y | x in L_1, y in L_2} $
] <concat-strings>


Finally, the equation the Kleene Star is also another operator

#definition[Kleene Star][
  Let $L$ be a language. Then the kleene closure  $L^*$ can defined as the smallest set such that the following all hold

  - $epsilon in L^*$
  - $L dot L^* subset L^*$

] <kleene-star-def>

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

When the amount of states in a finite state machine is small, it  can be convenient to represent the FST as a graph. The states will be represented by the vertices of the graph, and the edges will be represented by the $delta$ function, with a label for each character. A common convetion for representing the final states is by drawing those staes in a double circle.


As example, @example-dfa shows a machine under the alphabet $Sigma = {a, b}$ using this graph representation. It accepts a string if and only if the amount of "a"s in the strings is even. 

#figure(

automaton(
  (
    q0:       (q1: "a", q0: "b"),
    q1:       (q0: "a", q1: "b"),
  ),
  initial: "q0",
  final: ("q0",),
),

  
  caption: [Example Automaton]
) <example-dfa>

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

Every $delta$ can be extended to $delta^* : Q times Sigma^* -> P(Q) $ function by requiring 

- $delta^*(q, epsilon) = delta(q, epsilon)$
-  $ delta^*(q, c dot S) = union.big_(q' in delta(q, c)) delta^*(q', S) $  for each $c in Sigma union {epsilon }, S in Sigma^*$


Finally, an NFA _accepts_ a string $S$ if and only if $delta^*(s, S) inter F eq.not emptyset$. 

// A finite state machine $(Q, s, Sigma, F, delta)$ accepts a string $S$ when one of the following conditions are met:

// - $S = epsilon$ and $s in F$ 
// - $S = c dot S'$ and there exists a $q$ such that $q' in delta(q, c)$ and $M' = (Q, q', Sigma, F, delta)$ accepts $S'$

A deterministic finite state machine is trivially a non deterministic finite state machine. However, it may be surprising to know the converse is also true: any non deterministic finite state machine can be turned into a deterministic finite state machine which accepts the same language, stated in @nfs-equals-dfa.

#statement[Deterministic and Nondeterministic Finite State Machine Equivalence][
  A language $L$ can be recognized by a a deterministic finite state machine $F$ if and only if there exists there exists a nondeterministic finite state machine $F'$ that recognizes $L$.
] <nfs-equals-dfa>



 Due to @nfs-equals-dfa, it usually does not matter whether it is called a deterministic finite state machine or a non deterministic finite state machine, due the set of languages they recognize being the same. In general, they can be referred as (FSM) finite state machines.

 This can often simplify presentations in FSTs, due non deterministic machines often requiring less states.
 For instance, @example-nfa-end-aa is an nondeterministic machine that accepts strings that have a second to last character of "b".

 
#figure(

automaton(
  (
    q0:       (q1: "b", q0: "a, b"),
    q1:       (q2: "a, b"),
  ),
  initial: "q0",
  final: ("q2",),
),

  
  caption: [A Non Deterministic Machine]
) <example-nfa-end-aa>

@example-dfa-end-aa2 is an equivalent deterministic machine. It requires one more state and many more edges than @example-nfa-end-aa.

#figure(

automaton(
  (
    q0:       (q1: "b", q0: "a"),
    q1:       (q2: "a", q3: "b"),
    q2: (q0: "a", q1: "b"),
    q3: (q2: "a", q3: "b"),
  ),
  initial: "q0",
  final: ("q2", "q3"),
),

  
  caption: [The equivalent deterministic machine]
) <example-dfa-end-aa2>




When dealing with the theoretical properties of FSTs, whether the underlying machine is deterministic or nondeterministic can be relegated as an implementation detail. However, when dealing with real systems, the difference is more important, as determinizing a finite state machine can have a worst case performance of $O(2^n)$.


== Elementary Results in FSMs

=== Regular Expression


Many programming languages have some sort of library for _regexes_, often used for string processing. The term _regex_ is a shortening of the term _regular expression_, which @regular-expression defines. 



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
- If $L$ is regular, then the reverse of $L$ i.e. $ {"rev"(x) | x in L}$ where every string is reversed is also regular. 


Special attention may be paid towards @regular-closure, which will come up again in @fst-section.

Recall the definition of $dot$ from @concat-strings and the kleene star from @kleene-star-def

#statement[Regular Closure properties][
 If $L_1$ and $L_2$ are regular, then $L_1 union L_2$, $L_1 inter L_2$, $L_1 \\ L_2$ and $L_1 dot L_2$ are all regular. 

 Also, if $L$ is regular, then $L^*$ is regular. 
] <regular-closure>

A natural questions to ask is: are all languages regular? The answer is no. The pumping lemma describes the exact conditions in which a language is regular, but for brevity, it will not be described here. The most important example relevant to discussion is given in @simple-non-regular. 


#statement[Non Regular Example][
Let the alphabet be the binary digits, i.e. $Sigma = {0, 1}$ , and let the language $L = { 0^x 1^x: x in bb(N) }$. It can be shown that $L$ is not regular.
] <simple-non-regular>

This result can be generalized to alphabets beyond the binary digits. From this, it can be argued that many programming languages (which require parentheses to be matched like the example above) are _syntactically_ not regular. 


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

A $delta^*$ function can be defined as similarly, with the $delta^* : Q times Sigma^* times Sigma^* -> P(Q) $ function by requiring 

- $delta^*(q, epsilon, epsilon) = delta(q, epsilon, epsilon)$
-  $ delta^*(q, c_1 dot S_1, c_2 dot S_2) = union.big_(q' in delta(q, c)) delta^*(q', S_1, S_2) $  for each $c_1, c_2 in Sigma union {epsilon }$ and $S_1, S_2 in Sigma^*$

Each FST has two projections called the input and output projections by restricting the $delta_("input")(q, s) = union.big_(s' in Sigma union {epsilon}) delta(q, s, s')$ function and likewise for the output function, which can be used to create FSMs. Due to this, an FST can be thought of as a relation between two languages, i.e. a subset of $Sigma^* times Sigma^*$. An input string is related to the output string if and only if the input FSM accepts the input and the output FSM accepts the output strings. 


Such relation is called a _regular_ relation. 

Some properties of FSMs can be generalized to FSts. Concatenation, union, and reversal have corresponding constructoins in FSTs. Additionally, a very useful operator on two FSTs is the _composition_ operator, shown in @composition-definition.


#definition[Composition][
Given regular relations $R_1, R_2$, a pair $(x, y)$ is in the composition $R_1 compose R_2$ if and only if there is some $z$ such that $(x, z) in R_1$ and $(z,y) in R_2$. In terms of sound laws, this corresponds with combining one sound law after another. 
] <composition-definition>



Note that the input and output constructions were chosen arbitrarily to be left and right. Indeed, it does not matter which is chosen to be which because given a regular relation $R$, there is the isomorphic construction $R^(-1)$, which switches the input and the output. Although the names input and output are similar to the terminology in a function, in general, a regular relations does not need to be function, and can display non determinsitics behavior in the sense that one input could map to multiple outputs.  

It was noted in @regular-closure that for regular languages, the compliment, union and intersection were all closed.  There is a parallel statement given in @relation-closure. 

#statement[Closure Properties for Regular Relations][
  Given _regular relations_ $R_1$ and $R_2$, the relations $R_1 compose R_2$, $R_1 union U_2$, and $R_1 dot R_2$  are all regular. 

  Additionally, for a regular relation $R$, then $R^*$ is also a regular relation.
] <relation-closure>

Notice that the the _compliment_ construction and _intersection_ are not mentioned in @relation-closure. Indeed, they will not necessarily be regular.  @kaplan1994regular 

Consider the relation $R_1 = { angle.l a^n, b^n c^* angle.r | n >= 0}$ and $R_2 = { angle.l a^n, b^* c^n angle.r  | n >= 0}$. The intersection of $R_1$ and $R_2$ is ${ angle.l a^n, b^n c^n angle.r | n >= 0 }$. Doing an output projection gives $ { b^n c^n  | n >= 0 }$, and as stated in @simple-non-regular,  this language is known to not be regular. 

== The replace operator <replace-operator-section>

Using FSTs, it is relatively trivial to create a transducer that corresponds to an unconditional sound laws. However, creating conditional sound laws is surprisingly non trivial. A sound law describing #lp.rule([x], [y], [a #lp.dash() b]) is not as easy as it seems. The seemingly obvious approach shown in @wrong-autonmaton does not work.  

#figure(

automaton(
  (
    q0:       (q0: "*:*", q1: "a:a"),
    q1:       (q2: "x:y"),
    q2:       (q3: "b:b"),
    q3: (q3: "*:*")
  ),
  initial: "q0",
  final: ("q3",),
),

  
  caption: [Obvious Approach]
) <wrong-autonmaton>


Instead @wrong-autonmaton only describes a rule will apply if there exists a context that can be replaced. For instance, a string such as "axb" would be correctly transduced to "ayb".  However, this is not the desired automaton. What is desired is an automaton that will replace all occurrences in all contexts. If there are no contexts, however, it should output the same string without changes. @wrong-autonmaton would output no string if there were no appropriate contexts, such as a string like "ab". Additionally, it fails to output the correct behavior when there are multiple contexts. A string like "axbaxb" would not output "aybayb", but instead output "axbayb" and "aybaxb", both of which are undesired. 

Not only is such a simple case surprisigly complex, but there may be additional ambiguity when describing sound laws. For instance, consider the law in @ambiguous-law, what should be the output when transducing "xxxx"?

#figure(
  lp.rule([x], [y], [x #lp.dash() x])
  , caption: [An ambiguous law]
) <ambiguous-law>

A reasonable answer is "xyyx", but this poses a interesting situation. Due to each "x" being both a potential context and value to be transduced, certain heuristics may fail to hold. For instance, it may be appealing to have an approach that only transduces one character at a time, and then combine them together via the composition operator. This example show the problem with that approach: a single transduction will destroy the context in which the second transduction should have applied. Therefore, in order to output "xyyx", the transduction for both must happen at once. 

Additionally, this breaks a symmetry that would otherwise appear. For instance, a context free law like #lp.rule([x], [y]), has a corresponding inverse rule #lp.rule([y], [x]) that will reverse the application. 
A rule such as  #lp.rule([x], [y], [x #lp.dash() x]) does not have a reverse of #lp.rule([y], [x], [x #lp.dash() x]), since the string "xyyx" no longer has the same context in which the original rule applied. 

This case seems to be artificial, but there are some linguistic phenomena that are similar. In general, this happens when the element that changes has itself as a context. One example is in Mandarin Chinese, where the third tone becomes the second tone when before another third tone. 

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

The algorithm as presented in #cite(<karttunen1997replace>, form: "prose") is a six part process consisting of 

+ InsertBrackets
+ ConstrainBrackets
+ LeftContext
+ RightContext
+ Replace
+ RemoveBrackets

The first part, 




== Applications for fst


The most influential finite state toolkit for linguistics was by the Xerox Finite State Tools (XFST). These tool demonstrated how finite state transducers could be applied to linguistic problems, especially in morphological analysis.  @beesley2003finite. They established not only practical usage of FSTs, but also the syntactic convensiont that later toolkits would adopt. 

A limitation of XFST is that it is not open source. This has encouraged modern alternatives that replicate its functionality. 

 One of the most widely used open source FST libraries is OpenFST @openfst.  OpenFst is a relatively low-level library. Users can construct FSts like graphs, specifying states and arcs explicitly. OpenFst expands the basic definition as defined in @fst-definition by supporting weighted transducers.  This weight can take the form of any _semiring_ (any set equipped with multiplication and addition). 
  OpenFst also provides the closure properties mentioned in @regular-closure, allowing the user to use the closure properties in addition to constructing a graph based layout. This allows the user to specify the FST like a regular expression, as long as the FST is isomorphic to a finite state automaton. For those that are not, the compliment and intersection operators are not available. 
  
 OpenFst is also not library that focuses only on morphological analysis. It has found usage in speech recognition @poveykaldi and other machine learning algorithms.  

Foma @hulden2009foma  is newer open source FST library that is written in C. Unlike OpenFST, it is a higher level library. It is made with a similar goal as XFST, which is morphological analysis. Its syntax is very similar to XFST and is mostly compatible with it. 
 
 
 
 Finally, HFST (Helsinki Finite State Technology) @hfst integrates mulitple backends. It uses OpenFst @openfst as a potential backend and can also use FOMA as a backend. Its syntax is also compatible with XFST and FOMA, which allows for easy use of morphological analysis. 

 As the HFST source code is available on GitHub @hfst-repo , the algorithms and code architecture used in this project were heavily inspired from HFST. 



= Infastructure <infastructure-section>

== CAPR <capr-section>

This project was initially inspired by #cite(<capr>, form: "prose"), in which the CAPR system was described. It provides an innovative approach to using FSTs to model historical linguistics.

Unlike previous approaches, Capr does not seek to automatically come up with proto forms directly from data. The goal was to create a architecture that ultimately a useful tool for the linguist. The linguist starts with data, typically a cognate set, but could also be an arbitrary list of words that they believe have cognates. The linguist then theorizes that there is some relation between the languages. The linguist then has to come up with protoforms and sound laws that would fit derive the protoform into the outputted data. The emphasis here is on verification. The tool does not come up with theories, rather the linguist comes up with the idea, and the system tests to what extent the theory holds.

Unlike other computational approaches, which often are partially "black box" models, the CAPR system mirrors system a system closer to the traditional pen and paper process. A theory is developed incrementally: initial hypothesis may be incorrect, but will be gradually refined.

More concretely, CAPR is a web application.  The linguist first provides the set of data cognates. The linguist then provides the the sound laws by inputting a the sound laws using the XFST syntax. The sound laws need to specify which languages the laws to apply to, since CAPR requires that there be at least two descendant languages when constructing proto forms. The sound laws get fed into a backend server that takes the HFST rules and turns them into transducers. The backend then looks at each cognate set and performs the backward transduction according to the sound laws for the respective language. As mentioned ins @fst-section, the backward transduction creates potentially multiple protowords. If the protoword appears in multiple reverse transductions for different descendant languages, then this shows that the sound laws are valid for that word, and the words is displayed to the linguist. 

The core idea of CAPR is very promising, however there were some limitations that motivated this project. 

First was general inconvenience. Just installation was slightly difficult.  Although there was a docker container available, it was nevertheless difficult to get running. This could have come from a variety of sources: potentially, a bug on the developer end, or an error in the user configuration. This will likely be a barrier for linguists who may not be as technologically inclined or willing to invest time on setup. 

Second, the dependence on HFST potentially introduces inconveniences. Since rule compilation is handled server-side, the linguist must show competence by downloading the full project locally and building, or there has to be a server running somewhere to handle all the requests. This comes with additional infrastructure and maintenance costs. A simpler, self-contained web app that runs in the browser would improve accessibility for linguists. 


Third, the user interface requires the linguist to learn HFST. Although not particularly difficult, the syntax is somewhat different from the typical hand written form, creating an additional learning curve. Moreover, the linguist works mostly by typing in the edit box, which, although functional, is not as interactive as other interfaces. 

Finally, Capr requires a strict adherence to the Neogrammarian Hypothesis. A potentially better system can be imagined that would be more accommodating to failures of the Neogrammarian hypothesis. In this sense, a system that allows the strictness for the Neogrammarian Hypothesis to be strict when necessary, but also to relax the hypothesis when necessary.

Despite these drawbacks, the CAPR project makes a lot of headway by opening a new direction in the field of computational historical linguistics that has been understudied. The decision to put the emphasis on sound laws is still a great improvement.  


== Project Desciption

This project can be viewed as an exploration of lightweight transduction in the spirit of CAPR. Its primary goal was to explore more user friendly approaches for sound law transduction using finite state transducers. The end result is unfortunately in a more prototype state than the original CAPR, some expreience was gained in the direction of sound law transduction, and could potentially be a starting block for other approaches in sound law transduction. 

Like the CAPR predecessor, the aim of this project was not create a system automatically contruct or predict word forms. Instead, focus lies in being a tool to the linguist, while not infringing on the "job" of the linguist. The linguist does what they always do, but with assistance from the computer in doing the boring, mundane parts. This means the linguist still comes up theory of the proto language and how it changes. 

Instead the tool has two important roles ti plays. In addition to doing fulfilling the idea of making the linguist's life easier  of transducing sound laws, the idea behind it is potentially more general: which is verification of theories. Frequently, words will have etymology entries in dictionaries, but rarely is there any explicit verification on this. In some cases such as for the dictionaries of proto languages, a list of sound changes may be supplied to discuss general the history of the language and potentially the sound changes that occurred for various daughter languages. This is relatively rare when discussing every day dictionaries, and the user is often assumed to just believe the entry. Rarer still is for the user to be able to see some intermediate sound changes, which is occasionally seen in some Wiktionary entries. By making the tool explicitely see sound changes, it forces the process to be transparent to the reader along with teh author. This can be seen as also proof reading the author. The author makes claims, but the tool produces the evidence.

The second goal builds on top of the _verification_ goal, which is _discoverability_ of proto-word and sound laws.  Once a theory is reified, it becomes easier for a computer or any automatic system to propose potential proto-words that can then be checked if they transduce into valid words in the daughter languages. Using transducers, this becomes easy due to the inverse construction being available. However, even if an inverse construction were not feasible, it would not be difficult to devise a heuristical algorithm to search for possible matches. 

A sound law does not have to absolutely correct in order for it to be useful to the linguist. Often times, a partially correct sound law is enough. Once that law has applied to many examples, it gives a hint to the liguist as to under what conditions it succeeds and where it fails, allowing the linguist to refine the sound law. 

Following these ideas, the first main goal was to create a minimum working version of a transduction system to be used for historical linguisti construction. This should improve upon the CAPR predecessor by reducing barriers for those without technical background. It also should try to create a simpler to user UI and provide a simpler syntax that is more familiar than teh HFST rule syntax. To accomplish this, it was orginally envisioned to be a standalone, browser based applicatin requiring minimal setup. Whiel the technical goals were only partially achieved, the effort has left some expierence t o reflect on.

The core functionality is opened in Rust. Rust is a relatively new programming language that aims to be as fast as C or C++. The language does not have to deal with the 40 years of languages features that C and C++ have to suupport in order to be backwards compatible, and as a result, has a very modern experience when programming in it, while also not sacrificing in performance.  It achieves this high performance by using a unique memory management system that does not require a garbage collector, but uses compile time checks to prevent memory errors. 

The library was built using rustfst @rustfst, a rust port of the OpenFst @openfst that is a C++ transduccer library. While rustfst provides many low level FST operations, it lacks some higher level operations that are included in HFST. 

As a result, a considerable amount of development was devoted to reimplementing HFST functionality. The most important is the _replace operator_ as discussed in @replace-operator-section, which is required in expressing context sensitive rewrite rules. As discussed in @karttunen1997replace, the replace operator is decnetly complex. The currently implementation is based off the HFST source code, but Most importantly, the _complement_ operator is a crucial operator that is missing in rustfst. Of the four types mentioned in @four-orient, the project only implements the upper oriented replace operator. Additionally, it does not support the parallel replace operator, meaning it can only do one replace at a time.

A major challege was implementing the replace operator. The first step was to locate the code in the HFST library. Once the HFST code was located, much time was spent porting the code over to a rust version. The algorithm of the rust version was similar to the algorithm preseneted in #cite(<kaplan1994regular>, form: "prose"), yet it was not without difficulites. The algorithm crucially depended upon a the compliment opererator. As mentioned in @fst-section, however, in general, a compliment does not exist for an FST but ony for an FSM. 

todo: finish







= Case Study Celtic <celtic-section>


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
#let R = lp.fmat([resonant])
#let L = lp.fmat([liquid])
#let N = lp.fmat([nasal])
#let Palat = lp.fmat([+palatal])
#let NoPalat = lp.fmat([-palatal])
#let Aspir = lp.fmat([+aspiration])
#let NoAspir = lp.fmat([-aspiration])
#let Stop = lp.fmat([+stop])
#let Long = lp.fmat([+long])
#let NoLong = lp.fmat([-long])
#let Lab = lp.fmat([+Labial])
#let Dental = lp.fmat([+Dental])

#figure(
 table(
  columns: 1,
  align: left,
  [#lp.rule([h₁e], [e]), #lp.rule([h₂e], [a]), #lp.rule([h₃e], [o])],
  [#lp.rule([eh₁], [ē]), #lp.rule([eh₂], [ā]), #lp.rule([eh₃], [ō]),],
  [#lp.rule(lp.fmat[laryngeal], [a], [#C #lp.dash() #lp.fmat([consonant]) ])],
  [ #lp.rule[#lp.fmat[laryngeal]][#sym.emptyset][#lp.fmat[stop] #lp.dash() #lp.fmat[stop]]],
  [ #lp.rule[#lp.fmat[coronal][stop] ~ #lp.fmat[coronal][stop]][s s ]],
  [ #lp.rule[#sym.emptyset][a][#lp.fmat[consonant] ~ #lp.fmat[resonant] #lp.dash() #lp.fmat[laryngeal] ~ #lp.fmat[resonant] ]],
  [#lp.rule[#H][#sym.emptyset][#V #lp.dash() #lp.fmat[laryngeal] ~ #lp.fmat[resonant] ]], // lookup how they did verners law in the book, number 7
  [#lp.rule(H, [a], [\# #R #lp.dash() #C])],
  [#lp.rule(Palat, NoPalat)]
   ),
  caption:[Dialectical Indo-European Changes]
) <changes-a>

#figure(
  table(
    columns: 1,
    lp.rule([gʷ], [b]),
    lp.rule(Aspir, NoAspir),
    lp.rule(sym.emptyset, [i], [#C #lp.dash() #Stop]),
    lp.rule([e], [a], [#lp.dash() #R ~ a]),
    lp.rule(sym.emptyset, [a], [#C #lp.dash() #R #C]),
    lp.rule(H, sym.emptyset),
    [todo],
    lp.rule([e:], [i:]), //todo fix this
    lp.rule([o:], [u:]), // add final symble requirement
    lp.rule(Long, NoLong),
  ),
  caption: [Early Changes]
) <changes-b>

#figure(
  table(
    columns: 1,
    lp.rule(C, [x], [ #lp.dash() #C]),
    lp.rule([p], [b], [ #lp.dash() #L]),
    lp.rule([p], [w], [ #lp.dash() #N]),
    lp.rule([p], [f]),
    lp.rule([o:], [a:]),
    lp.rule([ey], [e:]),
    lp.rule([ew], [ow]),
    lp.rule([uw], [ow], [#lp.dash() #C]),
    
    
  ),
  caption: [Late Changes]
) <changes-c>


#figure(
  table(
    columns: 1,
    lp.rule([f], [r], [ r #lp.dash() ]),
    lp.rule([f], [l], [l #lp.dash() ]),
    lp.rule([r], [s], [r #lp.dash()]),
    lp.rule([s], sym.emptyset, [r #lp.dash() t]),
    lp.rule([m], [w], [#lp.dash() w]),
    lp.rule(#NoLong, #Long, [xs #L]),
    lp.rule([ar], [ra], [#Lab #lp.dash() #Dental #Dental]),
    lp.rule([al], [la], [#Lab #lp.dash() #Dental #Dental]),
    lp.rule(H, sym.emptyset, [#V y #C])
    
    
  ),
  caption: [Probable Changes]
) <changes-d>


When these laws were programmed in, it was discovered that certain laws had been incorrectly stated, or inadequately described.



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


== Discussion of Results

For simple cases, such as when the word is the exact same in Proto-Celtic as in PIE, the system performs well. There are many cases also, where the tool almost gets it right, but the Proto-Celtic has an extended suffix on it. In these cases, they havce been noted in the _notes_ column if the a suffix has been appended. 

= Discussion and Conclusion

In @infastructure-section, the project and infastructure was discussed. In @celtic-section, a proof of concept was introduced that used the tool to transduce Proto-Indo-European words to Proto-Celtic. The entire process of creating the tool and building was a long one and came with any expereiences and room for improvement. 

It may have been noticed that the concepts introduced in @linguistics-section were not all met. 

Working through the Celtic example has revealed some drawbacks of usin the FST formalism. 

One initial advantage of using the FST formalism is the supposed speed in transduction. Indeed, once a transducer is created, it is potentially very fast. However, this is only after the transducer is created. Crucially, the replace operator requires the complement construction, and the complement construction requires determinization of the FSM. As mentioned before, this is an $O(2^n)$ construction. This has dramatic effects on the runtimes speed. 

As a result, in order to achieve good performance, each FSt must be minimized to prevent the state explosion. This is however also hard to ensure.

This is related to another issue found with FSTs. Thta is, they are hard to reason about, and consequently hard to debug. A major problem is the replace operator, as mentioned in @replace-operator-section consists of a nontrivial algorithm to do the replacing. Additionally, if the algorithm is not implemented correctly, it is very hard to identify where it is incorrect. 

Unlike more typical programming languages, programming in FSTs often requires inspecting the entire FST. For instance, when programming in mainstream programming languages, a user can inspect a function call by having a debugger call the and see how the state changes after each statement. This functionality is unavailable when working with FSTs. The FST is relatively opaque. Although the graph view is availabe, it is relatively hard to reason about once there are a non trivial amount of vertices. 

Even more difficult are the runtimes difficulties. It is very hard to pinpoint where exactly the exact cause of a slow down. Due to the potential expoential nature, a single non optimal segment in the FSt can result in a significant slowdown. Even more unfortunate is that the library seems to have a slightl y different implementation for determinization than OpenFSt, and performance could be due to this difference or implementation error of other factors. 

Due to this , the project is in a state where it has drawbacks in comparison to a system that uses HFST directly. The benefits of being a single website without download is likely not outweighed by this drawback. An inconvenient system is likely better than a system that does not work. 

Additionally there are more theoritical reasons on why an FST system to transduce is not optimal.

One of the goals mentioned in @linguistics-section is _verfication_ of linguistic theories. The tool does this job correctly in that it checks _if_ it is incorrect. The next question to answer is _how_ it is incorrect. In the case that the theory is relatively simple, it may be sufficient to simply show the FST graph. But this quickly becomes unweildy for non trivial theories. Ideally, a system should not only provide a transduced result, but also to "show its work". As of now, the only way to achieve this is by reconstructing all intermediate FSTs, which is slow and requires infastructure. 

FSTs are also very much tied to the _string_ structure. Although language is for the most part able to be modled strings, certain phenomena introduce difficulties. Languages sometimes consist of supersegmental features, such as tone and stress. 


#bibliography(("bib.bib", "bib.yaml"))








