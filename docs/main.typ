
#import "@preview/ascii-ipa:2.0.0": *

#set page(margin: 1.75in)
#set par(leading: 0.55em, spacing: 0.55em, first-line-indent: 1.8em, justify: true)
#set text(font: "New Computer Modern")
#show raw: set text(font: "New Computer Modern Mono")
#show heading: set block(above: 1.4em, below: 1em)


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
The set $Sigma$ is usually called the _alphabet_ and the members _characters_. By convention, strings are usually put in double quotes and the $dot$ operator ommited. Examples: todo

A formal _language_ can be defined a (potentiall infiite) set of strings. Can be used to define natural language such as English, spansih, but can also be used to define more artifical lanuages, such as all prime numbers represented in decimal. 


A deterministic finite state machine can be defined as  $(Q, s, Sigma, F, delta)$, where 

- $Q$ is the set of states
- $s in Q$ is the start state
- $Sigma$ is the _alphabet_, a set of characters
- $F subset Q$ the set of final states
- $delta: Q times X -> Q$ the transition function that maps each state to its next state when given a character

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
- $delta: Q times X -> P(Q)$, $P$ being the powerset function. The nondeterministic transition function that maps each state and character to possible states.

The only difference here is the transition function, which can now map to multiple or zero states, in additon to just one state.

The non determinstic finite state machine also has a similar accepts function. 

A finite state machine $(Q, s, Sigma, F, delta)$ accepts a string $S$ when one of the following conditions are met:

- $S = epsilon$ and $s in F$ 
- $S = c dot S'$ and there exists a $q$ $delta(q, c) = q'$ and $M' = (Q, q', Sigma, F, delta)$ accepts $S'$

A deterministic finite state machine is trivially a non deterministic finite state machine. However, it may be surprising to know the the converse is also true: any 


= Infastructure

The project is made to imitate 

= Case Study Celtic

I used @celtic
  


#bibliography("bib.bib")







