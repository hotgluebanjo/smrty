# `smrty`

Typographic quotes, dashes and ellipsis from plaintext stdin.

```
$ smrty
"Somethings's 'test' example." Test---example 140--141...for example.
:w

“Somethings’s ‘test’ example.” Test—example 140–141…for example.
```

Similar to [Pandoc](https://pandoc.org/)'s' `smart` extension but with no language support (meaning it will probably mess up code blocks). Ideal for quick conversion. Escaping only works for implicit quotes.

LaTeX-like explicit quotes are supported, which solve the directional issue with contractions:

```
$ smrty -e
``Somethings's `test' example.'' Test---example 140--141...for example, the '90s.
:w

“Somethings’s ‘test’ example.” Test—example 140–141…for example, the ’90s.
```
