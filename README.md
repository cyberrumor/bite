# bite
Branching Information Trees Extractor


# How is this different from other summary engines?
This doesn't use deep learning, it just uses math. It works by taking slices of a corpus that look like the following:

Corpus: Mary had a little lamb

Slices: 
- Mary had
- mary had a 
- mary had a little
- mary had a little lamb
- had a
- had a little
- had a little lamb
- a little
- a little lamb
- little lamb

From here, it generates a frequency map and scores each sentence based on the map, dividing each sentence score by the average sentence length. Any sentence that scores over double the average score is included in the output. For those interested, a summary of Alice in Wonderland can be found [here](https://github.com/cyberrumor/bite/blob/main/summary_example.txt). Reddit [reports](https://www.reddit.com/r/LanguageTechnology/comments/nggaag/is_this_approach_to_summary_production_novel/) this is an [n-gram](https://en.wikipedia.org/wiki/N-gram) [Markov](https://en.wikipedia.org/wiki/Markov_model) model.

# installation
```
git clone https://github.com/cyberrumor/bite.git
cd bite
cargo build --release
sudo cp target/release/bite /usr/local/bin
```

# Usage
```bite alice_in_wonderland.txt```

