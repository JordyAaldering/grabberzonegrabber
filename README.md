# grabber.zone Grabber

Automatically downloads comics from [grabber.zone](grabber.zone) and converts them into `.cbz` files.

## TODO

Similarly for extracting issues from a collection.

```html
<div class="page-content-listing single-page">
    <div class="listing-chapters_wrap cols-4 show-more show">
        <ul class="main version-chap no-volumn active">

            <li class="wp-manga-chapter   has-thumb  ">
                <div class="chapter-thumbnail">
                    <a href="https://grabber.zone/comics/sonic-the-hedgehog-modern-era/sonic-the-hedgehog-247/">
                    <img class="thumb" src="https://grabber.zone/wp-content/uploads/thumb-1414-75x106.jpg">
                    </a>
                </div>
                <a href="https://grabber.zone/comics/sonic-the-hedgehog-modern-era/sonic-the-hedgehog-247/">
                </a><a href="https://grabber.zone/comics/sonic-the-hedgehog-modern-era/sonic-the-hedgehog-247/">
                Sonic the Hedgehog 247								</a>
                <span class="chapter-release-date">
                <i>November 24, 2021</i>									</span>
            </li>

            <li class="wp-manga-chapter   has-thumb  ">
                <div class="chapter-thumbnail">
                    <a href="https://grabber.zone/comics/sonic-the-hedgehog-modern-era/sonic-the-hedgehog-246/">
                    <img class="thumb" src="https://grabber.zone/wp-content/uploads/thumb-1413-75x106.jpg">
                    </a>
                </div>
                <a href="https://grabber.zone/comics/sonic-the-hedgehog-modern-era/sonic-the-hedgehog-246/">
                </a><a href="https://grabber.zone/comics/sonic-the-hedgehog-modern-era/sonic-the-hedgehog-246/">
                Sonic the Hedgehog 246								</a>
                <span class="chapter-release-date">
                <i>November 24, 2021</i>									</span>
            </li>

            <li class="wp-manga-chapter   has-thumb  ">
                <div class="chapter-thumbnail">
                    <a href="https://grabber.zone/comics/sonic-the-hedgehog-modern-era/sonic-the-hedgehog-160/">
                    <img class="thumb" src="https://grabber.zone/wp-content/uploads/thumb-1327-75x106.jpg">
                    </a>
                </div>
                <a href="https://grabber.zone/comics/sonic-the-hedgehog-modern-era/sonic-the-hedgehog-160/">
                </a><a href="https://grabber.zone/comics/sonic-the-hedgehog-modern-era/sonic-the-hedgehog-160/">
                Sonic the Hedgehog 160								</a>
                <span class="chapter-release-date">
                <i>November 24, 2021</i>									</span>
            </li>
        </ul>

        <div class="c-chapter-readmore">
            <span class="btn btn-link chapter-readmore less-chap" style="display: inline;">
            Show more 				</span>
        </div>
    </div>
</div>
```

It even has the pretty-printed title, and the release data.
We can use this in the metadata.

## TODO

This should then also change how we get a single issue, to simplify the approach and ensure we keep metadata.
Probably, still the collection url should be provided, but then there might be a flag `--latest` to get only the last issue, or the user might have to provide the index or the exact name match.
Not perfect, but makes my life a whole lot easier.
I expect `--latest` to be enough anyways. Typically, you will download an entire collection once, and then download new issues as they come out.
(Maybe it is possible to have `--latest <N>?` with an optional count, if clap supports that, to get the last N issues, instead of only the last one)
