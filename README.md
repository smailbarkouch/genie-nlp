# genie-nlp

A question answering program that uses wikipedia and natural language processing. Genie works best with research type questions (ie stuff you would look for on wikipedia).

### Accuracy

Genie isn't accurate right now. Some examples include:

```
Question: Are oranges or apple better?
Answer: Oranges & Lemons is the eleventh studio album and the second double album by the English band XTC, released 27 February 1989 on Virgin Records...
```

```
Question: What is the meaning of life?
Answer: The Meaning of Life is an Irish television programme, broadcast on RTÉ One. Presented by Gay Byrne, each edition involves the veteran broadcaster interviewing a prominent public figure...
```

There is a limit on how inaccurate answers can be. This limit is applied towards the question answering model's score, which is out of `1.0`. An answer has to have a score `> 0.5` to still be usuable.

### TODO
- [x] Use `rust bert` and `wikipedia` to compile answers
- [x] Make Genie give somewhat relevant answers
- [x] Make Genie a command
- [ ] Answers can be portions of a summary
- [ ] Use other methods to verify an answer's validity
- [ ] Make it robust and friendly
- [ ] Deploy it to a package manager (like Brew)

### Technologies 

Written in rust.
Libraries incoperated include
- rust-bert: transformer-based models implemented in pure rust
- wikipedia: a way to crawl and scrape wikipedia articles

The only transformer based model being used (for now) is `RoBERTa`, and it is used to determine if a given answer makes sense relative to the question (on a shallow level).
