
#import "@preview/ascii-ipa:2.0.0": *

// #import "@preview/casson-uom-thesis:0.1.1": *
#import "template.typ" : *

#show: uom-thesis.with(
  title: "My Thesis",
  author: "John Wang",
  faculty: "Philosophische intitustu",
  school: "tuebingn",
  departmentordivision: "Seminar fur sprachwssensvhf",
  //termsandabbreviations: [Enter terms and abbreviations as table or similar], // uncomment if want in thesis
  // layabstract: [Lay abstract goes here], // uncomment if want in thesis
  // theauthor: [If desired, a brief statement for External Examiners giving the candidate’s degree(s) and research experience, even if the latter consists only of the work done for this thesis.], // uncomment if want in thesis
  year: "2025",
  font: "times", // choices are: "times", "palatino", "roboto", "noto_sans" 
  fontsize: 11pt, // can be any reasonable value
)




// #set page(margin: 1in)
// #set par(leading: 0.55em, spacing: 0.55em, first-line-indent: 1.8em, justify: true)
// #set text(font: "New Computer Modern")
// #show raw: set text(font: "New Computer Modern Mono")
// #show heading: set block(above: 1.4em, below: 1em)




// #align(center, text(30pt)[
// *My Master Thesis*
// ])


// #align(center, text(17pt)[
// John Wang
// ])


/* #pagebreak()


#set page(numbering: "1") */



= Historical Linguistics

People have been noticing that language changes. Historical Linguistics is the study of change in language. @hock2021principles

The living pieces of evidence that language changes is simply examing the English language. The English lanuguage is notoriously known for its freqently arbitrary spelling rules. @silentk shows a list of _k_ words that do not have the _k_ sound. 


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


A partial reason is because of the effects of sound change. English spelling often reflects earlier stage of the language, where things were pronounced differently. The spoken language changed, but the written language had failed to update to the newer pronunciattion. 

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

It can be theorized that German and English were once the same language. However, through the many years of being apart, various changes in pronunciation, grammar and meaning have effected both languages, making them non mutually intelligable. One of these changes that happened relatively recently was the change shown in @silentk, of deleting \<k\> when before \<n\>, which happened in English, but did not happen in German. 



== Neogrammarian Hypothesis


As a child learning how to read English, seeing the spelling of the the words listed in @silentk for a general rule: \<kn\> gets pronounced as \<n\>. This generalization parallels the developement of the development of English, so what initially seems to be a pronunciation rule may be more accurately described as a _sound change rule_ or a _sound law_, a term used by the Neogrammarians. 

Additionally, this particular sound law is observable because of the conservative orthography of English. But English has only (relativly speaking) recently been written down. If, for instance, English and German had written down their language back when they were one language, would it be possible for a learner of the language to similarly form a large list of rules to derive the English pronunciation, and a different set of rules to derive the German pronunciation?


The Neogrammarians had a similar thought, and believed


Latin, Greek, and Sanskrit, but more importantly they found cognacy between sound laws. This is the most important part, in which they have a system that consistently maps between. That is, the sound is regualr.  @hock2021principles 

The Neogrammarians believed that sound change was absolutely regular. @hock2021principles They believed what exception that they encountered were merely sound laws that had been not sufficiently generalized, or had been regularized by analogy. 

However this is probably not a very realisitic model of language. Unlike the physical world, which has laws written in the language of mathematics that will predict exactly what will happen, languages are full of arbitrariness.

There are examples of changes that are _sporadic_ and effect only one word. Or maybe non determinsitcs behavior, where a single word splits. 



== Sound Laws

Linguists have a conventionalized representation for laws.

Usually they write them in teh form $A > B$ where $A$ is a phoneme that gets changed, and $B$ is the phoneme that it gets changed into. They can be changed to represent the sound laws $A > B > C$ such as the Grimm's law, which can be represeneted as #xsampa("d > t >")  ϑ

To represent context laws they usually 
@crowley2010introduction


$ A > B \/ X#box(width: 0.5cm, clip: true, outset: (y: 1em), underline(extent: 1000cm, sym.space.nobreak))Y $

where the variables $A, B, X, Y$ can represent strings, or potential features. The variables $X, Y$ represent the left hand side and right hand side contexts. 

It is important to note that this a rough convention. The notation is inadequate to describe all types of sound law change. Authors will usually need to supplement this notation with english language explanations.


== Computational Linguistics

It would seem that this notation would easily be read by a computer. Indeed, it is not hard to write a computer program that, given a set of words, and the sound laws that are known, to then write a computer program that transduces this. 

There are many benefits of this doing this. Is that we now have _verification_ of the sound laws. A linguist would otherwise be the only source of trust. As the amount of sound changes is usually non trivial, it is not hard to accidentally forget to apply a law, or apply it in the wrong order. 

- formal notation -> auto trandcution
- current standrad is primitve keep in your head
- verifcation of sound laws
- old Neogrammarian "all sound laws are regular" may be outdated and flawed

  - we want them to be ideally be regular
  - but if not regular, there may be some use after all
  - we want to say that high regularity = high evidence
  - we want a tool to help gauge how regualr theoreis are
  - additionally these tools may be help us find different kinds of "relatedness"
      - Borrowing
      - Japanese readings for instance
      - Romance doublets into latin
      - better modelling of wave based linguistics

- Solves also a different type of problem

== Sound Changes as a study of itself

- Index Diachronica
  @diachronica
  - More or less just a list
- Otherewise no comprehensive analysis
- Other ad hoc sound law transducers
- Example is Zompist @zompist for conlangers, which provide a forward application


 = Finite State Trandsducers

 == Finite State Automata

 Finite state autmata (or finite state machine) are a mathematical model basic form of autmata that can be used ot model a variety of phenomona. The most relvant usage is in string recognition.

 
 == Formal Definition

To talk about strings, a formal definition is required.

Given a set $Sigma$ a _string_ can be defined recursively defined as being either
- $epsilon$, the empty string
- $c dot S$ where $c in Sigma$ and $S$ is a string
The set $Sigma$ is usually called the _alphabet_ and the members _characters_. The $dot$ operator can be extended to a concatentation operator of two strings where

- $epsilon dot S = S$
- $(c dot S_1) dot S_2 = c dot (S_1 dot S_2)$ 

A string is a common example of a _monoid_ because the $dot$ operator is associative, and $epsilon$ is the identity. 
By convention, strings are usually put in double quotes and the $dot$ operator commonly ommited. Examples: todo


A formal _language_ can be defined a (potentially infinite) set of strings. Can be used to define natural language such as English, spansih, but can also be used to define more artifical lanuages, such as all prime numbers represented in decimal. 


A deterministic finite state machine can be defined as  $(Q, s, Sigma, F, delta)$, where 

- $Q$ is the set of states
- $s in Q$ is the start state
- $Sigma$ is the _alphabet_, a set of characters
- $F subset Q$ the set of final states
- $delta: Q times Sigma -> Q$ the transition function that maps each state to its next state when given a character

A finite state machine $M$ can then be used to define a language $L$ by stating that a word is in $L$ if and only if $M$ accepts $L$.

A finite state machine $(Q, s, Sigma, F, delta)$ accepts a string $S$ when one of the following conditions are met:

- $S = epsilon$ and $s in F$ 
- $S = c dot S'$ and $delta(q, c) = q'$ and $M' = (Q, q', Sigma, F, delta)$ accepts $S'$

todo: add an example with a picture

A related construct is the nondeterministic finite state machine.
A nondeterministic finite state machine can be defined as  $(Q, s, Sigma, F, delta)$, where 

- $Q$ is the set of states
- $s in Q$ is the start state
- $Sigma$ is the _alphabet_, a set of characters
- $F subset Q$ the set of final states
- $delta: Q times Sigma -> P(Q)$, $P$ being the powerset function. The nondeterministic transition function that maps each state and character to possible states.

The only difference here is the transition function, which can now map to multiple or zero states, in addition to just one state.

The non deterministic finite state machine also has a similar accepts function. 

A finite state machine $(Q, s, Sigma, F, delta)$ accepts a string $S$ when one of the following conditions are met:

- $S = epsilon$ and $s in F$ 
- $S = c dot S'$ and there exists a $q$ such that $delta(q, c) = q'$ and $M' = (Q, q', Sigma, F, delta)$ accepts $S'$

A deterministic finite state machine is trivially a non deterministic finite state machine. However, it may be surprising to know the converse is also true: any non deterministic finite state machine can be turned into a deterministic finite state machine which accepts the same language.


Additionally, a further extension of non deterministic machines can be created to also accept epsilon transitions, i.e. $delta: Q times (Sigma union { epsilon }) -> P(Q)$, $P$ being the powerset function. Note that the previous acceptance function is still valid due to the semantics of $epsilon$.

Again, this may be shown that any epsilon extended finite state machine can be turned into an equivalent machine without the epsilons. Due to this, it usually does not matter whether it is called a deterministic finite state machine or a non determinstic finite state machine and the specifics can be relegated as an implementation detail. In general, they can be referred as (FSM) finite state machines.

== Elementary Results in FSMs

Given an alphabet $Sigma$, a regular expression can be recursively defined as


- $epsilon$ the empty regular expression.
- $c$ where $c in Sigma$ the singleton character
- $R_1 dot R_2$ where $R_1$ and $R_2$ are also regular expressions, concatenation.
- $R_1 | R_2$ where $R_1$ and $R_2$ are also regular expressions, disjunction.
- $R_1^*$ where $R_1$ is a regular expression, the Kleene Star.

Given a particular expression $R$, we can define the semantics for $R$ matching a string $S$ if

- $R = epsilon$ and $S = epsilon$
- $R = c$, where $c in Sigma$  and $S = c$
- $R = R_1 dot R_2$ and there exists $S_1$ and $S_2$ such that $S = S_1 dot S_2$ and $R_1$ matches $S_1$ and $R_2$ matches $S_2$
- $R = R_1 | R_2$ and $R_1$ matches $S$ or $R_2$ matches $S$
- $R = R_1^*$ and $S = epsilon$ or $S = S_1 dot S_2$ and $R_1$ matches $S_1$ and $R_1^*$ matches $S_2$.


Regular expression are commonly used for 

=== Applications for fst

A common library is openfst @openfst.  Hfst @hfst uses openfst @openfst as a potential backend.

Hfst itself is used as tool for morphological analysis. This was described using the xerox tools described in  #cite(<beesley2003finite>, form: "prose") using the 
= Infastructure

== CAPR

This project was initially inspired by #cite(<capr>, form: "prose"), in which the CAPR system was described. It provides an innovative approach to using FSTs to model historical linguistics.

#cite(<capr>, form: "prose") was to create a architecture that ultimately a useful tool for the linguist. The linguist has data and theorizes that some group of languages are related. The linguist then has to come up with protoforms and sound laws that would fit the data. The tool does not come up with theories, rather the linguist comes up with the idea, and the system tests to what extent the theory holds.

Unlike other approaches, the CAPR system allows a system closer to the traditional pen and paper process by developing a theory incrementally: initial hypothesis may be incorrect, but will be gradually refined.

More concretely, the system is a web application where the linguist provides the set of data cognates. The linguist then provides the the sound laws by inputting a the sound laws in the form of transducers. The sound laws need to specify which languages the laws to apply to, since CAPR requires that there be at least two descendant languages when constructing proto forms. The sound laws get fed into a backend server that turns the HFST rules and turns them into transducers. The backend then looks at each cognate set and performs the backward transduction according to the sound laws for the respective language. The backward transduction creates potentially multiple protowords. If the protowrod appears in multiple reverse transductions for different descendant languages, then this shows that the sound laws are valid for word, and the words is displayed to the linguist. 


There are some areas which in which the original CAPR seemed to be lacking.

The first is simply installation. The original repository had a docker container. This did simplify installation somewhat, but the either the code was buggy, or I installed something wrong, or I didn't do some configuration or whatever. I don't really know something someting it like kind of just didn't work. The averae linguist is potentially not that technlogoically literate and also doesn't reall yhave that mu. Compare to like other transducers that I have to cite. 


Another potential inconvenience is the the dependence on HFST. Though it is not necessarily bad, I think there some things that could be done better. For instance it forces a server client architctue, meaning the linguist is forced to install something, or the have to rent out a server that transduces the thing for everyone or they have to like the things blah blah you know you just want liek a simple web page that does everytig locally. You got a no click install just go to a webpage and bam you are there. 

You also have to learn HFST, which I guess its not bad, but i does require some getting used to. The concatenation must still be done manually and the it's also just like you hvae to type in code and whatever. It would be nicer if there was somehting more visual and like you doesn't work as bad. And it's also slightly annnoying you have to click transduce and it like take a second to switch everyihtng. And yeah



== Project Desciption

The project can be thought of as a miniature version of Capr.

Basically my version of a webapp that is slightly buggy and kind of doesn't really work.

However it does work for very simple stuff and has a somewhat more visual user interface.

The backend is written in rust for the memes and becuase rust is fast and stuff and I don't lke c++. The project is made using rustfst @rustfst, which is a rust language port of the C++ library OpenFst.  @openfst The core functionality has been implemented in rustfst, but certain features had to be implemented that were already implemented in openfst. 

Additoinally Rustfst is a rather low level library. In comparison to Capr, which depended on Hfst, some impelmentation details had to be reimplemented in the Rust library that had already came for free with Hfst. 

The most important of this is the replace operator. The replace operator is surprisingly non trivial.  @karttunen1997replace

The code for the replace is essentially translated from Hfst's open source code. The rust implementation only implements a subset of Hfst's replace operator. It lacks the parallel replacement operator and also the various down up thing whatever and only the does the up replacement. 


The project tries to provide a similar function to hfst @hfst.

The original project was to provide a similar function to the one given in capr @capr



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

todo: actually write down these laws

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








