
#import "@preview/ascii-ipa:2.0.0": *

#set page(margin: 1in)
#set par(leading: 0.55em, spacing: 0.55em, first-line-indent: 1.8em, justify: true)
#set text(font: "New Computer Modern")
#show raw: set text(font: "New Computer Modern Mono")
#show heading: set block(above: 1.4em, below: 1em)

#set page(numbering: "1")
= Intro




= Historical Linguistics


Historical Linguistics is arguably a really old part of linguistics. The neogrammarians found cognates between latin, greek and sankrit, but more importantly they found cognancy between sound laws. This is the most important part, in which they have a system that consistently maps between. That is, the sound is regualr.


A famous example is Grimm's law. This is known as a chain shift in which certain consontats correspond in other consonants.

For exampls latin cognates picic fish, whatever german and lots

Another sound law that's really common is called paltalization.

For instance spanish _caesar_


- Sound Change is regular
- Well, probably not that regular
- Context based sound law changes
- Also that certain sound changes are universal


Somehting somthing let's get started with sound laws.

== Sound Laws

Linguists have a conventialized representation for laws.

Usually they write them in teh form $A > B$ where $A$ is a phoneme that gets changed, and $B$ is the phoneme that it gets changed into. They can be changed to represent the sound laws $A > B > C$ such as the Grimm's law, which can be represeneted as #xsampa("d > t >")  ϑ

To represent context laws they usually

$ A > B \/ X#box(width: 0.5cm, clip: true, outset: (y: 1em), underline(extent: 1000cm, sym.space.nobreak))Y $

where the variables $A, B, X, Y$ can represent strings, or potentiall features. The variables $X, Y$ represent the left hand side and right hand side contexts. 

It is important to note that this a rough convention. The notation is inadequate to describe all types of sound law change. Authors will usually need to supplement this notation with english language explanations.


- formal notation -> auto trandcution
- current standrad is primitve keep in your head
- verifcation of sound laws
- old neogrammarian "all sound laws are regular" may be outdated and flawed

  - we want them to be ideally be regular
  - but if not regular, there may be some use after all
  - we want to say that high regularity = high evidence
  - we want a tool to help guage how regualr theoreis are
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

The only difference here is the transition function, which can now map to multiple or zero states, in additon to just one state.

The non determinstic finite state machine also has a similar accepts function. 

A finite state machine $(Q, s, Sigma, F, delta)$ accepts a string $S$ when one of the following conditions are met:

- $S = epsilon$ and $s in F$ 
- $S = c dot S'$ and there exists a $q$ such that $delta(q, c) = q'$ and $M' = (Q, q', Sigma, F, delta)$ accepts $S'$

A deterministic finite state machine is trivially a non deterministic finite state machine. However, it may be surprising to know the the converse is also true: any non deterministic finite state machine can be turned into a deterministic finite state machine which accepts the same language.


Additionally, a further extension of non deterministic machines can be created to also accept epsilon transitions, i.e. $delta: Q times (Sigma union { epsilon }) -> P(Q)$, $P$ being the powerset function. Note that the previous acceptance function is still valid due to the semantics of $epsilon$.

Again, this may be shown that any epsilon extended finite state machine can be turned into an equivalent machine without the epsilons. Due to this, it usually does not matter whether it is called a deterministic finite state machine or a non determinstic finite state machine and the specifics can be relegated as an implementation detail. In general, they can be referred as (FSM) finite state machines.

== Elementary Results in FSMs

Given an alphabet $Sigma$, a regular expression can be recursively defined as


- $epsilon$ the empty regular expression.
- $c$ where $c in Sigma$ the singleton character
- $R_1 dot R_2$ where $R_1$ and $R_2$ are also regular expressions, concatention.
- $R_1 | R_2$ where $R_1$ and $R_2$ are also regular expressions, disjunction.
- $R_1^*$ where $R_1$ is a regular expression, the Kleene Star.

Given a particular expression $R$, we can define the semantics for $R$ matching a string $S$ if

- $R = epsilon$ and $S = epsilon$
- $R = c$, where $c in Sigma$ and  and $S = c$
- $R = R_1 dot R_2$ and there exists $S_1$ and $S_2$ such that $S = S_1 dot S_2$ and $R_1$ matches $S_1$ and $R_2$ matches $S_2$
- $R = R_1 | R_2$ and $R_1$ matches $S$ or $R_2$ matches $S$
- $R = R_1^*$ and $S = epsilon$ or $S = S_1 dot S_2$  and $R_1$ matches $S_1$ and $R_1*$  matches $S_2$.


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

You also have to learn HFST, which I guess its not bad, but i does require some getting used to. The concatenation must still be done manually and the it's also just like you hvae to type in code and whatever. It would be nicer if there was somehting more visual and like you doesn't work as bad. 



== Project Desciption



The project is made using rustfst @rustfst, which is a rust language port of the C++ library @openfst. 

The project tries to provide a similar function to hfst @hfst.

The original project was to provide a similar function to the one given in capr @capr



= Case Study Celtic


== Background Information

=== Celtic

Celtic is an Indo-European language family. This includes members such as Irish, Scottish, Welsh, and Gaulish. @celtic The language which these language descended from is called Proto-Celtic, because it has not been directly attested, and has been reconstructed from it's living descendants.  

=== Proto-Indo-European

Proto-Indo-European (PIE) is the reconstructed ancestor of many languages of Europe. Its descendants include Germanic Languages (English, German, etc), Slavic Languages (Russian, Polish, Ukranian), Romance Languages (Spanish, French, Italian, etc), and many languages of India. 

The grammar of PIE is reconstructed to be complex. The scope is quite large, but the most relevant details will be covered.

Freqeuntly 


The case study examines #cite(<celtic>, form: "prose").

  


#table(
  columns: 5,
  table.header[*Pie Stem*][*Transduced Result*][*Proto Celtic Root*][*Correct*][*Notes*],
  [phte:r], [fatiːr], [fatiːr], [true], [no comment],
  [krdtu], [krissu], [krissu], [true], [no comment],
  [plhno], [flaːno], [flaːno], [true], [no comment],
  [plhno], [blaːno], [flaːno], [false], [Wrong initial],
  [grhno], [ɡraːno], [ɡraːno], [true], [no comment],
  [grxno], [ɡraːno], [ɡraːno], [true], [no comment],
  [grqno], [ɡraːno], [ɡraːno], [true], [no comment],
  [g_wow], [bow], [bow], [true], [no comment],
  [krd], [krid], [krid], [true], [no comment],
  [b_hero], [bero], [bero], [true], [no comment],
  [terhtro], [teratro], [taratro], [false], [Spurious _e_ to _a_],
  [dnt], [dant], [dant], [true], [original danto],
  [b_hrso], [barso], [barso], [true], [no comment],
  [klheto], [kleto], [kaleto], [false], [_a_ inserted],
  [wlho], [walo], [walo], [true], [no comment],
  [hre:g], [riːɡ], [riːɡ], [true], [b8],
  [hepirom], [efirom], [efirom], [true], [no comment],
  [xwehnto], [winto], [winto], [true], [b11],
  [sixmdo], [siando], [sindo], [false], [Law incorrectly feeds],
  [septm], [sextm], [sextam], [false], [Fails to append add an _a_],
  [prptu], [frixtu], [frixtu], [true], [c1],
  [prptu], [brixtu], [frixtu], [false], [Wrong initial],
  [mrg_wto], [mrixto], [mrixto], [true], [c1],
  [xmlgto], [amlixto], [mlixto], [false], [Incorrectly feeds _a_ insertion],
  [piprqse], [fibraːse], [fibrase], [false], [c2],
  [supno], [sowno], [sowno], [true], [p is not accounted for],
  [deqno], [daːno], [daːno], [true], [c5],
  [reyd], [reːd], [reːd], [true], [no comment],
  [newyo], [nowyo], [nowyo], [true], [no comment],
  [potr], [fotr], [fatar], [false], [cannot account for a vocalism],
  [pelhu], [felu], [filu], [false], [says it preserves e grade, but don't know where i comes from],
  [plew], [flow], [flow], [true], [no comment],
  [plew], [blow], [flow], [false], [no comment],
  [kapr], [kabr], [ɡabro], [false], [initial consonant is irregular],
  [d_hg_hesi], [xɡesi], [ɡdesi], [false], [metathesis],
  [d_hg_ho:m], [xɡaːm], [ɡdon], [false], [metathesis],
  [gexr], [ɡaːr], [ɡaːr], [true], [original ga:ri],
  [g_hexns], [ɡans], [ɡans], [true], [original gansi],
  [g_helq], [ɡel], [ɡel], [true], [original gelo],
  [genhos], [ɡenos], [ɡenos], [true], [no comment],
  [genu], [ɡenu], [ɡenu], [true], [no comment],
  [g_hyemo], [ɡyemo], [ɡyemo], [true], [no comment],
  [k_wend_h], [kʷend], [kʷend], [true], [so element at the end seems to be an addition kwendso],
  [k_wey], [kʷeː], [kʷeː], [true], [original kwe:s s includes the nominative ending],
  [k_wetwores], [kʷetwores], [kʷetwores], [true], [no comment],
  [k_wid], [kʷid], [kʷid], [true], [no comment],
  [kwo:n], [kwaːn], [kʷon], [false], [not sure about u element],
  [kmti], [kanti], [kanti], [true], [no comment],
  [kewxro], [kowaro], [kawaro], [false], [no comment],
  [klexro], [klaːro], [klaːro], [true], [no comment],
  [klewos], [klowos], [kluwos], [false], [no comment],
  [krewx], [krow], [kruː], [false], [no comment],
  [leg_h], [leɡ], [leɡ], [true], [original lego/ legyo],
 [link_w], [linkʷ], [linkʷ], [true], [original link_wo original leyk_w, but n infix added],
  [leyd], [leːd], [loydo], [false], [incorrect vowel, may be from _o_ grade],
  [lewko], [lowko], [lowko], [true], [no comment],
  [hlud_h], [lud], [lud], [true], [no comment],
  [mexk], [maːk], [mak], [false], [original mako],
  [mexte:r], [maːtiːr], [maːtiː], [false], [no comment],
  [med_hyo], [medyo], [medyo], [true], [no comment],
  [mel], [mel], [mall], [false], [Does not account for double vowel],
  [nexu], [naːu], [naːwaː], [false], [no comment],
  [nem], [nem], [nem], [true], [original nemo],
  [neb_hos], [nebos], [nemos], [false], [no comment],
  [newos], [nowos], [nowyo], [false], [no comment],
  [nok_wt], [noxt], [noxt], [true], [no comment],
  [qektoh], [oxto], [oxtuː], [false], [no comment],
  [qeyno], [oyno], [oyno], [true], [no comment],
  [reyd_h], [reːd], [reːd], [true], [original re:do],
  [reyh], [reː], [reːno], [false], [no comment],
  [re:g], [riːɡ], [riːɡ], [true], [no comment],
  [re:gnix], [riːɡni], [riːɡaniː], [false], [no comment],
  [hrewd_h], [rowd], [rowd], [true], [original rowdo],
  [sexg], [saːɡ], [saɡyo], [false], [no comment],
  [sexl], [saːl], [salano], [false], [no comment],
  [sxl], [sal], [sal], [true], [original saltro],
  [smxel], [smal], [samali], [false], [no comment],
  [sext], [saːt], [saːt], [true], [original sa:ti],
  [sed], [sed], [sed], [true], [original sedo],
  [seno], [seno], [seno], [true], [no comment],
  [sent], [sent], [sent], [true], [original sentu],
  [soru], [soru], [serwaː], [false], [no comment],
  [spelhg_h], [sfelaɡ], [sfelɡaː], [false], [no comment],
  [sperxg], [sferaɡ], [sfraxto], [false], [no comment],
  [sisk_wo], [siskʷo], [sisku], [false], [no comment],
  [stex], [staː], [sista], [false], [no comment],
  [sek_w], [sekʷ], [skʷetlo], [false], [no comment],
  [skeqt], [skaːt], [skaːt], [true], [original ska:to],
  [skrib_h], [skrib], [skriːbbaː], [false], [no comment],
  [slunk], [slunk], [slunko], [false], [original slewk],
  [snex], [snaː], [snaː], [true], [no comment],
  [sneyg_w_h], [sneːb], [sniɡʷ], [false], [no comment],
  [srewm], [srowm], [srowman], [false], [no comment],
  [stomn], [stomn], [stamnaː], [false], [no comment],
  [xste:r], [stiːr], [steraː], [false], [no comment],
  [sterk], [sterk], [stronko], [false], [no comment],
  [hsu], [su], [su], [true], [no comment],
  [swexdu], [swaːdu], [swaːdu], [true], [no comment],
  [tenh], [ten], [torano], [false], [metathesis],
  [treyes], [treːes], [triːs], [false], [no comment],
  [tuh], [tu], [tu], [true], [no comment],
  [uper], [ufer], [ufor], [false], [no comment],
  [wid_hu], [widu], [widu], [true], [no comment],
  [xwehnto], [winto], [winto], [true], [no comment],
  [yemho], [yemo], [yemono], [false], [no comment],
  [wirho], [wipretonicro], [wiro], [false], [],
)

#bibliography(("bib.bib", "bib.yaml"))








