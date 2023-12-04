# `smrty`

Typographic quotes, dashes and ellipsis from plaintext stdin.

```
$ smrty
"Somethings's 'test' example." Test---example 140--141...for example.
:w

“Somethings’s ‘test’ example.” Test—example 140–141…for example.
```

Similar to [Pandoc](https://pandoc.org/) `smart` but with no language support. Escaping does not work.

LaTeX-like explicit quotes are supported, which solve the direction issue with contractions:

```
$ smrty -e
``Somethings's `test' example.'' Test---example 140--141...for example, the '90s.
:w

“Somethings’s ‘test’ example.” Test—example 140–141…for example, the ’90s.
```
