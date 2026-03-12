# grabber.zone Grabber

Automatically downloads comics from [grabber.zone](grabber.zone) and converts them into `.cbz` files.

## TODO

Change how we get a single issue, to simplify the approach and ensure we keep metadata.
Probably, still the collection url should be provided, but then there might be a flag `--latest` to get only the last issue, or the user might have to provide the index or the exact name match.
Not perfect, but makes my life a whole lot easier.
I expect `--latest` to be enough anyways. Typically, you will download an entire collection once, and then download new issues as they come out.
(Maybe it is possible to have `--latest <N>?` with an optional count, if clap supports that, to get the last N issues, instead of only the last one)
