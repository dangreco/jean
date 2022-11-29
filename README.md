
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
    </tr
    <tr>
      <td><code>alignment</code></td>
      <td>Sequence alignment tools</td>
      <td>
        (<a href="">Documentation</a>)
      </td>
    </tr
  </tbody>
</table>

## Example (DNA Sequence Alignment)

```rust
#[macro_use]
extern crate jean;

use jean::{
  dna::Dna,
  alignment::NeedlemanWunsch,
};

fn main() -> Result<(), Box<dyn std::error:Error>>
{
  let seq1: Dna = dna!("AGACTAGTTAC");
  let seq2: Dna = dna!("CGAGACGT");

  let (score, alignment) = NeedlemanWunsch::new()
    .similarity_matrix(vec![
      vec![10, -1, -3, -4],
      vec![-1,  7, -5, -3],
      vec![-3, -5,  9,  0],
      vec![-4, -3,  0,  8],
    ])?
    .gap_penalty(-5)
    .align(&seq1, &seq2)?;
    
  assert_eq!(score, 1);
  assert_eq!(alignment.0, dna!("AGACTAGTTAC"));
  assert_eq!(alignment.1, dna!("CGA---GACGT"));

  Ok(())
}
```