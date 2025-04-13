## command line
- 실행

`cargo run -- {query} {fileName}`

- env 주입

`$Env:IGNORE_CASE=1; cargo run -- to sample.txt`

- env 삭제

`Remove-Item Env:IGNORE_CASE`