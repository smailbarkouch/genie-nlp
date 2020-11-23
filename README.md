# genie-nlp

Get instant answers using wikipedia and transformer models. Genie works best with research type questions (ie stuff you would look for on wikipedia).

### Accuracy

Genie is somewhat accurate. Some examples include:

```
Question: What is Agar.io?
Answer: Agar.io is a massively multiplayer online action game created by Brazilian developer Matheus Valadares...
```

```
Question: What is the meaning of life?
Answer: The meaning of life, or the answer to the question: "What is the meaning of life?", pertains to the significance of living or existence in general... 
```

### TODO
- [x] Use `rust bert` and `wikipedia` to compile answers
- [x] Make Genie give somewhat relevant answers
- [x] Make Genie a command
- [ ] Use other methods to verify an answer's validity
- [ ] Make it robust and friendly
- [ ] Deploy it to a package manager (like Brew)

### Technologies 

Written in rust.
Libraries incoperated include
- rust-bert: transformer-based models implemented in pure rust
- wikipedia: a way to crawl and scrape wikipedia articles

The only transformer based model being used (for now) is `RoBERTa`, and it is used to determine if a given answer makes sense relative to the question (on a shallow level).
