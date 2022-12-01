
# jean

jean is a computational biology library written in Rust. It offers
computational biology primitives such as sequences (DNA, RNA, and Protein), bases, amino acids, and codons. Optional features extend the 
abilities of jean, such as `io` which allows for reading/writing from/to common biological file formats (e.g. FASTA, GFF3) and `alignment` which allows for aligning sequences. 

## Getting Started

Add `jean` and any needed features to your Rust project:

```sh
$ cargo add jean # --features io,...
```

## Feature Matrix

<table>
  <tbody>
    <tr>
      <th>Feature</th>
      <th>Use</th>
      <th>Links</th>
    </tr>
    <tr>
      <td><code>io</code></td>
      <td>I/O of common biological file formats (e.g. FASTA, GFF3)</td>
      <td>
        (<a href="">Documentation</a>)
      </td>
    </tr>
    <tr>
      <td><code>macros</code></td>
      <td>Macros for creating sequences</td>
      <td>
        (<a href="">Documentation</a>)
      </td>
    </tr>
    <tr>
      <td><code>alignment</code></td>
      <td>Sequence alignment tools</td>
      <td>
        (<a href="">Documentation</a>)
      </td>
    </tr>
    <tr>
      <td><code>blosum</code></td>
      <td>BLOSUM substitution matrices</td>
      <td>
        (<a href="">Documentation</a>)
      </td>
    </tr>
  </tbody>
</table>

## Examples

### DNA Sequence Alignment
```rust
#[macro_use]
extern crate jean;

use jean::{alignment::global::NeedlemanWunsch, blosum::NUC_4_2, dna::Dna};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let seq1: Dna = dna!("AGACTAGTTAC");
  let seq2: Dna = dna!("CGAGACGT");

  let alignment = NeedlemanWunsch::new()
    .matrix(&NUC_4_2)
    .gap_penalty(|k| -2 * k)
    .align(&seq1, &seq2)?;

  assert_eq!(alignment.score, 16);
  assert_eq!(alignment.a, dna!("--AGACTAGTTAC"));
  assert_eq!(alignment.b, dna!("CGAGAC--G-T--"));

  Ok(())
}
```

### DNA -> RNA -> Protein
```rust
#[macro_use]
extern crate jean;

use jean::{cut::Cut, dna::Dna, prelude::*, protein::Protein, rna::Rna};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let cut = Cut::read_file("homo_sapien.cut")?;

  /* DNA -> RNA -> Protein */
  let d: Dna = dna!("AGGCTGGGCACC");
  let r: Rna = d.transcribe();
  let p: Protein = r.translate(&cut);

  assert_eq!(p, protein!("RLGT"));

  /* ...and back again */
  let r_ = p.rev_translate(&cut);
  let d_ = r_.rev_transcribe();

  assert_eq!(d, d_);

  Ok(())
}
```
